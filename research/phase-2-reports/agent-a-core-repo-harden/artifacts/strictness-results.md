## codex
- command: pnpm -s run format && pnpm -s run lint
  returncode: 0
  status: PASS
  output:
    Checking formatting...
    All matched files use Prettier code style!

## opencode
- command: go test ./...
  returncode: 1
  status: WARN
  output:
    ?   	github.com/opencode-ai/opencode	[no test files]
    ?   	github.com/opencode-ai/opencode/cmd	[no test files]
    ?   	github.com/opencode-ai/opencode/cmd/schema	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/app	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/completions	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/config	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/db	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/diff	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/fileutil	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/format	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/history	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/llm/agent	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/llm/models	[no test files]
    ok  	github.com/opencode-ai/opencode/internal/llm/prompt	2.037s
    ?   	github.com/opencode-ai/opencode/internal/llm/provider	[no test files]
    --- FAIL: TestLsTool_Run (0.02s)
        --- FAIL: TestLsTool_Run/handles_empty_path_parameter (0.00s)
    panic: config not loaded [recovered]
    	panic: config not loaded
    
    goroutine 26 [running]:
    testing.tRunner.func1.2({0x100821660, 0x1008c1d00})
    	/Users/kooshapari/go/pkg/mod/golang.org/toolchain@v0.0.1-go1.24.0.darwin-arm64/src/testing/testing.go:1734 +0x1ac
    testing.tRunner.func1()
    	/Users/kooshapari/go/pkg/mod/golang.org/toolchain@v0.0.1-go1.24.0.darwin-arm64/src/testing/testing.go:1737 +0x334
    panic({0x100821660?, 0x1008c1d00?})
    	/Users/kooshapari/go/pkg/mod/golang.org/toolchain@v0.0.1-go1.24.0.darwin-arm64/src/runtime/panic.go:787 +0x124
    github.com/opencode-ai/opencode/internal/config.WorkingDirectory(...)
    	/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode/internal/config/config.go:874
    github.com/opencode-ai/opencode/internal/llm/tools.(*lsTool).Run(0x1008c6150?, {0x1400002f0a0?, 0x19?}, {{0x0, 0x0}, {0x1003d48fc, 0x2}, {0x140001a7e90, 0x19}})
    	/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode/internal/llm/tools/ls.go:99 +0x504
    github.com/opencode-ai/opencode/internal/llm/tools.TestLsTool_Run.func3(0x14000485180)
    	/Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/opencode/internal/llm/tools/ls_test.go:139 +0xe8
    testing.tRunner(0x14000485180, 0x1008c0690)
    	/Users/kooshapari/go/pkg/mod/golang.org/toolchain@v0.0.1-go1.24.0.darwin-arm64/src/testing/testing.go:1792 +0xe4
    created by testing.(*T).Run in goroutine 23
    	/Users/kooshapari/go/pkg/mod/golang.org/toolchain@v0.0.1-go1.24.0.darwin-arm64/src/testing/testing.go:1851 +0x374
    FAIL	github.com/opencode-ai/opencode/internal/llm/tools	3.065s
    ?   	github.com/opencode-ai/opencode/internal/llm/tools/shell	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/logging	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/lsp	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/lsp/protocol	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/lsp/util	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/lsp/watcher	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/message	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/permission	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/pubsub	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/session	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/tui	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/tui/components/chat	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/tui/components/core	[no test files]
    ok  	github.com/opencode-ai/opencode/internal/tui/components/dialog	2.552s
    ?   	github.com/opencode-ai/opencode/internal/tui/components/logs	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/tui/components/util	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/tui/image	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/tui/layout	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/tui/page	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/tui/styles	[no test files]
    ok  	github.com/opencode-ai/opencode/internal/tui/theme	3.539s
    ?   	github.com/opencode-ai/opencode/internal/tui/util	[no test files]
    ?   	github.com/opencode-ai/opencode/internal/version	[no test files]
    FAIL

## cliproxyapi-plusplus
- command: go test ./...
  returncode: 137
  status: WARN
  output:
    # github.com/kooshapari/CLIProxyAPI/v7/internal/runtime/executor
    internal/runtime/executor/copilot_cli_executor_test.go:346:1: imports must appear before other declarations
    FAIL	github.com/kooshapari/CLIProxyAPI/v7/internal/runtime/executor [setup failed]

## goose
- command: cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test --workspace
  returncode: 137
  status: WARN
  output:
        Updating crates.io index
        Blocking waiting for file lock on package cache
        Blocking waiting for file lock on package cache
     Downloading crates ...
      Downloaded aws-smithy-query v0.60.9
      Downloaded base64-simd v0.8.0
      Downloaded biome_diagnostics_macros v0.3.1
      Downloaded better_scoped_tls v1.0.1
      Downloaded ast_node v5.0.0
      Downloaded number_prefix v0.4.0
      Downloaded num-derive v0.4.2
      Downloaded countme v3.0.1
      Downloaded core_maths v0.1.1
      Downloaded core-graphics-types v0.1.3
      Downloaded pbkdf2 v0.11.0
      Downloaded primal-check v0.3.4
      Downloaded aws-smithy-xml v0.60.14
      Downloaded aws-smithy-json v0.61.9
      Downloaded proc-macro-error-attr v1.0.4
      Downloaded deno_package_json v0.29.0
      Downloaded pulp-wasm-simd-flag v0.1.0
      Downloaded bzip2 v0.4.4
      Downloaded biome_js_factory v0.3.1
      Downloaded biome_diagnostics v0.3.1
      Downloaded deno_path_util v0.6.4
      Downloaded serial_test_derive v3.3.1
      Downloaded seq-macro v0.3.6
      Downloaded console v0.15.11
      Downloaded find_cuda_helper v0.2.0
      Downloaded nanoid v0.4.0
      Downloaded fixedbitset v0.5.7
      Downloaded etcetera v0.11.0
      Downloaded pctx_deno_transpiler v0.1.1
      Downloaded cfb v0.10.0
      Downloaded enumflags2 v0.7.12
      Downloaded ecow v0.2.6
      Downloaded swc_allocator v4.0.1
      Downloaded objc v0.2.7
      Downloaded num-modular v0.6.1
      Downloaded deno_ast v0.52.0
      Downloaded biome_json_parser v0.3.1
      Downloaded typeid v1.0.3
      Downloaded yoke-derive v0.7.5
      Downloaded zip v8.1.0
      Downloaded swc_ecma_codegen_macros v2.0.2
      Downloaded swc_ecma_codegen v20.0.2
      Downloaded text_lines v0.6.0
      Downloaded swc_ecma_ast v18.0.0
      Downloaded swc_config_macro v1.0.1
      Downloaded swc_config v3.1.2
      Downloaded futures-lite v2.6.1
      Downloaded futures-concurrency v7.7.1
      Downloaded thousands v0.2.0
      Downloaded syn-match v0.3.0
      Downloaded sys_traits_macros v0.1.0
      Downloaded xterm-color v1.0.2
      Downloaded sourcemap v9.3.2
      Downloaded rangemap v1.7.1
      Downloaded malloc_buf v0.0.6
      Downloaded lebe v0.5.3
      Downloaded lazy-regex-proc_macros v3.6.0
      Downloaded sys_traits v0.1.17
      Downloaded unbinder v0.1.7
      Downloaded sys-info v0.9.1
      Downloaded memo-map v0.3.3
      Downloaded utoipa v5.4.0
      Downloaded temporal_rs v0.1.2

## kilocode
- command: pnpm lint && pnpm test
  returncode: 143
  status: WARN
  output:
     WARN  Unsupported engine: wanted: {"node":"20.19.2"} (current: {"node":"v25.6.1","pnpm":"10.8.1"})
    
    > kilo-code@ lint /Users/kooshapari/temp-PRODVERCEL/485/heliosHarness/clones/kilocode
    > turbo lint --log-order grouped --output-logs new-only
    
    turbo 2.6.1
    
    • Packages in scope: @kilo-code/jetbrains-host, @kilo-code/jetbrains-plugin, @kilocode/cli, @roo-code/build, @roo-code/cloud, @roo-code/config-eslint, @roo-code/config-typescript, @roo-code/evals, @roo-code/ipc, @roo-code/playwright-e2e, @roo-code/storybook, @roo-code/telemetry, @roo-code/types, @roo-code/vscode-e2e, @roo-code/vscode-nightly, @roo-code/vscode-webview, @roo-code/web-evals, @roo-code/web-roo-code, kilo-code, kilocode-docs
    • Running lint in 20 packages
    • Remote caching disabled

