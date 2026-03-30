# User Journeys — heliosCLI

**Document:** User Journeys for heliosCLI
**Version:** 1.0
**Date:** 2026-03-29
**Status:** Complete

---

## Overview

User journeys capture end-to-end workflows that users perform with heliosCLI. Each journey maps to one or more Functional Requirements (FR-*) and represents a realistic use case.

---

## UJ-001: Interactive AI Conversation Session

**Actor:** Software Developer
**Goal:** Start an interactive conversation with an AI agent to get coding help
**Preconditions:** heliosCLI installed, user authenticated with API key

### Flow

```
Developer $ helios
┌──────────────────────────────────────┐
│   helios TUI (helios-tui launch)     │  [FR-CLI-002: Default Interactive TUI]
│                                      │
│   > "How do I handle errors in Rust?"│  [User types query]
│                                      │
│   [AI Agent responds with code       │
│    snippet and explanation]          │
│                                      │
│   > "Apply this to my project"       │  [User requests action]
│                                      │
│   [Patch file generated and applied] │  [FR-CLI-006: Apply Patch]
│                                      │
│   > "Exit" or Ctrl+C                 │
└──────────────────────────────────────┘

Result: Conversation history saved to $HELIOS_HOME/sessions/<session-id>/
        Patch applied to working directory
```

### Involved FRs
- FR-CLI-002: Default Interactive TUI Launch
- FR-CLI-006: Apply Patch
- FR-AUTH-001 or FR-AUTH-002: Authentication

---

## UJ-002: Batch Processing with Piped Input

**Actor:** DevOps Engineer
**Goal:** Process code snippets in a batch script without interactive prompts
**Preconditions:** heliosCLI in PATH, piped input available

### Flow

```
DevOps $ cat code.rs | helios --model claude-opus --provider anthropic
┌────────────────────────────────────────────┐
│ Non-Interactive Mode (stdin is not TTY)    │  [FR-CLI-003: Batch Mode]
│                                            │
│ Input: Rust code snippet                   │
│ Process: Analyze & suggest improvements    │
│ Output: JSON result to stdout              │
│ Exit Code: 0 (success) or non-zero (error) │
└────────────────────────────────────────────┘

DevOps $ cat code.rs | helios | jq '.suggestions'
[
  "Add error handling with Result<T>",
  "Use pattern matching instead of match statement",
  ...
]
```

### Involved FRs
- FR-CLI-001: Multi-Backend Agent Dispatch (`--model`, `--provider`)
- FR-CLI-003: Batch Non-Interactive Mode
- FR-AUTH-001: API Key stored from prior login

---

## UJ-003: Session Resume & Continuation

**Actor:** Research Scientist
**Goal:** Resume a prior conversation session after closing the terminal
**Preconditions:** At least one prior session exists in `$HELIOS_HOME/sessions/`

### Flow

```
Scientist $ helios session list
[1] Session ID: 7f9e2a1c-4d3b-11ee-be56-0242ac130001
    Created: 2026-03-29 10:15 UTC
    Messages: 24
    Tags: machine-learning, pytorch

Scientist $ helios resume 7f9e2a1c-4d3b-11ee-be56-0242ac130001
┌──────────────────────────────────────┐
│ TUI loads prior session state         │  [FR-CLI-004: Session Resume]
│ Conversation history visible above    │
│                                      │
│ > "Continue from where we left off"  │
│                                      │
│ [AI responds with context preserved] │
│                                      │
│ > "Fork this into a new session"     │  [FR-CLI-005: Session Fork]
└──────────────────────────────────────┘

Result: New session created with history up to fork point
        Original session unchanged
        New session ID returned: 8a1f3b2d-4e4c-11ee-be56-0242ac130002
```

### Involved FRs
- FR-CLI-004: Session Resume
- FR-CLI-005: Session Fork (variant)

---

## UJ-004: Sandboxed Code Execution

**Actor:** Security Researcher
**Goal:** Execute untrusted code safely in a sandboxed environment
**Preconditions:** heliosCLI installed, sandbox policy configured

### Flow

```
Researcher $ helios sandbox macos --workspace /tmp/test-env
┌────────────────────────────────────────┐
│ macOS Seatbelt Sandbox                │  [FR-SBX-001: macOS Sandbox]
│                                       │
│ Profile: Restrict writes to /tmp/test-env │  [FR-SBX-004: Write Denial]
│ Network: Allow anthropic.com, block * │  [FR-SBX-005: Network Policy]
│ Timeout: 30s (from config)            │  [FR-SBX-006: Timeout]
│                                       │
│ Command: python malicious.py          │
│ Result: DENIED write to /etc/passwd   │
│         Error: "Write denied to       │
│         /etc/passwd (outside          │
│         workspace /tmp/test-env)"     │
│                                       │
│ Exit Code: 1 (sandboxed execution)    │
└────────────────────────────────────────┘

Researcher $ helios execpolicy check
✓ Network policy allows: anthropic.com
✓ Workspace isolation: /tmp/test-env
✓ Timeout: 30s
✓ All assertions pass
```

### Involved FRs
- FR-SBX-001: macOS Seatbelt Sandbox
- FR-SBX-004: Write-Outside-Workspace Denial
- FR-SBX-005: Configurable Network Policy
- FR-SBX-006: Execution Timeout
- FR-SBX-007: Execution Policy Check

---

## UJ-005: Shell Completion Setup

**Actor:** DevOps Engineer
**Goal:** Enable shell completion for helios commands
**Preconditions:** heliosCLI installed

### Flow

```
Engineer $ helios completion zsh > ~/.zfunc/_helios
┌──────────────────────────────────────┐
│ Shell Completion Script Generation   │  [FR-CLI-007: Shell Completion]
│                                      │
│ Supported shells:                    │
│ - bash      → .bashrc integration    │
│ - zsh       → .zshrc integration     │
│ - fish      → .config/fish/conf.d    │
│ - powershell → $PROFILE              │
│                                      │
│ Output: Complete script with all     │
│         subcommands, flags, options  │
└──────────────────────────────────────┘

Engineer $ source ~/.zfunc/_helios
Engineer $ helios <TAB><TAB>
apply      completion  fork      login     logout    mcp       resume    sandbox   session   app-server

Engineer $ helios sandbox <TAB><TAB>
linux      macos       windows
```

### Involved FRs
- FR-CLI-007: Shell Completion

---

## UJ-006: Background App Server with IPC

**Actor:** Integration Developer
**Goal:** Run helios as a background service and communicate via IPC
**Preconditions:** heliosCLI installed

### Flow

```
Developer $ helios app-server start
┌────────────────────────────────────────┐
│ App Server Lifecycle Management       │  [FR-SRV-001: Start]
│                                       │
│ Starting: helios-app-server process   │
│ Socket: /tmp/helios-app.sock          │
│ Status: Running                        │
└────────────────────────────────────────┘

Developer $ helios app-server status
Status: Running
PID: 12345
Socket: /tmp/helios-app.sock
Uptime: 5m 23s

Developer $ helios stdio-to-uds /tmp/helios-app.sock
┌────────────────────────────────────────┐
│ stdio → UDS Bridge                     │  [FR-SRV-003: stdio-to-UDS Bridge]
│                                       │
│ Bridge stdin/stdout to Unix socket    │
│                                       │
│ > {"method": "chat", "msg": "..."}   │  [FR-SRV-002: Typed Protocol]
│ < {"result": "...", "status": "ok"}   │
│                                       │
│ Message Format: JSON (serde)          │
└────────────────────────────────────────┘

Developer $ helios app-server stop
Status: Stopped
```

### Involved FRs
- FR-SRV-001: App Server Lifecycle
- FR-SRV-002: Typed Protocol Messages
- FR-SRV-003: stdio-to-UDS Bridge

---

## UJ-007: MCP Server Integration

**Actor:** LLM Application Developer
**Goal:** Access helios tools from an external AI client via MCP protocol
**Preconditions:** heliosCLI installed, MCP client configured

### Flow

```
LLM Developer $ helios mcp --port 9000
┌────────────────────────────────────────┐
│ MCP Server Launched                    │  [FR-SRV-004: MCP Server Integration]
│                                       │
│ Port: 9000                             │
│ Tools exposed:                         │
│ - shell-tool (execute shell commands) │  [FR-SRV-005: Shell Tool MCP Package]
│ - apply-patch (patch file application)│
│ - query-knowledge (semantic search)   │
│                                       │
│ Ready for external clients             │
└────────────────────────────────────────┘

External LLM Client:
┌──────────────────────────────────┐
│ MCP Client connects to :9000      │
│                                  │
│ Discovers available tools        │
│ - shell-tool (schema validated)  │
│                                  │
│ Calls: {"jsonrpc": "2.0",        │
│         "method": "shell-tool",  │
│         "params": {"cmd": "..."}}│
│                                  │
│ Response: JSON with result       │
└──────────────────────────────────┘
```

### Involved FRs
- FR-SRV-004: MCP Server Integration
- FR-SRV-005: Shell Tool MCP Package

---

## UJ-008: Multi-Backend Agent Dispatch

**Actor:** Platform Engineer
**Goal:** Route AI requests to different backends based on model requirements
**Preconditions:** heliosCLI installed, multiple AI backends configured

### Flow

```
Engineer $ helios query --model claude-opus --provider anthropic \
                        "Explain quantum computing"
┌──────────────────────────────────────────┐
│ Multi-Backend Dispatch                   │  [FR-CLI-001: Agent Dispatch]
│                                         │
│ Parsed flags:                            │
│ - model: claude-opus                     │
│ - provider: anthropic                    │
│                                         │
│ Alias expansion:                         │
│ - oss_provider → open-source backend     │  [Alias expansion]
│ - local_provider → local inference       │
│                                         │
│ Route: Anthropic API (claude-opus)       │
│ Response: [Multi-turn conversation]      │
└──────────────────────────────────────────┘

Engineer $ helios query --model llama2-70b --provider oss_provider \
                        "Summarize this paper"
Route: Local OSS backend (Llama 2 70B)
Response: [Local inference result]
```

### Involved FRs
- FR-CLI-001: Multi-Backend Agent Dispatch

---

## UJ-009: Authentication Flow (API Key)

**Actor:** Enterprise User
**Goal:** Authenticate helios with API key for automated workflows
**Preconditions:** heliosCLI installed, API key obtained from provider

### Flow

```
Enterprise $ export HELIOS_API_KEY="sk-..."
Enterprise $ helios login --api-key < secret.txt
┌───────────────────────────────────┐
│ API Key Authentication            │  [FR-AUTH-001: API Key Login]
│                                  │
│ Input: API key from stdin         │
│ Storage: System keyring via       │  [helios-keyring-store]
│ Security: Never logged/exposed    │
│                                  │
│ Status: ✓ Authenticated           │
│ User: enterprise@company.com      │
│ Provider: anthropic               │
└───────────────────────────────────┘

Enterprise $ helios query "Extract info from report" < report.pdf
[Response uses stored API key from keyring]
```

### Involved FRs
- FR-AUTH-001: API Key Login

---

## UJ-010: OAuth Device-Code Flow

**Actor:** Individual Developer
**Goal:** Authenticate helios via device-code OAuth (user-friendly)
**Preconditions:** heliosCLI installed, browser available

### Flow

```
Developer $ helios login
┌──────────────────────────────────────────┐
│ Device-Code OAuth Flow                   │  [FR-AUTH-002: Device OAuth]
│                                         │
│ Go to: https://device.anthropic.com     │
│ Code: ABCD-1234                         │
│                                         │
│ [Developer opens browser, logs in,      │
│  approves helios access]                │
│                                         │
│ Polling for approval...                 │
│ ✓ Approved!                             │
│                                         │
│ Token stored in system keyring          │
│ User: developer@example.com             │
│ Scopes: read:sessions, write:messages   │
└──────────────────────────────────────────┘

Developer $ helios session list
[Sessions loaded using OAuth token]
```

### Involved FRs
- FR-AUTH-002: Device-Code OAuth

---

## Journey Metrics

| Journey | Actor | Frequency | Priority | Status |
|---------|-------|-----------|----------|--------|
| UJ-001 | Developer | Daily | P0 | ✅ Core |
| UJ-002 | DevOps Engineer | Weekly | P1 | ✅ Core |
| UJ-003 | Researcher | Weekly | P2 | ✅ Enhancement |
| UJ-004 | Security Researcher | Monthly | P1 | ✅ Critical |
| UJ-005 | DevOps Engineer | One-time | P3 | ✅ Nice-to-have |
| UJ-006 | Integration Dev | Weekly | P2 | ✅ Advanced |
| UJ-007 | LLM App Dev | Monthly | P2 | ✅ Advanced |
| UJ-008 | Platform Engineer | Daily | P1 | ✅ Core |
| UJ-009 | Enterprise User | Daily | P0 | ✅ Core |
| UJ-010 | Indie Developer | One-time | P1 | ✅ Core |

---

## Traceability Matrix

### By Functional Requirement

| FR | UJs | Coverage |
|----|----|----------|
| FR-CLI-001 | UJ-002, UJ-008 | ✅ |
| FR-CLI-002 | UJ-001 | ✅ |
| FR-CLI-003 | UJ-002 | ✅ |
| FR-CLI-004 | UJ-003 | ✅ |
| FR-CLI-005 | UJ-003 | ✅ |
| FR-CLI-006 | UJ-001 | ✅ |
| FR-CLI-007 | UJ-005 | ✅ |
| FR-SBX-001 | UJ-004 | ✅ |
| FR-SBX-004 | UJ-004 | ✅ |
| FR-SBX-005 | UJ-004 | ✅ |
| FR-SBX-006 | UJ-004 | ✅ |
| FR-SBX-007 | UJ-004 | ✅ |
| FR-SRV-001 | UJ-006 | ✅ |
| FR-SRV-002 | UJ-006 | ✅ |
| FR-SRV-003 | UJ-006 | ✅ |
| FR-SRV-004 | UJ-007 | ✅ |
| FR-SRV-005 | UJ-007 | ✅ |
| FR-AUTH-001 | UJ-009 | ✅ |
| FR-AUTH-002 | UJ-010 | ✅ |

**Coverage:** 100% (all FRs traced to at least one UJ)

---

## Sign-Off

- **Document Owner:** heliosCLI Team
- **Last Updated:** 2026-03-29
- **Next Review:** 2026-04-15
- **Approval:** Complete & Ready for Refinement
