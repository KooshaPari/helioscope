use crate::error::ApiError;
use codex_protocol::config_types::ReasoningSummary as ReasoningSummaryConfig;
use codex_protocol::config_types::Verbosity as VerbosityConfig;
use codex_protocol::models::ResponseItem;
use codex_protocol::openai_models::ReasoningEffort as ReasoningEffortConfig;
use codex_protocol::protocol::RateLimitSnapshot;
use codex_protocol::protocol::TokenUsage;
use futures::Stream;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use tokio::sync::mpsc;

/// Canonical input payload for the compaction endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct CompactionInput<'a> {
    pub model: &'a str,
    pub input: &'a [ResponseItem],
    pub instructions: &'a str,
}

/// Canonical input payload for the memory summarize endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct MemorySummarizeInput {
    pub model: String,
    #[serde(rename = "traces")]
    pub raw_memories: Vec<RawMemory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<Reasoning>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RawMemory {
    pub id: String,
    pub metadata: RawMemoryMetadata,
    pub items: Vec<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RawMemoryMetadata {
    pub source_path: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemorySummarizeOutput {
    #[serde(rename = "trace_summary", alias = "raw_memory")]
    pub raw_memory: String,
    pub memory_summary: String,
}

#[derive(Debug)]
pub enum ResponseEvent {
    Created,
    OutputItemDone(ResponseItem),
    OutputItemAdded(ResponseItem),
    /// Emitted when the server includes `OpenAI-Model` on the stream response.
    /// This can differ from the requested model when backend safety routing applies.
    ServerModel(String),
    /// Emitted when `X-Reasoning-Included: true` is present on the response,
    /// meaning the server already accounted for past reasoning tokens and the
    /// client should not re-estimate them.
    ServerReasoningIncluded(bool),
    Completed {
        response_id: String,
        token_usage: Option<TokenUsage>,
        /// Whether the client can append more items to a long-running websocket response.
        can_append: bool,
    },
    OutputTextDelta(String),
    ReasoningSummaryDelta {
        delta: String,
        summary_index: i64,
    },
    ReasoningContentDelta {
        delta: String,
        content_index: i64,
    },
    ReasoningSummaryPartAdded {
        summary_index: i64,
    },
    RateLimits(RateLimitSnapshot),
    ModelsEtag(String),
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Reasoning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffortConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummaryConfig>,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TextFormatType {
    #[default]
    JsonSchema,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
pub struct TextFormat {
    /// Format type used by the OpenAI text controls.
    pub r#type: TextFormatType,
    /// When true, the server is expected to strictly validate responses.
    pub strict: bool,
    /// JSON schema for the desired output.
    pub schema: Value,
    /// Friendly name for the format, used in telemetry/debugging.
    pub name: String,
}

/// Controls the `text` field for the Responses API, combining verbosity and
/// optional JSON schema output formatting.
#[derive(Debug, Serialize, Default, Clone, PartialEq)]
pub struct TextControls {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<OpenAiVerbosity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormat>,
}

#[derive(Debug, Serialize, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OpenAiVerbosity {
    Low,
    #[default]
    Medium,
    High,
}

impl From<VerbosityConfig> for OpenAiVerbosity {
    fn from(v: VerbosityConfig) -> Self {
        match v {
            VerbosityConfig::Low => OpenAiVerbosity::Low,
            VerbosityConfig::Medium => OpenAiVerbosity::Medium,
            VerbosityConfig::High => OpenAiVerbosity::High,
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ResponsesApiRequest {
    pub model: String,
    pub instructions: String,
    pub input: Vec<ResponseItem>,
    pub tools: Vec<serde_json::Value>,
    pub tool_choice: String,
    pub parallel_tool_calls: bool,
    pub reasoning: Option<Reasoning>,
    pub store: bool,
    pub stream: bool,
    pub include: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextControls>,
}

impl From<&ResponsesApiRequest> for ResponseCreateWsRequest {
    fn from(request: &ResponsesApiRequest) -> Self {
        Self {
            model: request.model.clone(),
            instructions: request.instructions.clone(),
            previous_response_id: None,
            input: request.input.clone(),
            tools: request.tools.clone(),
            tool_choice: request.tool_choice.clone(),
            parallel_tool_calls: request.parallel_tool_calls,
            reasoning: request.reasoning.clone(),
            store: request.store,
            stream: request.stream,
            include: request.include.clone(),
            prompt_cache_key: request.prompt_cache_key.clone(),
            text: request.text.clone(),
            generate: None,
            client_metadata: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ResponseCreateWsRequest {
    pub model: String,
    pub instructions: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    pub input: Vec<ResponseItem>,
    pub tools: Vec<Value>,
    pub tool_choice: String,
    pub parallel_tool_calls: bool,
    pub reasoning: Option<Reasoning>,
    pub store: bool,
    pub stream: bool,
    pub include: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextControls>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct ResponseAppendWsRequest {
    pub input: Vec<ResponseItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_metadata: Option<HashMap<String, String>>,
}
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum ResponsesWsRequest {
    #[serde(rename = "response.create")]
    ResponseCreate(ResponseCreateWsRequest),
    #[serde(rename = "response.append")]
    ResponseAppend(ResponseAppendWsRequest),
}

pub fn create_text_param_for_request(
    verbosity: Option<VerbosityConfig>,
    output_schema: &Option<Value>,
) -> Result<Option<TextControls>, ApiError> {
    if verbosity.is_none() && output_schema.is_none() {
        return Ok(None);
    }

    if let Some(schema) = output_schema {
        validate_strict_output_schema(schema)?;
    }

    Ok(Some(TextControls {
        verbosity: verbosity.map(std::convert::Into::into),
        format: output_schema.as_ref().map(|schema| TextFormat {
            r#type: TextFormatType::JsonSchema,
            strict: true,
            schema: schema.clone(),
            name: "codex_output_schema".to_string(),
        }),
    }))
}

fn validate_strict_output_schema(schema: &Value) -> Result<(), ApiError> {
    validate_strict_schema(schema, "$")
}

fn validate_strict_schema(schema: &Value, path: &str) -> Result<(), ApiError> {
    let Some(map) = schema.as_object() else {
        return Err(ApiError::InvalidRequest {
            message: format!("{path} must be a JSON object schema"),
        });
    };

    let is_object_schema = map
        .get("type")
        .and_then(Value::as_str)
        .is_some_and(|ty| ty == "object")
        || map.contains_key("properties")
        || map.contains_key("required")
        || map.contains_key("additionalProperties");

    if !is_object_schema {
        if let Some(items) = map.get("items") {
            validate_strict_schema(items, &format!("{path}.items"))?;
        }
        if let Some(items) = map.get("prefixItems").and_then(Value::as_array) {
            for (index, item) in items.iter().enumerate() {
                validate_strict_schema(item, &format!("{path}.prefixItems[{index}]"))?;
            }
        }
        for key in ["oneOf", "anyOf", "allOf"] {
            if let Some(branches) = map.get(key).and_then(Value::as_array) {
                for (index, branch) in branches.iter().enumerate() {
                    validate_strict_schema(branch, &format!("{path}.{key}[{index}]"))?;
                }
            }
        }
        for key in ["not", "if", "then", "else"] {
            if let Some(nested) = map.get(key) {
                validate_strict_schema(nested, &format!("{path}.{key}"))?;
            }
        }
        if let Some(defs) = map.get("$defs").and_then(Value::as_object) {
            for (name, nested) in defs {
                validate_strict_schema(nested, &format!("{path}.$defs.{name}"))?;
            }
        }
        if let Some(defs) = map.get("definitions").and_then(Value::as_object) {
            for (name, nested) in defs {
                validate_strict_schema(nested, &format!("{path}.definitions.{name}"))?;
            }
        }
        return Ok(());
    }

    match map.get("additionalProperties") {
        Some(Value::Bool(false)) => {}
        Some(other) => {
            return Err(ApiError::InvalidRequest {
                message: format!(
                    "{path} must set additionalProperties to false in strict mode, got {other}"
                ),
            });
        }
        None => {
            return Err(ApiError::InvalidRequest {
                message: format!("{path} must set additionalProperties to false in strict mode"),
            });
        }
    }

    let Some(properties) = map.get("properties").and_then(Value::as_object) else {
        return Err(ApiError::InvalidRequest {
            message: format!("{path} must include an object properties map in strict mode"),
        });
    };

    let Some(required) = map.get("required").and_then(Value::as_array) else {
        return Err(ApiError::InvalidRequest {
            message: format!("{path} must include required fields in strict mode"),
        });
    };

    let mut required_names = std::collections::BTreeSet::new();
    for item in required {
        let Some(name) = item.as_str() else {
            return Err(ApiError::InvalidRequest {
                message: format!("{path}.required must contain only strings"),
            });
        };
        required_names.insert(name);
    }

    let property_names = properties
        .keys()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    if required_names != property_names {
        return Err(ApiError::InvalidRequest {
            message: format!("{path} requires every property to appear exactly once in required"),
        });
    }

    for (name, nested) in properties {
        validate_strict_schema(nested, &format!("{path}.properties.{name}"))?;
    }

    if let Some(items) = map.get("items") {
        validate_strict_schema(items, &format!("{path}.items"))?;
    }
    if let Some(items) = map.get("prefixItems").and_then(Value::as_array) {
        for (index, item) in items.iter().enumerate() {
            validate_strict_schema(item, &format!("{path}.prefixItems[{index}]"))?;
        }
    }
    for key in ["oneOf", "anyOf", "allOf"] {
        if let Some(branches) = map.get(key).and_then(Value::as_array) {
            for (index, branch) in branches.iter().enumerate() {
                validate_strict_schema(branch, &format!("{path}.{key}[{index}]"))?;
            }
        }
    }
    for key in ["not", "if", "then", "else"] {
        if let Some(nested) = map.get(key) {
            validate_strict_schema(nested, &format!("{path}.{key}"))?;
        }
    }
    if let Some(defs) = map.get("$defs").and_then(Value::as_object) {
        for (name, nested) in defs {
            validate_strict_schema(nested, &format!("{path}.$defs.{name}"))?;
        }
    }
    if let Some(defs) = map.get("definitions").and_then(Value::as_object) {
        for (name, nested) in defs {
            validate_strict_schema(nested, &format!("{path}.definitions.{name}"))?;
        }
    }

    Ok(())
}

pub struct ResponseStream {
    pub rx_event: mpsc::Receiver<Result<ResponseEvent, ApiError>>,
}

impl Stream for ResponseStream {
    type Item = Result<ResponseEvent, ApiError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx_event.poll_recv(cx)
    }
}
