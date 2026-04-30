use std::ffi::OsString;
use std::net::TcpListener;
use std::path::PathBuf;
use std::time::Duration;

use codex_rmcp_client::ElicitationAction;
use codex_rmcp_client::ElicitationResponse;
use codex_rmcp_client::OAuthCredentialsStoreMode;
use codex_rmcp_client::RmcpClient;
use codex_utils_cargo_bin::CargoBinError;
use futures::FutureExt as _;
use rmcp::model::AnnotateAble;
use rmcp::model::ClientCapabilities;
use rmcp::model::ElicitationCapability;
use rmcp::model::FormElicitationCapability;
use rmcp::model::Implementation;
use rmcp::model::InitializeRequestParams;
use rmcp::model::ListResourceTemplatesResult;
use rmcp::model::ProtocolVersion;
use rmcp::model::ReadResourceRequestParams;
use rmcp::model::ResourceContents;
use serde_json::json;
use tokio::net::TcpStream;
use tokio::process::Child;
use tokio::process::Command;
use tokio::time::Instant;
use tokio::time::sleep;

const RESOURCE_URI: &str = "memo://codex/example-note";
const RESOURCE_HTTP_SERVER_NAME: &str = "rmcp_http";
const RESOURCE_HTTP_ENV_VALUE: &str = "propagated-env-http";
const SANDBOX_NETWORK_DISABLED_ENV_VAR: &str = "CODEX_SANDBOX_NETWORK_DISABLED";

fn stdio_server_bin() -> Result<PathBuf, CargoBinError> {
    codex_utils_cargo_bin::cargo_bin("test_stdio_server")
}

fn streamable_http_server_bin() -> Result<PathBuf, CargoBinError> {
    codex_utils_cargo_bin::cargo_bin("test_streamable_http_server")
}

fn init_params() -> InitializeRequestParams {
    let mut capabilities = ClientCapabilities::default();
    capabilities.elicitation = Some(ElicitationCapability {
        form: Some(FormElicitationCapability {
            schema_validation: None,
        }),
        url: None,
    });

    InitializeRequestParams::new(
        capabilities,
        Implementation::new("codex-test", "0.0.0-test").with_title("Codex rmcp resource test"),
    )
    .with_protocol_version(ProtocolVersion::V_2025_06_18)
}

async fn assert_resource_surface(client: &RmcpClient) -> anyhow::Result<()> {
    let list = client
        .list_resources(None, Some(Duration::from_secs(5)))
        .await?;
    let Some(memo) = list
        .resources
        .iter()
        .find(|resource| resource.uri == RESOURCE_URI)
    else {
        panic!("memo resource present");
    };
    assert_eq!(
        memo,
        &rmcp::model::RawResource {
            uri: RESOURCE_URI.to_string(),
            name: "example-note".to_string(),
            title: Some("Example Note".to_string()),
            description: Some("A sample MCP resource exposed for integration tests.".to_string()),
            mime_type: Some("text/plain".to_string()),
            size: None,
            icons: None,
            meta: None,
        }
        .no_annotation()
    );

    let templates = client
        .list_resource_templates(None, Some(Duration::from_secs(5)))
        .await?;
    assert_eq!(
        templates,
        ListResourceTemplatesResult {
            meta: None,
            next_cursor: None,
            resource_templates: vec![
                rmcp::model::RawResourceTemplate {
                    uri_template: "memo://codex/{slug}".to_string(),
                    name: "codex-memo".to_string(),
                    title: Some("Codex Memo".to_string()),
                    description: Some(
                        "Template for memo://codex/{slug} resources used in tests.".to_string(),
                    ),
                    mime_type: Some("text/plain".to_string()),
                    icons: None,
                }
                .no_annotation()
            ],
        }
    );

    let read = client
        .read_resource(
            ReadResourceRequestParams::new(RESOURCE_URI),
            Some(Duration::from_secs(5)),
        )
        .await?;
    let Some(text) = read.contents.first() else {
        panic!("resource contents present");
    };
    assert_eq!(
        text,
        &ResourceContents::TextResourceContents {
            uri: RESOURCE_URI.to_string(),
            mime_type: Some("text/plain".to_string()),
            text: "This is a sample MCP resource served by the rmcp test server.".to_string(),
            meta: None,
        }
    );

    Ok(())
}

async fn initialize_client(client: &RmcpClient) -> anyhow::Result<()> {
    client
        .initialize(
            init_params(),
            Some(Duration::from_secs(5)),
            Box::new(|_, _| {
                async {
                    Ok(ElicitationResponse {
                        action: ElicitationAction::Accept,
                        content: Some(json!({})),
                    })
                }
                .boxed()
            }),
        )
        .await?;
    Ok(())
}

async fn wait_for_streamable_http_server(address: &str, timeout: Duration) -> anyhow::Result<()> {
    let deadline = Instant::now() + timeout;

    loop {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(anyhow::anyhow!(
                "timed out waiting for streamable HTTP server at {address}: deadline reached"
            ));
        }

        match tokio::time::timeout(remaining, TcpStream::connect(address)).await {
            Ok(Ok(_)) => return Ok(()),
            Ok(Err(error)) => {
                if Instant::now() >= deadline {
                    return Err(anyhow::anyhow!(
                        "timed out waiting for streamable HTTP server at {address}: {error}"
                    ));
                }
            }
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "timed out waiting for streamable HTTP server at {address}: connect call timed out"
                ));
            }
        }

        sleep(Duration::from_millis(50)).await;
    }
}

async fn spawn_streamable_http_server() -> anyhow::Result<(Child, String, String)> {
    let rmcp_http_server_bin = streamable_http_server_bin()?;
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener);

    let bind_addr = format!("127.0.0.1:{port}");
    let server_url = format!("http://{bind_addr}/mcp");
    let child = Command::new(&rmcp_http_server_bin)
        .kill_on_drop(true)
        .env("MCP_STREAMABLE_HTTP_BIND_ADDR", &bind_addr)
        .env("MCP_TEST_VALUE", RESOURCE_HTTP_ENV_VALUE)
        .spawn()?;

    wait_for_streamable_http_server(&bind_addr, Duration::from_secs(5)).await?;
    Ok((child, bind_addr, server_url))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn rmcp_client_can_list_and_read_resources() -> anyhow::Result<()> {
    let client = RmcpClient::new_stdio_client(
        stdio_server_bin()?.into(),
        Vec::<OsString>::new(),
        None,
        &[],
        None,
    )
    .await?;

    initialize_client(&client).await?;
    assert_resource_surface(&client).await?;

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn rmcp_client_can_list_and_read_resources_over_streamable_http() -> anyhow::Result<()> {
    if std::env::var(SANDBOX_NETWORK_DISABLED_ENV_VAR).is_ok() {
        eprintln!(
            "Skipping streamable HTTP resource test because network is disabled in this sandbox."
        );
        return Ok(());
    }

    let (_server_child, _bind_addr, server_url) = spawn_streamable_http_server().await?;
    let client = RmcpClient::new_streamable_http_client(
        RESOURCE_HTTP_SERVER_NAME,
        &server_url,
        None,
        None,
        None,
        OAuthCredentialsStoreMode::Auto,
    )
    .await?;

    initialize_client(&client).await?;
    assert_resource_surface(&client).await?;

    Ok(())
}
