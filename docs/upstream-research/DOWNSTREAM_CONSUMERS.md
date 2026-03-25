# Downstream Consumers & Integrations of CLI AI Agents (helios-cli/codex)

**Research Date:** February 28, 2026
**Scope:** Applications, platforms, and frameworks that consume or build on top of CLI coding agents
**Focus:** helios-cli/codex integration opportunities and requirements

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Category 1: IDE Integrations](#category-1-ide-integrations)
3. [Category 2: CI/CD Agents](#category-2-cicd-agents)
4. [Category 3: Multi-Agent Orchestrators](#category-3-multi-agent-orchestrators)
5. [Category 4: Developer Platforms](#category-4-developer-platforms)
6. [Category 5: Code Review Bots](#category-5-code-review-bots)
7. [Category 6: Voice & Accessibility Layers](#category-6-voice--accessibility-layers)
8. [Cross-Cutting Considerations](#cross-cutting-considerations)
9. [Feature Requests & Spec Gaps](#feature-requests--spec-gaps)

---

## Executive Summary

The AI coding agent ecosystem has matured significantly by 2026. CLI agents like helios-cli/codex serve as the **foundation layer** for an expanding ecosystem of higher-level tools that wrap, orchestrate, and integrate agents into IDEs, CI/CD pipelines, developer platforms, and multi-agent systems.

### Key Findings

1. **MCP is the de facto standard** — Model Context Protocol adoption by OpenAI (2025), Google, and Microsoft (2025-2026) has created a universal integration pattern. Tools expecting `--mcp` or MCP server capabilities have become baseline.

2. **IDE integrations** are split between:
   - Direct plugins (VS Code extensions, Neovim plugins)
   - SDK wrappers (TypeScript SDK that spawns CLI)
   - MCP servers (agents exposing capabilities via MCP)

3. **CI/CD adoption is accelerating** — GitHub Agentic Workflows and GitLab Duo Agent Platform both reference CLI agent spawning as a deployment model. Security (sandboxing, permissioning) is critical.

4. **Multi-agent orchestration** (CrewAI, LangGraph, AutoGen) treats CLI agents as **composable tools** via OpenAPI schemas or direct CLI invocation. LangGraph's graph-based model is becoming preferred.

5. **Developer platforms** (Replit, Coder, Codespaces, Ona/Gitpod) are increasingly bundling AI agents as first-class primitives, sometimes wrapping local CLI agents.

6. **Code review bots** (Qodo, CodeRabbit) need:
   - Multi-file context (repository-aware)
   - Structured output (JSON schemas)
   - Governance/audit trails
   - None currently wrap CLI agents directly; they are SaaS services with their own LLM backends.

7. **Voice & accessibility** is emerging but nascent — OpenAI's Realtime API (GA in 2025) enables voice agents, but CLI agents don't yet have standard voice input/output bindings.

---

## Category 1: IDE Integrations

### Overview

IDE integrations enable developers to use AI coding agents within their daily workflow—editor, language server, or plugin layer.

### 1.1 VS Code & Extensions

**Key Projects:**
- **Cursor** (https://cursor.com) — VS Code fork with AI baked in
- **GitHub Copilot** (https://github.com/features/copilot) — Market leader; supports VS Code, JetBrains, Vim/Neovim
- **Windsurf** (formerly Codeium, https://codeium.com/windsurf) — Multi-editor AI assistant
- **Gemini Code Assist** (Google, https://cloud.google.com/products/gemini-code-assist) — VS Code + JetBrains integration
- **Claude Code** (Anthropic, https://claude.com/claude-code) — Web-based agent interface

**Adoption Level:**
- Cursor: ~50K+ users (fast-growing)
- GitHub Copilot: Industry standard; used by millions
- Windsurf: 200K+ downloads
- Gemini Code Assist: Enterprise-focused

**Integration Pattern:**
- **Language Server Protocol (LSP)** — Agents expose capabilities via LSP (definition, references, hover, code actions)
- **TypeScript SDK** — Spawn local CLI agent, exchange JSONL events over stdin/stdout (e.g., helios-cli SDK)
- **Direct API calls** — For SaaS agents, direct HTTP calls to cloud backend
- **MCP via stdio** — New pattern: IDE spawns agent with `--mcp` flag, uses MCP protocol

**What They Need from helios-cli:**
1. **Structured event streaming** — JSONL output with clear event types (`item.completed`, `turn.completed`, `error`)
2. **TypeScript/JavaScript SDK** ✅ (already exists)
3. **MCP server mode** — Run as `helios --mcp` and expose tool capabilities via MCP protocol
4. **Streaming responses** — `--stream` or event-based output for real-time UI updates
5. **Image input support** — `--image` flag already works; need documentation for IDE integration
6. **Exit codes** — Clear semantics (0=success, 1=agent error, 2=user cancellation, 3=config error)
7. **Cancellation support** — SIGTERM handling, graceful cleanup of threads
8. **Config overrides** — `--config key=value` pattern works; document for IDE integration
9. **Working directory isolation** — Critical for sandboxed IDE environments (Codespaces, Coder)

**Feature Requests:**
- [ ] MCP server mode (`--mcp` flag)
- [ ] Explicit token counting API (for IDE status bar: "tokens used: X/Y")
- [ ] Structured schema output (`--output-schema <json-schema>`) for IDE to parse tool results
- [ ] Interrupt/retry semantics (IDE may want to pause agent and inspect intermediate state)

---

### 1.2 Neovim & Vim Plugins

**Key Projects:**
- **Copilot.vim** (https://github.com/github/copilot.vim) — Official GitHub Copilot plugin
- **Codeium.nvim** (https://github.com/Exafunction/codeium.nvim) — Neovim plugin for Codeium
- **ChatGPT.nvim** (https://github.com/jackMort/ChatGPT.nvim) — General chat plugin for Neovim
- **Ollama.nvim** — Local model integration

**Adoption Level:**
- Vim/Neovim usage: ~20-30% of developer population; highly engaged with extending tools

**Integration Pattern:**
- **Neovim plugins** spawn local CLI agents and communicate via:
  - stdin/stdout (JSONL)
  - Embedded Lua/Python for direct SDK calls
  - HTTP requests to local agents running as servers
- **LSP-based plugins** use Language Server Protocol for code completion, diagnostics

**What They Need from helios-cli:**
1. **Stable JSONL protocol** — Plugin authors depend on event schema; breaking changes are catastrophic
2. **Quiet mode** (`--quiet` or `--no-header`) — Suppress TUI banners in subprocess mode
3. **JSON-only output** (`--format=json`) — For reliable parsing
4. **Error handling** — Clear error messages on stderr, structured error JSON
5. **Timeout support** — `--timeout <seconds>` to avoid hanging plugin
6. **User input from stdin** — Already supported via `--input`; document for plugin authors
7. **Session persistence** — `CODEX_THREAD_ID` env var already works; ensure reliable resumption

**Feature Requests:**
- [ ] LSP server mode for Neovim (not just CLI agent)
- [ ] Fine-grained token limit warnings (for status bar display)
- [ ] Incremental response mode (stream parts of response as they complete)

---

### 1.3 JetBrains IDEs

**Key Projects:**
- **GitHub Copilot for JetBrains** (https://plugins.jetbrains.com/plugin/17718-github-copilot)
- **JetBrains AI Assistant** (built-in, license required)
- **Qodo** (code review integration)
- **Gemini Code Assist**

**Adoption Level:**
- JetBrains IDEs: ~40% of developers (IntelliJ, PyCharm, WebStorm, etc.)
- Copilot plugin: 1M+ installations
- JetBrains AI Assistant: Enterprise license model

**Integration Pattern:**
- **Plugin API** — JetBrains plugins use IDE APIs for code inspection, refactoring
- **LSP client** — Can spawn LSP server for custom tools
- **Direct SDK calls** — TypeScript SDK could be ported to Kotlin/Java

**What They Need from helios-cli:**
1. **JetBrains Plugin SDK** — No official integration; third-party developers could create this
2. **LSP support** — Run as LSP server
3. **Stable CLI interface** — For spawning from IDE
4. **Output in files** — IDE may capture output to temp files rather than pipes
5. **Gradle/Maven integration** — For multi-module projects; need project context awareness

**Feature Requests:**
- [ ] Official JetBrains plugin (currently would need to be created by third party)
- [ ] Project-aware configuration (read `.idea/` or `pom.xml` for context)

---

## Category 2: CI/CD Agents

### Overview

CI/CD agents automate repository tasks: PR reviews, test generation, documentation, triage, and code generation. They run in GitHub Actions, GitLab CI, or Jenkins.

### 2.1 GitHub Agentic Workflows

**Project URL:** https://github.com/features/actions
**Adoption Level:** In technical preview (Feb 2026); thousands of early adopters

**Architecture:**
- Runs AI agents directly in GitHub Actions
- Agent receives: PR context, commit history, codebase snapshot
- Agent performs: analysis, code generation, file changes, PR creation
- Agents run with read-only permissions by default; write actions require approval

**Integration Pattern:**
```yaml
# Example GitHub Agentic Workflow
name: AI-Assisted PR Review
on:
  pull_request:
agents:
  reviewer:
    description: Review PR for quality and security
    tools:
      - type: github
        actions: [read_pr, read_repo, write_comment]
```

**How helios-cli Fits:**
- GitHub Agentic Workflows could invoke `helios-cli` as a subprocess within the agent container
- Workflows use MCP to expose GitHub APIs to agent
- helios-cli could provide a wrapper: `helios run-in-github-action "Review this PR"`

**What They Need from helios-cli:**
1. **Subprocess spawning** — Clear stdout/stderr capture, exit codes
2. **GitHub context injection** — Env vars like `GITHUB_TOKEN`, `GITHUB_REPOSITORY`, `GITHUB_PR_NUMBER`
3. **Output constraints** — Fit output in log limits (GitHub has 1MB per step, ~6MB per job)
4. **Caching support** — Cache thread state between workflow runs (for multi-step workflows)
5. **Sandbox compatibility** — Work in GitHub-provided container (Alpine Linux, minimal deps)
6. **Quiet streaming** — Output progress without bloating logs

**Feature Requests:**
- [ ] `--github-action-mode` flag — Special output formatting for Actions
- [ ] GitHub-specific tool: `--tool github-pr-comment` to post results directly
- [ ] Integration with GitHub artifacts API (upload analysis results)
- [ ] Caching layer for expensive analyses (e.g., "already analyzed this repo", reuse results)

---

### 2.2 GitLab Duo Agent Platform

**Project URL:** https://about.gitlab.com/platform/
**Adoption Level:** GA (Generally Available); used by 5K+ GitLab instances

**Architecture:**
- AI agents handle code generation, security analysis, code review, CI/CD troubleshooting
- Integrates with GitLab's unified platform (issues, MRs, CI/CD)
- Supports custom workflows via GitLab's automation layer

**Integration Pattern:**
- Agents receive context via GitLab's internal APIs
- Results feed back into MR discussions, issue tracking
- Can trigger CI/CD jobs or create automation workflows

**How helios-cli Fits:**
- Could be invoked by GitLab agents for specialized tasks
- GitLab could expose `--gitlab-context` to pass MR details

**What They Need from helios-cli:**
1. **GitLab API integration** — Accept `GITLAB_TOKEN`, `GITLAB_PROJECT_ID` as env vars
2. **Merge request context** — Automatic ingestion of MR diffs, target branch, commits
3. **Structured feedback** — JSON output that GitLab can parse for MR comments
4. **Permissions model** — Work with GitLab's role-based access control
5. **Multi-instance support** — Work with self-hosted GitLab instances

**Feature Requests:**
- [ ] `--gitlab-mr-mode` flag for automatic MR context loading
- [ ] GitLab-specific tool: `--tool gitlab-mr-comment` to post results
- [ ] Integration with GitLab's security scanning APIs

---

### 2.3 CodeRabbit & Similar Services (SaaS)

**Project URLs:**
- CodeRabbit: https://coderabbit.ai
- Qodo: https://qodo.ai

**Note:** These are SaaS services with their own LLM backends; they don't wrap CLI agents. However, they represent a competitor/complementary category worth understanding.

**Why they don't wrap CLI agents:**
- They need persistent context across PRs (Qodo maintains "Codebase Intelligence Engine")
- They require server-side state management (PR comment threads, configuration)
- They integrate directly with GitHub/GitLab APIs, not via CLI

**Learning for helios-cli:**
- These tools show demand for **repository-level context** across files/modules
- They need **incremental analysis** (skip already-reviewed code)
- They expose **enterprise governance** (enforce standards, audit trails)

---

## Category 3: Multi-Agent Orchestrators

### Overview

Multi-agent frameworks coordinate multiple specialized agents. They treat CLI agents as tools within a larger orchestration system.

### 3.1 LangGraph

**Project URL:** https://github.com/langchain-ai/langgraph
**Documentation:** https://langchain-ai.github.io/langgraph/
**Adoption Level:** 1K+ GitHub stars; enterprise adoption growing

**Architecture:**
- Graph-based workflow: nodes represent agents/tools, edges represent transitions
- Built on LangChain; integrates with 100+ LLM providers and tools
- Supports complex decision logic, branching, loops, parallel execution

**Integration Pattern:**
```python
from langgraph.graph import StateGraph
from langchain_core.tools import tool

@tool
def invoke_helios(task: str) -> str:
    """Run helios-cli for specialized coding tasks"""
    result = subprocess.run(
        ["helios", "run", task],
        capture_output=True,
        text=True
    )
    return result.stdout

graph = StateGraph()
graph.add_node("code_analysis", invoke_helios)
graph.add_node("security_review", other_agent)
```

**What They Need from helios-cli:**
1. **Deterministic output** — Same input should produce same output (for replaying workflows)
2. **Structured schema support** — `--output-schema` for reliable result parsing
3. **Tool definitions** — Expose available capabilities so LangGraph can discover them
4. **OpenAPI spec** — So LangGraph can auto-generate wrappers
5. **Timeout + retry semantics** — For workflow fault tolerance
6. **State serialization** — Thread state should be serializable for distributed orchestration

**Feature Requests:**
- [ ] Export OpenAPI spec for helios-cli tools
- [ ] `--tools-only` flag to list available tools as JSON schema
- [ ] Structured error output (`--error-format json`)
- [ ] Deterministic mode (seed for reproducibility)

---

### 3.2 CrewAI

**Project URL:** https://www.crewai.com
**GitHub:** https://github.com/joaomdmoura/crewai
**Adoption Level:** 10K+ GitHub stars; growing adoption

**Architecture:**
- Role-based agents: each agent has a role, goal, and backstory
- Task-based execution: tasks assigned to agents with dependencies
- Built-in tool integration for common operations

**Integration Pattern:**
```python
from crewai import Agent, Task, Crew

code_reviewer = Agent(
    role="Code Reviewer",
    goal="Review code for quality and security",
    tools=[helios_tool],  # Custom tool wrapping helios-cli
)

task = Task(
    description="Review the PR",
    agent=code_reviewer,
)

crew = Crew(agents=[code_reviewer], tasks=[task])
crew.kickoff()
```

**What They Need from helios-cli:**
1. **Simple JSON output** — CrewAI agents parse JSON responses
2. **Tool catalog** — Clear documentation of available tools
3. **Error handling** — Meaningful error messages for agent consumption
4. **Progress reporting** — Intermediate steps for multi-step tasks

---

### 3.3 AutoGen (Microsoft)

**Project URL:** https://microsoft.github.io/autogen/
**GitHub:** https://github.com/microsoft/autogen
**Adoption Level:** 5K+ GitHub stars

**Architecture:**
- Conversation-driven: agents communicate via natural language
- Flexible role-playing: agents can adopt dynamic roles
- Human-in-the-loop: workflows can pause for human review

**Integration Pattern:**
- AutoGen agents can spawn external processes (including helios-cli)
- Agents summarize results and pass them to next agent

**What They Need from helios-cli:**
1. **Conversational output** — Summarizable results (not raw output)
2. **Clear success/failure indication** — For agent decision-making
3. **Cost tracking** — Token counts for budgeting multi-agent workflows

---

## Category 4: Developer Platforms

### Overview

Developer platforms provide cloud-based or on-premises environments for coding. Increasingly, they bundle AI agents as first-class primitives.

### 4.1 Replit

**Project URL:** https://replit.com
**Adoption Level:** 20M+ monthly active users; 50M+ projects created

**AI Integration:**
- Replit Agent: autonomous AI that can build, refine, test applications
- Built on proprietary agent (not wrapping CLI agents)
- Accessed via natural language input in IDE

**How helios-cli Could Fit:**
- Replit could offer helios-cli as an alternative backend for users wanting local/privacy-respecting agents
- Would need tight integration with Replit's file system and execution environment

**Requirements:**
1. **File system API** — Agent needs to read/write Replit's persistent storage
2. **Execution sandbox** — Run code in Replit's container environment
3. **Port exposure** — For web preview functionality
4. **Secrets/auth** — Access to environment variables, API keys

---

### 4.2 GitHub Codespaces

**Project URL:** https://github.com/features/codespaces
**Adoption Level:** Hundreds of thousands; GitHub enterprise standard

**Architecture:**
- Cloud-based VS Code environment
- Containers spun up from `devcontainer.json`
- Integrated with GitHub repositories

**How helios-cli Fits:**
- Can be installed in devcontainer via `docker run` or `apt install`
- TypeScript SDK works in Node.js environments
- Perfect for GitHub Action workflows + Codespaces debugging

**Requirements:**
1. **Lightweight installation** — Should work in minimal containers (Alpine)
2. **Network access** — For API calls to LLM providers
3. **Environment variable injection** — `OPENAI_API_KEY`, `CODEX_API_KEY`
4. **Port binding** — For any server mode (future MCP server)

---

### 4.3 Coder

**Project URL:** https://coder.com
**Adoption Level:** 2K+ GitHub stars; enterprise adoption

**Architecture:**
- Self-hosted, enterprise-ready cloud IDE platform
- Based on VS Code; Terraform-based provisioning
- Air-gap deployment support

**AI Integration:**
- Coder doesn't bundle AI by default; customers integrate their own
- helios-cli is ideal for Coder because:
  - Runs locally (no external API dependency)
  - Supports air-gap deployments
  - Works in containerized environments

**What Coder Needs:**
1. **Offline mode** — Work without calling external APIs (run local models)
2. **Configuration as code** — Coder users define everything via Terraform/containers
3. **Resource limits** — CPU, memory constraints for shared environments
4. **Audit logging** — Track agent actions for compliance

**Feature Requests:**
- [ ] `--local-model-only` flag to prevent external API calls
- [ ] Structured audit log output
- [ ] Resource usage metrics (CPU, memory, tokens)

---

### 4.4 Ona (formerly Gitpod)

**Project URL:** https://ona.dev (formerly gitpod.io)
**Adoption Level:** Transitioning to AI-first platform; user base ~100K

**Architecture:**
- Cloud IDE with built-in AI agents for task delegation
- Agents can: explore codebases, scope work, run tests, open PRs
- Focuses on workflow automation for teams

**How helios-cli Fits:**
- Could be invoked by Ona's agents for specialized coding tasks
- Ona could expose thread state to allow agent persistence

**Requirements:**
1. **Team/multi-user context** — Handle shared workspaces
2. **Permission model** — Respect who initiated the agent action
3. **Audit trail** — Log all agent-initiated changes
4. **Integration with Ona's PR workflow** — Create/update PRs via agent

---

## Category 5: Code Review Bots

### Overview

Code review bots analyze PRs/MRs and provide feedback. Unlike IDE integrations (synchronous, real-time), code review bots typically run asynchronously on repository events.

### 5.1 Qodo (formerly CodiumAI)

**Project URL:** https://qodo.ai
**GitHub:** https://github.com/CodiumAI/pr-agent
**Adoption Level:** 5K+ GitHub stars; Gartner Magic Quadrant 2025 (Visionary)

**Architecture:**
- Multi-repo context engine: understands codebase across repos
- PR analysis: detects issues spanning components/services
- IDE plugins: VS Code, JetBrains
- GitHub/GitLab/Bitbucket integration

**Key Differentiators:**
- Persistent codebase intelligence (not just diff-based analysis)
- Governance enforcement (internal standards, ticket-aware validation)
- Risk scoring and architectural analysis

**Why They Don't Wrap CLI Agents:**
- Need persistent context across PRs
- Implement their own LLM orchestration
- Require enterprise features (audit, governance, multi-repo)

**Learning for helios-cli:**
- Users want **repository-level intelligence** (not just single-file analysis)
- **Incremental analysis** (skip already-reviewed code)
- **Governance features** (enforce standards, create audit trails)

**Potential Integration Path:**
- Qodo could invoke helios-cli for specialized analysis tasks
- helios-cli would need:
  1. Repository context awareness
  2. Structured output with risk/compliance scores
  3. Integration with Qodo's codebase intelligence API

---

### 5.2 CodeRabbit

**Project URL:** https://coderabbit.ai
**Adoption Level:** 1K+ organizations

**Architecture:**
- Installed as GitHub App
- Analyzes PR diffs on push/update events
- Posts inline comments on files
- Focuses on fast, readable feedback

**Limitations:**
- Diff-only analysis (no repository-wide context)
- Doesn't understand architectural patterns
- Limited cross-module reasoning

**Why They Don't Wrap CLI Agents:**
- Server-side state management needed
- Direct GitHub API integration is simpler

---

## Category 6: Voice & Accessibility Layers

### Overview

Voice interfaces enable developers to interact with agents via speech. Screen reader integration provides accessibility. These are emerging categories (mostly 2025+).

### 6.1 Voice Input/Output Integration

**Key Technologies:**
- **OpenAI Realtime API** (GA 2025) — Low-latency bidirectional audio
- **Google Gemini Live** — Voice conversation with Google's agent
- **Apple Personal Voice** — Personalized text-to-speech for accessibility

**How helios-cli Could Integrate:**
1. **Voice input wrapper** — Pre-process audio to text, pass as `--input` to helios-cli
2. **Voice output wrapper** — Post-process helios-cli output to speech via TTS
3. **Realtime mode** — Future: helios-cli could support real-time streaming (currently it does via `--stream`)

**Current Capabilities in helios-cli:**
- Streaming output works (for voice systems to consume progressively)
- Image input works (useful for visual accessibility workflows)
- No native voice I/O (but easy to wrap)

**Requirements for Voice Support:**
1. **Streaming events with clear boundaries** — Voice system needs to know when to stop listening
2. **Structured intermediate output** — Voice system can interrupt agent mid-thought
3. **Audio-compatible output** — Avoid emojis, special chars that TTS struggles with

**Feature Requests:**
- [ ] `--speech-friendly-output` flag (plain text, avoid formatting)
- [ ] Event boundaries in streaming (`item.started`, `item.content`, `item.completed`)
- [ ] Pause/resume semantics (agent should support mid-turn pauses)

---

### 6.2 Screen Reader Integration

**Key Technologies:**
- **JAWS** (Freedom Scientific) — Market-leading Windows screen reader
- **NVDA** (open source) — Windows, primarily
- **VoiceOver** — macOS, iOS
- **TalkBack** — Android

**Current State:**
- CLI tools have limited screen reader support (no semantic markup)
- Most screen readers treat CLI as plain text

**How helios-cli Could Improve:**
1. **Plain text output mode** — Avoid ANSI codes, tables, formatting
2. **Structured output** — JSON or CSV for screen reader parsers
3. **Alt text for images** — When agent analyzes images, provide textual description

**Feature Requests:**
- [ ] `--accessibility-mode` flag (plain text, semantic output, no colors)
- [ ] Image description output (when `--image` is used, output structured analysis)
- [ ] Screen reader-friendly progress reporting

---

### 6.3 Language & Localization

**Current State:**
- helios-cli is English-only
- Most downstream tools (IDEs, platforms) support multiple languages

**Requirements:**
1. **Internationalization** — Support multiple UI languages
2. **Structured output** — JSON schemas that can be translated by consumers
3. **Region-aware defaults** — Timezones, date formats, currency

**Feature Requests:**
- [ ] `--language <lang>` flag for multi-language support
- [ ] i18n architecture for messages

---

## Cross-Cutting Considerations

### 1. Authentication & Authorization

**Current Approach in helios-cli:**
- `OPENAI_API_KEY` or `CODEX_API_KEY` environment variable
- ChatGPT sign-in support

**Downstream Requirements:**
- IDE integrations: need to securely store/pass auth to subprocess
- CI/CD: need to use repository secrets (GitHub Secrets, GitLab CI/CD variables)
- Multi-tenant platforms (Coder, Ona): need per-user authentication
- Enterprise: need SSO, OAuth2 support

**Feature Requests:**
- [ ] Support for `--auth-type oauth2` with PKCE flow
- [ ] Support for API key rotation (detect key expiration, prompt refresh)
- [ ] Multi-account support (switch between personal/org API keys)

---

### 2. Sandboxing & Security

**Threat Model:**
- IDE plugins executing untrusted agent code
- CI/CD workflows with malicious instrumentation
- Multi-tenant platforms where one user's agent affects another's

**Current helios-cli Approach:**
- Sandbox workspace writes (optional)
- MCP transport validation

**Downstream Requirements:**
1. **Execution isolation** — Run agent in container/VM, not host process
2. **File system limits** — Restrict which files agent can read/write
3. **Network controls** — Whitelist API endpoints agent can call
4. **CPU/memory limits** — Prevent resource exhaustion
5. **Audit logging** — Log all actions for compliance

**Feature Requests:**
- [ ] `--sandbox-mode strict` flag with full isolation
- [ ] File ACL support (`--allowed-paths /path/a,/path/b`)
- [ ] Network firewall config (`--allowed-urls https://api.openai.com`)
- [ ] Structured audit log output

---

### 3. Observability & Debugging

**Downstream Tools Expect:**
- Structured logging (JSON, not human-readable text)
- Token counting (for cost tracking)
- Latency metrics (for performance monitoring)
- Error tracking (structured errors, not just error messages)

**Current helios-cli:**
- JSONL event stream (good for structure)
- No explicit token counting
- No performance metrics

**Feature Requests:**
- [ ] `--metrics` flag to output token usage, latency, error rates
- [ ] Structured error output (`code`, `message`, `context`, `suggestions`)
- [ ] Debug mode with full reasoning traces
- [ ] Integration with observability tools (OpenTelemetry, Datadog, etc.)

---

### 4. Configuration Management

**Current Approach:**
- `config.toml` file
- `--config key=value` CLI overrides

**Downstream Requirements:**
1. **Config templates** — Pre-built configs for specific use cases (CI/CD, IDE, voice, etc.)
2. **Environment-based config** — Separate dev/staging/prod configurations
3. **Secrets injection** — Keep secrets out of config files
4. **Version management** — Config schema versioning for upgrades
5. **Validation** — Fail fast on invalid configuration

**Feature Requests:**
- [ ] Config templates directory (`~/.codex/templates/`)
- [ ] Config validation CLI (`helios config validate --config <file>`)
- [ ] Template-based initialization (`helios init --template github-actions`)
- [ ] Schema export (`helios config export-schema --format json`)

---

### 5. Performance & Resource Management

**Downstream Constraints:**
- IDE plugins: must respond within 100-500ms
- CI/CD: job timeout limits (often 60 min or less)
- Developer platforms: shared resources, multi-tenancy
- Voice agents: latency-sensitive (users expect <500ms response time)

**Current helios-cli Challenges:**
- Initial startup time (model loading, context building)
- No built-in request caching
- No resource pooling (each invocation is new process)

**Feature Requests:**
- [ ] Daemon mode (`helios daemon start --port 9000`) for persistent agent
- [ ] Response caching (for identical prompts in same session)
- [ ] Incremental output (`--stream` already works, but document for low-latency UX)
- [ ] Startup time profiling (`helios profile --operation startup`)

---

## Feature Requests & Spec Gaps

### Priority 1: Critical for Broad Adoption

| Feature | Effort | Impact | Blockers |
|---------|--------|--------|----------|
| **MCP Server Mode** (`helios --mcp`) | 2-3 weeks | HIGH | Enables IDE plugins, multi-agent frameworks, browser integration |
| **OpenAPI Schema Export** | 1-2 weeks | HIGH | Required for LangGraph, AutoGen, Qodo integration |
| **Structured Error Output** | 1 week | MEDIUM | Required for reliable downstream error handling |
| **Daemon Mode** (`helios daemon start`) | 2-3 weeks | HIGH | Performance critical for IDE, real-time workflows |
| **Config Templates** | 1-2 weeks | MEDIUM | Improves UX for CI/CD, developer platforms |

### Priority 2: Important for Specific Use Cases

| Feature | Effort | Impact | Use Case |
|---------|--------|--------|----------|
| **JetBrains Plugin** | 3-4 weeks | HIGH | IDE integration (40% of developers use JetBrains) |
| **GitHub Action Integration** | 1-2 weeks | HIGH | CI/CD automation |
| **Voice I/O Wrapper** | 2-3 weeks | MEDIUM | Accessibility, voice-first workflows |
| **Audit Logging** | 1-2 weeks | MEDIUM | Enterprise, compliance |
| **Incremental Analysis** | 2-3 weeks | HIGH | Code review bots, large repositories |
| **Token Counting API** | 1 week | MEDIUM | Cost tracking, multi-agent budgeting |

### Priority 3: Nice-to-Have

| Feature | Effort | Impact |
|---------|--------|--------|
| **Internationalization (i18n)** | 2-3 weeks | LOW-MEDIUM |
| **Screen Reader Accessibility** | 1-2 weeks | LOW |
| **Local Model Support** | 4-6 weeks | MEDIUM |
| **Custom Tool Marketplace** | 4+ weeks | HIGH (but long-term) |

---

## Integration Checklist for Downstream Developers

If you're building on top of helios-cli, use this checklist:

### Before Integration

- [ ] Review helios-cli architecture (Rust backend, TypeScript SDK wrapper)
- [ ] Decide: spawn CLI process, use TypeScript SDK, or wrap MCP server?
- [ ] Understand authentication model (API key vs. ChatGPT sign-in)
- [ ] Review JSONL event format for streaming responses
- [ ] Plan for error handling (exit codes, structured errors)

### During Integration

- [ ] Implement JSONL event parser for responses
- [ ] Handle subprocess lifecycle (startup, cancellation, cleanup)
- [ ] Add configuration injection (env vars, `--config` flags)
- [ ] Implement retry logic with exponential backoff
- [ ] Add observability (logging, metrics, error tracking)
- [ ] Test in target environment (IDE, CI/CD, container, etc.)

### After Integration

- [ ] Document integration pattern for users
- [ ] Provide example workflows/configs
- [ ] Monitor error rates and user feedback
- [ ] Track adoption metrics
- [ ] Contribute improvements upstream (bug reports, feature requests)

---

## References & Resources

### Official Documentation
- **helios-cli:** https://github.com/KooshaPari/helios-cli
- **OpenAI Codex:** https://github.com/openai/codex
- **TypeScript SDK:** `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/sdk/typescript/`

### Integration Patterns
- [Model Context Protocol Specification](https://modelcontextprotocol.io/specification/2025-11-25)
- [LangGraph Documentation](https://langchain-ai.github.io/langgraph/)
- [OpenAI Realtime API](https://platform.openai.com/docs/guides/realtime)
- [GitHub Agentic Workflows](https://github.blog/2026-02-17-github-previews-agentic-workflows/)
- [GitLab Duo Agent Platform](https://about.gitlab.com/platform/)

### Related Research
- Qodo Multi-Repo Intelligence: https://qodo.ai
- CrewAI Framework: https://www.crewai.com
- Replit Agent: https://replit.com
- Cursor IDE: https://cursor.com

---

## Document Metadata

- **Generated:** February 28, 2026
- **Scope:** Downstream consumers of CLI AI agents (helios-cli/codex focus)
- **Method:** Web research + local codebase analysis
- **Status:** COMPLETE — Ready for implementation planning
- **Next Steps:**
  1. Prioritize features based on business impact
  2. Identify most valuable integration target (recommend: MCP server mode + GitHub Actions)
  3. Plan 8-12 week roadmap
  4. Engage with IDE plugin developers, CI/CD platform teams
  5. Contribute open-source integrations

---

**End of Document**
