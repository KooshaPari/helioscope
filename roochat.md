Ramlord
ramlord
Do Not Disturb
Official channel for Roo Code updates, releases, and important information. Updates include: plugin releases, documentation changes, major bug fixes, and community news. Only administrators and moderators can post.

Hannes

Role icon, Roo Code Dev — 12/11/25, 8:37 PM
Roo Code 3.36.5 Release Updates
@everyone GPT-5.2 support, configurable Enter key behavior, and stability fixes!

GPT-5.2 Model Support
GPT-5.2 is now available and set as the default model for the OpenAI provider:

400K context window
128K max output tokens
Configurable reasoning levels (none, low, medium, high, xhigh)
24-hour prompt cache retention

Enter Key Behavior Toggle
New setting to configure how Enter works in chat input (thanks lmtr0!):

Default: Enter sends, Shift+Enter for newline
Alternative: Enter for newline, Ctrl/Cmd+Enter sends

Find it in Settings > UI > Enter Key Behavior. Useful for multiline prompts and CJK input methods where Enter confirms character composition.

Bug Fixes
Gemini: Fixed reasoning loops and empty response errors
API errors: Fixed "Expected toolResult blocks at messages" errors during parallel tool execution
API errors: Fixed ToolResultIdMismatchError when conversation history has orphaned tool_result blocks

Provider Updates
Z.ai: Added API endpoint options for users on API billing instead of Coding plan (thanks richtong!)

Misc Improvements
Removed deprecated list_code_definition_names tool

See full release notes v3.36.5
Image
Hannes

Role icon, Roo Code Dev — 12/12/25, 7:23 PM
Roo Code 3.36.6 Release Updates
@everyone This release improves follow-up auto-approval behavior, expands tool customization options, and fixes provider settings and error reporting.

QOL Improvements
Clearer auto-approve timing in follow-up suggestions: Makes the auto-approve countdown harder to miss

Bug Fixes
Auto-approval stops when you start typing: Fixes an issue where an auto-approve timer could still fire after you began writing a response
More actionable OpenRouter error messages: Fixes an issue where failures could show only generic errors by surfacing upstream details when available
LiteLLM tool protocol dropdown always appears: Restores the tool protocol dropdown in the provider’s Advanced settings even when LiteLLM model metadata isn’t available yet
MCP tool calls work with stricter providers: Fixes failures caused by special characters in MCP server/tool names by sanitizing names and using an unambiguous mcp--server--tool ID format

Misc Improvements
Tool aliases for model-specific tool naming: Adds support for alternative tool names so different models can call the same tool using the naming they expect
Workspace task visibility controls for organizations: Adds an org-level setting for how visible Roo Code Cloud “extension tasks” are across the workspace

See full release notes v3.36.6
Image
Roo Code 3.36.6 Release Notes (2025-12-12) | Roo Code Documentation
This release improves follow-up auto-approval behavior, expands tool customization options, and fixes provider settings and error reporting.
Roo Code 3.36.6 Release Notes (2025-12-12) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 12/15/25, 8:23 PM
Roo Code 3.36.7 Release Updates
@everyone Simplifies Auto-Approve settings, improves OpenRouter behavior for OpenAI models, and expands Amazon Bedrock options.

OpenRouter tool support for OpenAI models
When you use OpenAI models via OpenRouter, Roo now makes tool usage more predictable by explicitly enabling apply_patch and avoiding file-writing tools that aren’t supported in that context.

This reduces friction when tool availability differs between providers/models and helps keep edits moving without unexpected interruptions.

Simplified Auto-Approve settings
Auto-Approve no longer has separate toggles for Retry failed requests and Update todo list—when Auto-Approve is enabled, retries and todo updates run automatically.

If you relied on those individual switches, this reduces configuration overhead and makes the Auto-Approve behavior easier to understand.

Bug Fixes
More consistent tool validation for modes: Improves reliability by consolidating mode tool-availability checks in one place

Provider Updates
More detailed OpenRouter error reporting: Captures more provider-specific error metadata so failures are easier to diagnose
AWS Bedrock service tier support: Adds a Bedrock Service tier option (Standard/Flex/Priority) for supported models (thanks Smartsheet-JB-Brown!)
Amazon Nova 2 Lite in Bedrock: Adds the Nova 2 Lite model to the Bedrock provider model list (thanks Smartsheet-JB-Brown!)

Misc Improvements
Improved web-evals run logs: Makes evaluation runs easier to inspect by improving run logs and formatting

See full release notes v3.36.7
Hannes

Role icon, Roo Code Dev — 12/16/25, 3:31 PM
Roo Code 3.36.9 Release Updates
@everyone This release defaults more providers to native tools, improves tool-call reliability across providers, and makes errors easier to debug.

Native tools by default (more providers)
Roo now defaults to the Native tool protocol for OpenAI, Azure OpenAI, OpenAI Compatible, Anthropic, Google Gemini, Vertex AI, xAI, Mistral, Groq, and Amazon Bedrock.

If you’re already using OpenRouter, you’ve been getting native tool calling by default. This release extends that same default behavior across the providers listed above. You can still switch back to XML tools in your profile’s advanced settings if you need to.

QOL Improvements
More complete streaming failure details: Improves the streaming failure UI so you can view/copy full error details directly in Roo instead of relying on the developer console
Richer error details dialog: Adds extra context (extension version, provider/model, timestamp, etc.) to the error details dialog to make debugging and reporting issues faster
Fewer read_file failures on large files: Improves large-file reading by incrementally reading up to a token budget and returning cleaner truncation when needed

Bug Fixes
Cross-provider tool-call ID compatibility: Fixes an issue where tool calls could fail when routing via OpenRouter to providers/models with stricter tool-call ID requirements
MCP nested schema compatibility: Fixes an issue where MCP tools could fail against stricter schema validation by ensuring nested tool schemas set additionalProperties: false
More reliable delegation resume: Fixes an issue where resuming a parent task after delegation could fail due to mismatched tool result IDs
Avoid deleting the wrong API messages: Fixes a race condition where deleting a user message could remove earlier assistant API messages, especially during streaming/tool use
Deduplicate MCP tools across configs: Fixes a “tool is already defined” error when the same MCP server exists in both global and project configs
Fix provider pricing page link: Fixes a broken route so the provider pricing link takes you to the correct destination

Misc Improvements
Control public task sharing: Adds an organization-level setting to disable public task sharing links
Evals UI: clearer tool grouping + duration fixes: Improves the evals UI by grouping related tools and fixing cases where run duration could be missing or incorrect

Provider Updates
Bedrock custom ARNs are less restrictive: Removes overly strict ARN validation that could block valid AWS Bedrock custom ARNs, while keeping a non-blocking region mismatch warning (thanks wisestmumbler!)
Cleaner Bedrock service tier UI: Removes extra description text under the Bedrock service tier selector to make the UI easier to scan

See full release notes v3.36.9
Roo Code 3.36.9 Release Notes (2025-12-16) | Roo Code Documentation
Roo Code 3.36.9 defaults more providers to native tools, improves reliability for tool calls and MCP schemas, and makes error details easier to debug.
Roo Code 3.36.9 Release Notes (2025-12-16) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 12/17/25, 10:09 AM
Roo Code 3.36.10 Release Updates
@everyone Gemini 3 Flash is HEREEEEEEEEEEE!!!! Also DeepSeek v3.2 actually works in Roo now!!

Gemini 3 Flash preview model
The gemini-3-flash-preview model is now available in the Roo Code Cloud provider, Google Gemini, GCP Vertex AI, Requesty, and OpenRouter providers. It’s the latest model from Google, released this morning (thanks contributors!).

DeepSeek reasoner: interleaved thinking during tool use
The DeepSeek provider’s deepseek-reasoner model now supports “interleaved thinking” and native tool calling. In our internal evals, tool calling succeeded 100% of the time, and the extended-run score improved to 93.4% (thanks zbww\_!).

Bug Fixes
Context truncation token display: Fixes an issue where the context truncation UI could show incorrect before/after token totals, especially in tool-heavy conversations

Misc Improvements
Framework updates: Updates Next.js to ~15.2.8 for improved compatibility with upstream fixes

See full release notes v3.36.10
Roo Code 3.36.10 Release Notes (2025-12-17) | Roo Code Documentation
This release adds a new Gemini 3 Flash preview model across more providers, improves DeepSeek tool calling, and fixes token count display issues.
Roo Code 3.36.10 Release Notes (2025-12-17) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 12/17/25, 3:13 PM
Roo Code 3.36.11 Release Updates
https://github.com/RooCodeInc/Roo-Code/releases/download/v3.36.11/roo-cline-3.36.11.vsix

download manually as the vscode marketplace is not updating.

@everyone SQUASH!! + Native tool calling now enabled by default for more providers!

Native Tool Calling Enhancements
Native tool calling is now enabled by default for three additional providers: Claude Code, Z.ai models (GLM-4.5 series, GLM-4.6, etc.), and OpenAI Compatible providers. This provides more direct and efficient communication with these models, improving reliability and performance when using tools.

Bug Fixes
MCP Tool Schema Normalization: Fixes an issue where MCP tool schemas could fail validation when used with Amazon Bedrock or OpenAI in strict mode by normalizing JSON Schema formats
MCP Tool Names with Bedrock: Fixes validation errors when using MCP servers with dots or colons in their names (like awslabs.aws-documentation-mcp-server) with Amazon Bedrock
Bedrock Task Resumption: Fixes an error when resuming tasks with Amazon Bedrock when native tools are disabled, where users would encounter The toolConfig field must be defined errors
Roo Code Cloud Model Refresh: Fixes an issue where authentication-required models (like google/gemini-3-flash) wouldn't appear immediately after logging into Roo Code Cloud
AWS GovCloud and China Region Support: Fixes an issue where users in AWS GovCloud and China regions couldn't use custom ARNs with the Bedrock provider (thanks
wisestmumbler!)

See full release notes v3.36.11
Roo Code 3.36.11 Release Notes (2025-12-17) | Roo Code Documentation
Native tool calling now enabled by default for Claude Code, Z.ai, and OpenAI Compatible providers, plus several Bedrock and MCP compatibility fixes.
Roo Code 3.36.11 Release Notes (2025-12-17) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 12/18/25, 8:49 PM
Roo Code 3.36.12-3.36.14 Release Updates
@everyone Native tool calling becomes the default across all providers with improved file editing and better error handling.

Native Tool Calling Improvements
Native Protocol is Now Default: Models that support native tool calling now use it by default instead of XML. The XML protocol is still available in provider settings.
Smarter Tool Defaults for Gemini and OpenAI: Gemini and OpenAI models now use better default tools for file editing, improving reliability out of the box.
Native Tool Calling for Claude on Vertex AI: All Claude models on Vertex AI now use native tool calling by default, matching the behavior of direct Anthropic API access.
VS Code LM Native Tools: Native tool calling now works with VS Code's built-in Copilot models.
LiteLLM Tool Protocol Dropdown: The Native/XML protocol selector now appears correctly for LiteLLM models.
Task Resumption Fix: Tasks no longer break when resuming after changing the Native Tool Calling setting.

QOL Improvements
Improved File Editing with Gemini Models: New edit_file tool makes Gemini models more effective at editing files.
Grace Retry for Tool Errors: When models fail to use tools, Roo Code now silently retries before showing errors. Clearer "Model Response Incomplete" messages appear only after consecutive failures.

Bug Fixes
Bedrock Embedder CloudTrail Fix: AWS Bedrock users now see Roo Code identified in CloudTrail logs when using Codebase Indexing.
MCP Compatibility with OpenAI Providers: Fixes an issue where MCP servers using format: "uri" in their tool schemas would fail with OpenAI providers.

Misc Improvements
Better Error Grouping: Improved error tracking for faster issue resolution.
Error Monitoring: Improved tracking of consecutive mistake errors.

See full release notes v3.36.12 | v3.36.13 | v3.36.14
Roo Code 3.36.12 Release Notes (2025-12-18) | Roo Code Documentation
Improved tool preferences for Gemini and OpenAI models, Bedrock embedder fix, and better error tracking.
Roo Code 3.36.12 Release Notes (2025-12-18) | Roo Code Documentation
Roo Code 3.36.13 Release Notes (2025-12-18) | Roo Code Documentation
Native tool protocol becomes the new default for compatible models, plus new edit_file tool and VS Code LM native tool support.
Roo Code 3.36.13 Release Notes (2025-12-18) | Roo Code Documentation
Roo Code v3.36.14 Release Notes (2025-12-18) | Roo Code Documentation
Native tool calling for Claude models on Vertex AI, improved error handling with grace retry, and MCP compatibility fixes for OpenAI providers.
Roo Code v3.36.14 Release Notes (2025-12-18) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 12/19/25, 2:52 PM
Roo Code 3.36.15 Release Updates
@everyone This release adds a 1M context window option for Claude Sonnet 4.5 on Vertex AI, improves chat error troubleshooting, and expands native tool calling support.

Vertex AI: 1M context window for Claude Sonnet 4.5
When you use Claude Sonnet 4.5 on Vertex AI, you can now enable a 1M context window option for supported models.

Chat error troubleshooting improvements
Chat error states now make it easier to understand what went wrong and to share the right details when filing a bug report:

Clearer error visibility: Error rows more consistently surface full error details (including status codes) via a more obvious View details affordance
Downloadable diagnostics: You can generate a local diagnostics file from a chat error (including error metadata and the API conversation history) so you can review/redact and share it with an issue report

Bug Fixes
Native tool calling support for LM Studio and Qwen-Code: Fixes an issue where these providers were missing OpenAI-style native tool call support.
More reliable tool defaults for OpenAI Compatible providers: Fixes cases where tool calling could be inconsistent unless you manually adjusted custom model info, by applying native tool defaults unless you’ve explicitly overridden them
Requesty native tool calls enabled: Fixes native tool calling defaults for the Requesty provider (and aligns behavior for Unbound) so tool use is more consistent, especially when model metadata is cached
Strict JSON Schema compatibility: Fixes an issue where some MCP tool schemas could fail strict validation due to missing additionalProperties: false on object schemas
Refresh models cache reliability: Fixes an issue where Refresh models could fail to fully flush/refresh cached model lists for some providers, and improves correctness of initial model selection when starting a new task

See full release notes v3.36.15
Roo Code 3.36.15 Release Notes (2025-12-19) | Roo Code Documentation
Roo Code 3.36.15 adds a 1M context window option for Claude Sonnet 4.5 on Vertex AI, improves chat error troubleshooting, and expands native tool calling support.
Roo Code 3.36.15 Release Notes (2025-12-19) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 12/23/25, 12:24 AM
Roo Code 3.37 Release Updates
@everyone GLM 4.7, MM2.1, experimental custom tools, and MOOOOOOREEE!!!

New models
Z.ai GLM-4.7 (thinking mode)
GLM-4.7 is now available directly through the Z.ai provider in Roo Code, as well as via the Roo Code Cloud provider (and other provider routes that surface Z.ai). It’s a strong coding model for agentic workflows, with improved multilingual coding, terminal tasks, tool use, and complex reasoning compared to GLM-4.6.

MiniMax M2.1 improvements
MiniMax M2.1 is now available directly through the MiniMax provider in Roo Code, as well as via the Roo Code Cloud provider (and other provider routes that surface MiniMax). It’s a strong pick for agentic coding workflows, with better tool use, instruction following, and long-horizon planning for multi-step tasks—and it’s fast.

Experimental custom tools
You can now define and use custom tools so Roo can call your project- or team-specific actions like built-in tools. This makes it easier to standardize workflows across a team by shipping tool schemas alongside your project, instead of repeatedly re-prompting the same steps.

Bug Fixes
Fixes an issue where Roo could appear stuck after a tool call with some OpenAI-compatible providers when streaming ended at the tool-calls boundary (thanks torxeon!)
Fixes an issue where Roo could appear stuck after a tool call with some OpenAI-compatible providers by ensuring final tool-call completion events are emitted
Fixes an issue where MCP tools could break under strict schema mode when optional parameters were treated as required
Fixes an issue where the built-in read_file tool could fail on some models due to invalid schema normalization for optional array parameters
Fixes an issue where search_replace / search_and_replace could miss matches on CRLF files, improving cross-platform search-and-replace reliability
Fixes an issue where Requesty’s Refresh Models could leave the model list stale by not including credentials in the refresh flow (thanks requesty-JohnCosta27!)
Fixes an issue where Chutes model loading could fail if the provider returned malformed model entries
Fixes an issue where reasoning_details could be merged/ordered incorrectly during streaming, improving reliability for providers that depend on strict reasoning serialization
Fixes an issue where DeepSeek-reasoner could error after condensation if the condensed summary lacked required reasoning fields

Misc Improvements
Cleaner eval logs: Deduplicates repetitive message log entries so eval traces are easier to read

QOL Improvements
New tasks now default to native tool calling on models that support it, reducing the need for manual tool protocol selection

Provider Updates
Improves Z.ai thinking model message formatting by attaching environment_details to tool results instead of emitting separate system messages
LiteLLM no longer sends parallel_tool_calls, improving tool-call compatibility (thanks farazoman!)

See full release notes v3.37
Roo Code 3.37 Release Notes (2025-12-22) | Roo Code Documentation
Roo Code 3.37 introduces custom tool calling, expands provider support, and improves tool-call reliability.
Roo Code 3.37 Release Notes (2025-12-22) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 12/23/25, 7:18 PM
Roo Code 3.37.1 Release Updates
@everyone BUG FIXES on tool-calling and chat reliability issues!! Sorry about 3.37.0!!! 😮

QOL Improvements
Improves tool-calling reliability for Roo Code Cloud by preventing tool-result metadata (like environment_details) from interrupting tool call sequences
Improves tool-calling reliability across OpenAI-compatible providers by merging trailing tool-result text into the last tool message, reducing cases where tool call sequences get interrupted

Bug Fixes
Fixes an issue where Roo could show errors when a provider returned an empty assistant message by retrying once and only showing an error if the problem repeats
Fixes an issue where OpenAI/OpenAI-compatible chats could fail to use native tools when custom model info didn’t explicitly set tool support, by sending native tool definitions by default
Fixes an issue where Roo could send malformed reasoning_details data after transforming conversation history, preventing provider-side errors and improving compatibility with OpenAI Responses-style reasoning blocks
Fixes an issue where “ask” flows could hang if your reply was queued instead of being delivered as an ask response, so conversations continue reliably

Misc Improvements
Provider-centric signup: Makes Roo easier to get started with by defaulting the welcome/sign-up flow to the Roo provider (while keeping other providers available), so you can reach a working setup faster with fewer choices up front

See full release notes v3.37.1
Roo Code 3.37.1 Release Notes (2025-12-23) | Roo Code Documentation
Roo Code 3.37.1 refreshes the signup flow, improves OpenAI-compatible tool calling, and fixes several reliability issues.
Roo Code 3.37.1 Release Notes (2025-12-23) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 12/24/25, 9:23 PM
@everyone Well shit. Looks like we messed up a bit.

https://blog.roocode.com/p/sorry-we-didnt-listen-sooner-native

Thanks you @NaccOll for your belief and persistence on Native Tool Calling.

Oh yeah and Merry Xmas community!!
Sorry we didn’t listen sooner: Native Tool Calling
Well shit. Looks like we messed up a bit.
Sorry we didn’t listen sooner: Native Tool Calling
Hannes

Role icon, Roo Code Dev — 12/29/25, 1:46 PM
Roo Code 3.38.0 Release Updates
@everyone Agent Skills have arrived in Roo Code!!

Agent Skills
Roo now supports Agent Skills, which are portable skill folders containing instructions, scripts, and resources that the agent can discover and load on demand. This lets you package repeatable workflows and domain knowledge once and reuse them across projects to make results more consistent and reliable.

📚 Documentation: See Skills for setup and usage.

QOL Improvements
Slash commands can declare a target mode in their front matter, so triggering a command can switch Roo to the right mode first.
Removes the legacy “simple read file” tool path so file reading consistently uses the standard read_file tool.

Bug Fixes
Fixes an issue where some Claude Sonnet 4.5 requests could fail with HTTP 400 errors after context condensing.

Misc Improvements
Custom tools can import npm packages, and can load secrets from a same-folder .env file.

Provider Updates
Removes the “OpenRouter Transforms” setting and stops sending the transforms parameter on OpenRouter requests.

See full release notes v3.38.0
Image
Hannes

Role icon, Roo Code Dev — 12/29/25, 6:27 PM
Roo Code 3.38.1 Release Updates
@everyone Sorry for the second release announcement in one day! Playing catchup from Christmas break! This is a bug fix release! More to come as we continue to fine tune native tool calling. Ty for your patience.

QOL Improvements
Clarifies the navigation path for deleting a Roo Code Cloud account in the privacy policy, so you can find Security Settings faster
Improves internal analytics consistency by recording custom tools the same way as MCP tools, which helps tool usage and error tracking stay accurate

Bug Fixes
Fixes an issue where manually condensing context while a tool is still running could trigger provider protocol errors, so tool runs and manual condenses can safely overlap
Reverts a change for OpenAI-compatible providers to avoid message-format regressions related to text appearing after tool results
Enforces the maxConcurrentFileReads limit for read_file, returning a clear error instead of silently ignoring the setting
Improves the error shown when read_file is run on a directory by explicitly explaining the issue and suggesting list_files instead
Fixes an issue where the "open source" link on roocode.com could point to a non-existent GitHub organization, so it reliably opens the correct repository (thanks jishnuteegala!)

See full release notes v3.38.1
Image
Hannes

Role icon, Roo Code Dev — 12/31/25, 5:27 PM
Roo Code 3.38.2 Release Updates
@everyone This release improves Agent Skills compatibility, clarifies rate-limit status, and few other tweaks and fixes.

QOL Improvements
Agent Skills load more reliably across environments by aligning skill metadata validation and discovery with the Agent Skills specification.
The recommended onboarding video is easier to find by updating the “Tutorial & Feature Videos” README section to point to the newer Context Management tutorial (including localized READMEs) (thanks SannidhyaSah!).

Bug Fixes
Fixes an issue where rate limiting looked like a provider error so a client-side rate limit now shows as a normal “rate limit wait” status row and completes cleanly.
Fixes an issue where write_to_file could create files at an unintended truncated path, reducing accidental file placement and cleanup.

Provider Updates
Removes the Human Relay provider so it no longer appears as a configurable option in Providers.
Updates the Cerebras default max completion tokens to 16,384 to reduce premature rate-limit estimation issues (thanks sebastiand-cerebras!).

See full release notes v3.38.2
Roo Code 3.38.2 Release Notes (2025-12-31) | Roo Code Documentation
This release improves Agent Skills compatibility, clarifies rate-limit status, and updates provider defaults.
Roo Code 3.38.2 Release Notes (2025-12-31) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 1/8/26, 7:45 PM
Roo Code 3.39.0-3.39.1 Release Updates
@everyone This release adds image file @mentions, makes provider profiles stick to tasks, renames YOLO to BRRR and a BUNCH of other fixes!!!

Image file @mentions
You can now @mention image files to include them as inline images in your message, making it easier to share screenshots and UI mockups without manually attaching files (thanks hannesrudolph!).

Sticky provider profiles
Tasks now remember the provider profile (API configuration) they started with, so switching profiles elsewhere doesn’t affect running tasks or resumed tasks (thanks hannesrudolph!).

YOLO → BRRRRRRRRRR
The auto-approve mode label has been renamed from “YOLO” to “BRRRRRRRRRR” across the UI. THANKS @SLOPTIMUS PRIME

QOL Improvements
The @ file picker now respects .rooignore, reducing noise in large workspaces and helping you avoid accidentally attaching ignored/generated files (thanks app/roomote, jerrill-johnson-bitwerx!)
Adds debug-only proxy routing settings so you can inspect extension network traffic while running under the VS Code debugger (F5) (thanks hannesrudolph, SleeperSmith!)
Improves the follow-up suggestion mode badge styling for better readability (thanks mrubens!)
Clarifies in the native read_file tool description that image formats are supported when the model supports vision (thanks app/roomote, nabilfreeman!)

Bug Fixes
Fixes an issue where conversations could fail after condensation due to missing/mismatched tool call IDs, improving reliability in longer chats (thanks daniel-lxs!)
Fixes an issue where duplicate tool_result blocks could cause provider API errors (including Anthropic “duplicate toolResult” failures), improving reliability in tool-heavy workflows (thanks daniel-lxs!)
Fixes an edge case where switching terminals mid-run could produce duplicate tool results and trigger protocol errors, reducing unattended-mode soft-locks (thanks app/roomote, nabilfreeman!)
Fixes an issue where Roo could generate the wrong command chaining syntax on Windows, making suggested terminal commands more likely to work without edits (thanks app/roomote, AlexNek!)
Fixes an issue where chat requests could fail on Windows systems without PowerShell in PATH (“spawnSync powershell ENOENT”) (thanks app/roomote, Yang-strive!)
Fixes a rare edge case where an API rate limit setting could be ignored when provider state is temporarily unavailable (thanks app/roomote!)
Fixes validation failures in nightly builds by adding missing setting descriptions for debug proxy configuration (thanks app/roomote!)
Fixes an issue where file paths shown during native tool-call streaming could appear incorrect or truncated, making it harder to confirm which file Roo is reading or editing
Fixes an issue where resuming a task with Gemini models that use extended thinking could fail with a “Corrupted thought signature” / INVALID_ARGUMENT error
Fixes an issue where ask_followup_question could fail with some Anthropic-backed setups due to strict tool schema validation

Provider Updates
Provider/model list updates and compatibility improvements across multiple providers (e.g., Fireworks AI, OpenAI-compatible endpoints, Cerebras, Bedrock), including new model options and removing legacy/unsupported entries.

Misc Improvements
CLI improvements: simpler install/upgrade workflow plus early-stage CLI support used by eval tooling.

See full release notes v3.39.0 | v3.39.1
Image
Hannes

Role icon, Roo Code Dev — 1/11/26, 12:02 PM
Roo Code 3.39.2-3 Release Updates
@everyone This release adds a new debug toggle in settings, improves provider error messaging, renames Roo Code Cloud Provider to Roo Code Router, and improves CLI and internal organization.

Roo Code Router rename
Roo Code now shows Roo Code Router anywhere the extension previously referred to the Roo Code Cloud Provider. The new name is meant to make the intent clearer: Roo Code Router is a router-style provider (similar in idea to other router/aggregator services), but with a curated set of models we’ve tested with Roo Code.

Bug Fixes
Fixes an issue where Cerebras could fail tool calling with a 422 error when a chat included both built-in tools and MCP tools, by ensuring tools sent to Cerebras use consistent strict-mode values
Fixes an issue where OpenAI-compatible providers could break tool calling, causing models to describe tool usage instead of invoking tools
Fixes cases where file edits could fail to apply due to overly strict edit_file matching, improving edit reliability
Fixes an issue where the VS Code Language Model (vscode-lm) integration could error when tool-call parts were ordered before text parts in assistant messages
Fixes an issue where Gemini requests via OpenRouter could fail when an assistant message contained only tool calls
Fixes an issue where approving a tool run while adding feedback could produce duplicate tool results, improving reliability for unattended runs
Fixes an issue where Gemini tool calling could behave inconsistently across turns due to thought signatures not being preserved correctly

Provider Updates
Improves file-edit reliability for Gemini and Vertex models by disabling the edit_file tool for those providers so Roo can route edits through the more appropriate patch-based flow when available
Improves Gemini tool selection behavior by no longer overriding tool allow/deny lists, letting Roo’s routing decide more consistently

QOL Improvements
Adds an Enable debug mode toggle in Roo Code settings (About Roo Code) so you can turn debug logging on/off from the extension UI
Improves the error message shown when a provider terminates a streaming response early (for example timeouts or rate limits), making it clearer the interruption came from the provider
Roo Code Router naming consistency: Updates the Roo Code Router service name and related wording so the UI uses consistent naming
CLI improvements: Improves CLI ergonomics, testing, and internal organization (https://github.com/RooCodeInc/Roo-Code/pull/10480, https://github.com/RooCodeInc/Roo-Code/pull/10597, https://github.com/RooCodeInc/Roo-Code/pull/10599)

Misc Improvements
Improves CLI defaults by switching the default model to anthropic/claude-opus-4.5
Updates Roo Code Cloud’s Terms of Service (effective 2026-01-09)
Improves internal CLI development and maintenance by moving more shared types to @roo-code/types
Improves internal CLI stability by adding new functionality to @roo-code/core
Improves internal CLI workflows by adding additional slash commands for CLI development

See full release notes v3.39.2 | v3.39.3
Image
Hannes

Role icon, Roo Code Dev — 1/13/26, 10:32 PM
Roo Code 3.40.0-3.40.1 Release Updates
@everyone This ROOOOOLEASE adds settings search, improves the stop button and error display, fixes tool-calling compatibility issues, and fixes a mode-switching error when using Gemini.

Settings search
You can now quickly find and jump to the setting you need with a dedicated search inside Roo Code settings. Instead of hunting through sections, you can search by keyword and jump straight to the right setting, with a cleaner results layout that’s easier to scan.

Stop button improvements
Stopping a streaming response is now clearer and more consistent with a standard stop button, with better visibility while editing messages. The stop action stays visible in more situations and replaces the old, oversized cancel UI, so interrupting long responses feels more familiar and less visually disruptive.

Tool-calling compatibility fixes
This release improves compatibility across providers (especially Gemini and OpenAI-compatible backends) by addressing request/response validation edge cases (thanks Idlebrand!). Roo now avoids sending tool-calling parameters that some backends reject and handles cases where tool output is empty, reducing validation failures that could previously break tool-using chats mid-run.

QOL Improvements
Errors in chat are easier to interpret, with improved styling/visibility and more complete details when something goes wrong.
The stop button stays visible and more consistent while editing messages, making it easier to interrupt long responses when needed.
Roo uses a standard stop button while streaming, making task cancellation more familiar and less visually disruptive.

Bug Fixes
Fixes an issue where some LiteLLM routes could fail during native tool use because an unsupported tool-calling parameter was always being sent.
Fixes an issue where Gemini-based providers could reject tool results when the tool output was empty, causing request validation errors mid-run.
Fixes an issue where switching modes (e.g., from Code to Architect) while using Gemini would cause API errors due to tool permission conflicts in the conversation history.

See full release notes v3.40.0 | v3.40.1
Image
Hannes

Role icon, Roo Code Dev — 1/14/26, 11:51 PM
Roo Code 3.41.0 Release Updates
@everyone This release adds OpenAI Codex subscription access, expands OpenAI model choices, and improves stability.

OpenAI Codex provider with OAuth subscription access
Adds the OpenAI Codex provider with subscription-based OAuth sign-in, so you can connect a ChatGPT Plus/Pro account to use Codex models without API keys or per-token billing. Just select OpenAI - ChatGPT Plus/Pro in the provider settings!

GPT-5.2-Codex model option for OpenAI (Native)
Adds the GPT-5.2-Codex model to the OpenAI (Native) provider so you can select the coding-optimized model with its expanded context window and reasoning effort controls.

Bug Fixes
Gemini sessions no longer fail after a provider switch: Resolves a streaming error where LiteLLM Gemini tool calls could fail with corrupted thought signatures when switching models mid-task.
Long terminal runs no longer degrade memory: Fixes a memory leak where large command outputs could keep growing buffers after completion, leading to gray screens during long sessions.

Misc Improvements
End-to-end tests run reliably again: Restores MCP and subtask coverage and fixes flaky tool tests so contributors can run CI-like checks locally and catch regressions earlier. (thanks ArchimedesCrypto, dcbartlett!)
Automated tests no longer stall on tool approvals: Fixes a problem where MCP end-to-end tests could hang on manual approval prompts by auto-approving time server tools. (thanks ArchimedesCrypto!)

See full release notes v3.41.0
Roo Code 3.41.0 Release Notes (2026-01-15) | Roo Code Documentation
Adds OpenAI Codex access via OAuth subscriptions, expands OpenAI model options, and fixes stability issues.
Roo Code 3.41.0 Release Notes (2026-01-15) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 1/16/26, 8:10 PM
Roo Code 3.41.1-3.41.2 Release Updates
@everyone This update improves Orchestrator cost visibility, makes settings navigation faster, and adds a new way to read long Markdown replies.

Open markdown replies in VS Code preview
Markdown responses in chat can now be opened directly in VS Code’s Markdown Preview, making longer formatted replies easier to read and navigate without scrolling through the chat bubble.

Aggregated Orchestrator subtask costs
Orchestrator parent tasks now show a cumulative cost total that aggregates spend across the full subtask tree, making it easier to understand the true end-to-end cost of an orchestrated workflow at a glance.

Settings search now includes tab titles
Settings search now indexes tab/section titles (like “MCP” or “Providers”), so you can search for a section name and jump directly to the right part of the settings UI instead of hunting through individual settings.

QOL Improvements
Clarifies that Slack and Linear integrations are only available on the Roo Code Cloud Team plan, reducing confusion when comparing plans.

Bug Fixes
Fixes an issue where OpenAI – ChatGPT Plus/Pro could incorrectly require an API key during setup, blocking you on provider selection even though it uses OAuth sign-in.
Fixes an issue where the OpenAI Codex provider could keep an invalid model selection after switching providers, restoring a valid model choice automatically.
Fixes an issue where Anthropic native tool calling could fail with a 400 error after stream retries/reconnections due to duplicate tool call IDs.
Fixes an issue where OpenAI Native streaming could drop tool identity fields mid-stream, causing tool calls to be missed or fail in some OpenAI-compatible streams.
Fixes an issue where the OpenAI Responses API could reject tool calls when call_id exceeded the 64-character limit, preventing 400 validation errors during tool use.
Fixes an issue where Gemini requests (for example via OpenRouter) could fail with thought-signature validation errors mid-task, improving reliability for Gemini tool-calling chats.
Fixes an issue where Gemini requests could fail when a user message contained an empty text block, ensuring only non-empty content is sent while preserving images.
Fixes an issue where some MCP tool schemas could be rejected by providers when they used top-level anyOf/oneOf/allOf, improving cross-provider tool schema compatibility.

Provider Updates
Prevents Ollama setups from offering models that can’t use native tool calling, avoiding loops where the model outputs tool JSON as text instead of actually invoking tools.
Fixes an issue where Gemini requests routed through LiteLLM could fail when the model name contained spaces, improving compatibility for Gemini model group naming.

See full release notes v3.41.1 | v3.41.2
Hannes

Role icon, Roo Code Dev — 1/22/26, 11:29 PM
Roo Code 3.42.0 Release Updates
@everyone This release adds ChatGPT usage tracking for the OpenAI Codex provider, refreshes provider options and model selection UX, and improves reliability for prompts, exporting, and editing safety.

QOL Improvements
Adds a usage limits dashboard in the OpenAI Codex provider so you can track your ChatGPT subscription usage and avoid unexpected slowdowns or blocks.
Standardizes the model picker UI across providers, reducing friction when switching providers or comparing models.
Warns you when too many MCP tools are enabled, helping you avoid bloated prompts and unexpected tool behavior.
Makes exports easier to find by defaulting export destinations to your Downloads folder.
Clarifies how linked SKILL.md files should be handled in prompts.

Bug Fixes
Fixes an issue where switching workspaces could temporarily show an empty Mode selector, making it harder to confirm which mode you’re in.
Fixes a race condition where the context condensing prompt input could become inconsistent, improving reliability when condensing runs.
Fixes an issue where OpenAI native and Codex handlers could emit duplicated text/reasoning, reducing repeated output in streaming responses.
Fixes an issue where resuming a task via the IPC/bridge layer could abort unexpectedly, improving stability for resumed sessions.
Fixes an issue where file restrictions were not enforced consistently across all editing tools, improving safety when using restricted workflows.
Fixes an issue where a “custom condensing model” option could appear even when it was no longer supported, simplifying the condense configuration UI.
Fixes gray-screen performance issues by avoiding redundant task history payloads during webview state updates.

Misc Improvements
Improves prompt formatting consistency by standardizing user content tags to <user_message>.
Removes legacy XML tool-calling support so new tasks use the native tool format only, reducing confusion and preventing mismatched tool formatting across providers.
Refactors internal prompt plumbing by migrating the context condensing prompt into customSupportPrompts.

Provider Updates
Removes the deprecated Claude Code provider from the provider list.
Enables prompt caching for the Cerebras zai-glm-4.7 model to reduce latency and repeat costs on repeated prompts.
Adds the Kimi K2 thinking model to the Vertex AI provider.

See full release notes v3.42.0
Roo Code 3.42.0 Release Notes (2026-01-22) | Roo Code Documentation
This release adds ChatGPT usage tracking for the OpenAI Codex provider, refreshes provider options and model selection UX, and improves reliability for prompts, exporting, and editing safety.
Roo Code 3.42.0 Release Notes (2026-01-22) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 1/26/26, 8:38 PM
Roo Code 3.43.0 Release Updates
@everyone This release updates Intelligent Context Condensation, removes deprecated settings, and fixes export and settings issues.

Intelligent Context Condensation v2
Intelligent Context Condensation runs when the conversation is near the model’s context limit. It summarizes earlier messages instead of dropping them. After a condense, Roo continues from a single summary, not a mix of summary plus a long tail of older messages. If your task starts with a slash command, Roo preserves those slash-command-driven directives across condenses. Roo is less likely to break tool-heavy chats during a condense, which reduces failed requests and missing tool results.

Settings changes: the Condense prompt editor is now under Context Management and Reset clears your override. Condensing uses the active conversation model/provider. There is no separate model/provider selector for condensing.

QOL Improvements
Removes the unused “Enable concurrent file edits” experimental toggle to reduce settings clutter.
Removes the experimental Power Steering setting (a deprecated experiment that no longer improves results).
Removes obsolete diff/match-precision provider settings that no longer affect behavior.
Adds a pnpm install:vsix:nightly command to make installing nightly VSIX builds easier.

Bug Fixes
Fixes an issue where MCP config files saved via the UI could be rewritten as a single minified line. Files are now pretty-printed. (thanks Michaelzag!)
Fixes an issue where exporting tasks to Markdown could include [Unexpected content type: thoughtSignature] lines for some models. Exports are now clean. (thanks rossdonald!)
Fixes an issue where the Model section could appear twice in the OpenAI Codex provider settings.

Misc Improvements
Removes legacy XML tool-calling code paths that are no longer used, reducing maintenance surface area.

Provider Updates
Updates Z.AI models with new variants and pricing metadata. (thanks ErdemGKSL!)
Corrects Gemini 3 pricing for Flash and Pro models to match published pricing. (thanks rossdonald!)

See full release notes v3.43.0
Roo Code 3.43.0 Release Notes (2026-01-23) | Roo Code Documentation
This release improves Intelligent Context Condensation, updates providers, and fixes export and settings issues.
Roo Code 3.43.0 Release Notes (2026-01-23) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 1/27/26, 4:17 PM
Roo Code 3.44 Release Updates
@everyone Worktrees ARE HERE!!!!! + parallel tool calls are back (experimental) and a whole slew of other improvements!!

Worktrees
Worktrees are easier to work with in chat. The Worktree selector is more prominent, creating a worktree takes fewer steps, and the Create Worktree flow is clearer (including a native folder picker), so it’s faster to spin up an isolated branch/workspace and switch between worktrees while you work.

📚 Documentation: See Worktrees for detailed usage.

Parallel tool calls (Experimental)
Re-enables parallel tool calling (with added isolation safeguards) so you can use the experimental “Parallel tool calls” setting again without breaking task delegation workflows.

QOL Improvements
Makes subtasks easier to find and navigate by improving parent/child visibility across History and Chat (including clearer “back to parent” navigation), so you can move between related tasks faster.
Lets you auto-approve all tools from a trusted MCP server by allowing all tool names, so you don’t have to list each one individually.
Reduces token overhead in prompts by removing a duplicate MCP server/tools section from internal instructions, leaving more room for your conversation context.
Improves Traditional Chinese (zh-TW) UI text for better clarity and consistency. (thanks PeterDaveHello!)

Bug Fixes
Fixes an issue where context condensing could accidentally pull in content that was already condensed earlier, which could reduce the effectiveness of long-conversation summaries.
Fixes an issue where automatic context condensing could silently fail for VS Code LM API users when token counting returned 0 outside active requests, which could lead to unexpected context-limit errors. (thanks srulyt!)
Fixes an issue where Roo didn’t record a successful truncation fallback when condensation failed, which could make Rewind restores unreliable after a condensing error.
Fixes an issue where MCP tools with hyphens in their names could fail to resolve in native tool calling (for example when a provider/model rewrites “-” as “\_”). (thanks hori-so!)
Fixes an issue where tool calls could fail validation through AWS Bedrock when the tool call ID exceeded Bedrock’s 64-character limit, improving reliability for longer tool-heavy sessions.
Fixes an issue where Settings section headers could look transparent while scrolling, restoring an opaque background so the UI stays legible.
Fixes a Fireworks provider type mismatch by removing unsupported model tool fields, keeping provider model metadata consistent and preventing breakage from schema changes.
Fixes an issue where task handoffs could miss creating a checkpoint first, making task state more consistent and recoverable.
Fixes an issue where leftover Power Steering experiment references could display raw translation keys in the UI.
Fixes an issue where Roo could fail to index code in worktrees stored inside hidden directories (for example “~/.roo/worktrees/”), which could break search and other codebase features in those worktrees.

Provider Updates
5 provider updates — see full release notes for more detail.

See full release notes 3.44
Hannes

Role icon, Roo Code Dev — 1/27/26, 10:07 PM
Roo Code 3.45.0 Release Updates
@everyone One more thing.. This release adds Smart Code Folding so context condensing preserves a lightweight map of the code you were working on.

Smart Code Folding
When Roo condenses a long conversation, it now keeps a lightweight “code outline” for recently used files—things like function signatures, class declarations, and type definitions—so you can keep referring to code accurately after condensing without re-sharing files. (thanks @Shariq Riaz!)

📚 Documentation: See Intelligent Context Condensing for details on configuring and using context condensing.

See full release notes v3.45.0
Hannes

Role icon, Roo Code Dev — 1/30/26, 6:37 PM
Roo Code 3.46.0 Release Updates
@everyone This is a BIG UPDATE! This release adds parallel tool calling, overhauls how Roo reads files and handles terminal output, and begins a major refactor to use the AI SDK at Roo's core for much better reliability. Together, these changes shift how Roo manages context and executes multi-step workflows in a serious way! Oh, and we also added a UI to manage your skills!!

This is not hype.. this is truth.. you will 100% feel the changes (and see them). Make sure intelligent context condensing is not disabled, its not broken anymore. And reset the prompt if you had customized it at all.

Parallel tool calling
Roo can now run multiple tools in one response when the workflow benefits from it. This gives the model more freedom to batch independent steps (reads, searches, edits, etc.) instead of making a separate API call for each tool. This reduces back-and-forth turns on multi-step tasks where Roo needs several independent tool calls before it can propose or apply a change.

Total read_file tool overhaul
Roo now caps file reads by default (2000 lines) to avoid context overflows, and it can page through larger files as needed. When Roo needs context around a specific line (for example, a stack trace points at line 42), it can also request the entire containing function or class instead of an arbitrary “lines 40–60” slice. Under the hood, ⁨read_file⁩ now has two explicit modes: slice (⁨offset⁩/⁨limit⁩) for chunked reads, and indentation (anchored on a target line) for semantic extraction. (thanks pwilkin!)

Terminal handling overhaul
When a command produces a lot of output, Roo now caps how much of that output it includes in the model’s context. The omitted portion is saved as an artifact. Roo can then page through the full output or search it on demand, so large builds and test runs stay debuggable without stuffing the entire log into every request.

Skills management in Settings
You can now create, edit, and delete Skills from the Settings panel, with inline validation and delete confirmation. Editing a skill opens the ⁨SKILL.md⁩ file in VS Code. Skills are still stored as files on disk, but this makes routine maintenance faster—especially when you keep both Global skills and Project skills. (thanks SannidhyaSah!)

Provider migration to AI SDK
We’ve started migrating providers toward a shared Vercel AI SDK foundation, so streaming, tool calling, and structured outputs behave more consistently across providers. In this release, that migration includes shared AI SDK utilities plus provider moves for Moonshot/OpenAI-compatible, DeepSeek, Cerebras, Groq, and Fireworks, and it also improves how provider errors (like rate limits) surface.

Boring stuff
More misc improvements are included in the full release notes: https://docs.roocode.com/update-notes/v3.46.0
Roo Code 3.46.0 Release Notes (2026-01-30) | Roo Code Documentation
This release adds parallel tool calling, overhauls file reading and terminal output handling, begins a shift toward the AI SDK, and adds a Settings UI for managing Skills.
Roo Code 3.46.0 Release Notes (2026-01-30) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 2/4/26, 10:59 AM
Roo Code 3.46.1-3.46.2 Release Updates
@everyone Keeping the updates ROOLLING. Here are a few tweaks and bug fixes to continue improving your Roo experience. Sorry for the delay in the announcement!

QOL Improvements
Import settings during first-run setup: You can import a settings file directly from the welcome screen on a fresh install, before configuring a provider. (thanks emeraldcheshire!)
Change a skill’s mode from the Skills UI: You can set which mode a skill targets (including “Any mode”) using a dropdown, instead of moving files between mode folders manually. (thanks SannidhyaSah!)

Bug Fixes
More reliable tool-call history: Fixes an issue where mismatched tool-call IDs in conversation history could break tool execution.
MCP tool results can include images: Fixes an issue where MCP tools that return images (for example, Figma screenshots) could show up as “(No response)”. See Using MCP in Roo for details. (thanks Sniper199999!)
More reliable condensing with Bedrock via LiteLLM: Fixes an issue where conversation condensing could fail when the history contained tool-use and tool-result blocks.
Messages aren’t dropped during command execution: Fixes an issue where messages sent while a command was still running could be lost. They are now queued and delivered when the command finishes.
OpenRouter model list refresh respects your Base URL: Fixes an issue where refreshing the OpenRouter model list ignored a configured Base URL and always called openrouter.ai. See OpenRouter for details. (thanks sebastianlang84!)
More reliable task cancellation and queued-message handling: Fixes issues where canceling or closing tasks, or updating queued messages, could behave inconsistently between the VS Code extension and the CLI.

Misc Improvements
Quieter startup when no optional env file is present: Avoids noisy startup console output when the optional env file is not used.
Cleaner GitHub issue templates: Removes the “Feature Request” option from the issue template chooser so feature requests are directed to Discussions.

Provider Updates
Code indexing embedding model migration (Gemini): Keeps code indexing working by migrating away from a deprecated embedding model. See Gemini and Codebase Indexing.
Mistral provider migration to AI SDK: Improves consistency for streaming and tool handling while preserving Codestral support and custom base URLs. See Mistral.
SambaNova provider migration to AI SDK: Improves streaming, tool-call handling, and usage reporting. See SambaNova.
xAI provider migration to the dedicated AI SDK package: Improves consistency for streaming, tool calls, and usage reporting when using Grok models. See xAI.

See full release notes v3.46.1 | v3.46.2
Hannes

Role icon, Roo Code Dev — 2/5/26, 11:15 AM
@everyone OPUS 4.6 IS LIVE ON ROO CODE ROUTER!!!
Hannes

Role icon, Roo Code Dev — 2/5/26, 11:16 AM
Image
Hannes

Role icon, Roo Code Dev — 2/5/26, 3:09 PM
Roo Code 3.47.0 Release Updates
@everyone Claude Opus 4.6 WITH 1M CONTEXT and GPT-5.3-Codex (without ads! lol) are here!!

GPT-5.3-Codex - With your Chat GPT Plus/Pro subscription!
GPT-5.3-Codex is available right in Roo Code with your ChatGPT Plus or Pro subscription—no separate API billing. It posts new highs on SWE-Bench Pro (57%, across four programming languages) and Terminal-Bench 2.0 (77.3%, up from 64% for 5.2-Codex), while using fewer tokens than any prior model and running 25% faster.

You get the same 400K context window and 128K max output as 5.2-Codex, but the jump in sustained, multi-step engineering work is noticeable.

Claude Opus 4.6 - 1M CONTEXT IS HERE!!!
Opus 4.6 is available in Roo Code across Anthropic, AWS Bedrock, Vertex AI, OpenRouter, Roo Code Router, and Vercel AI Gateway. This is the first Opus-class model with a 1M token context window (beta)—enough to feed an entire large codebase into a single conversation. And it actually uses all that context: on the MRCR v2 needle-in-a-haystack benchmark it scores 76%, versus just 18.5% for Sonnet 4.5, which means the "context rot" problem—where earlier models fell apart as conversations grew—is largely solved.

Opus 4.6 also leads all frontier models on Terminal-Bench 2.0 (agentic coding), Humanity's Last Exam (multi-discipline reasoning), and GDPval-AA (knowledge work across finance and legal). It plans better, stays on task longer, and catches its own mistakes. (thanks @Peter Dave Hello )

QOL Improvements
Multi-mode Skills targeting: Skills can now target multiple modes at once using a modeSlugs frontmatter array, replacing the single mode field (which remains backward compatible). A new gear-icon modal in the Skills settings lets you pick which modes a skill applies to. The Slash Commands settings panel has also been redesigned for visual consistency.
AGENTS.local.md personal override files: You can now create an AGENTS.local.md file alongside AGENTS.md for personal agent-rule overrides that stay out of version control. The local file's content is appended under a distinct "Agent Rules Local" header, and both AGENTS.local.md and AGENT.local.md are automatically added to .gitignore.

Bug Fixes
Reasoning content preserved during AI SDK message conversion: Fixes an issue where reasoning/thinking content from models like DeepSeek deepseek-reasoner was dropped during message conversion, causing follow-up requests after tool calls to fail. Reasoning is now preserved as structured content through the conversion layer.
Environment details no longer break interleaved-thinking models: Fixes an issue where <environment_details> was appended as a standalone trailing text block, causing message-shape mismatches for models that use interleaved thinking. Details are now merged into the last existing text or tool-result block.

Provider Updates
Gemini and Vertex providers migrated to AI SDK: Streaming, tool calling, and structured outputs now use the shared Vercel AI SDK. Full feature parity retained.
Kimi K2.5 added to Fireworks: Adds Moonshot AI's Kimi K2.5 model to the Fireworks provider with a 262K context window, 16K max output, image support, and prompt caching.

Misc Improvements
Roo Code CLI v0.0.50 released: See the full release notes for details.

See full release notes v3.47.0
Roo Code 3.47.0 Release Notes (2026-02-05) | Roo Code Documentation
Roo Code 3.47.0 adds Claude Opus 4.6 across all providers, GPT-5.3-Codex for the OpenAI Codex provider, multi-mode Skills targeting, AGENTS.local.md personal overrides, and CLI improvements.
Roo Code 3.47.0 Release Notes (2026-02-05) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 2/17/26, 4:07 PM
Roo Code 3.48.0 Release Updates
@everyone Claude Sonnet 4.6 is here, plus API config locking, recursive subtask history trees, and significant cleanup across providers, browser automation, and skills.

Claude Sonnet 4.6 Support
Claude Sonnet 4.6 (claude-sonnet-4-6) is now available across Anthropic, Amazon Bedrock, Google Vertex, OpenRouter, and Vercel AI Gateway with a 1M token context window and 64K max output tokens. (thanks PeterDaveHello!)

API Config Lock
A new lock icon in the API config selector lets you pin your active provider and model across all mode switches in the current workspace. When locked, switching modes no longer swaps out your API configuration. Unlock at any time to restore normal per-mode behavior.

Recursive Subtask History Tree
The History view now renders the complete nested subtask hierarchy as an expandable tree. Each level of nesting can be expanded or collapsed independently, making it easy to navigate deep orchestrator task chains.

More Changes
search_and_replace renamed to edit with a flatter parameter model; backward-compatible alias kept
New disabledTools setting lets admins globally disable native tools via org/extension settings
Consecutive file ops in chat now collapse into grouped blocks with batch approve/deny
Nine providers retired (Cerebras, Chutes, DeepInfra, Doubao, Featherless, Groq, Hugging Face, IO Intelligence, Unbound); saved configs preserved
Built-in Puppeteer browser tool removed — migrate to Playwright MCP
Built-in skills removed — skills from global/workspace dirs only; find community skills at skills.sh
.roo/system-prompt-{mode} file override removed — migrate to custom instructions
GLM-5 added to Z.ai (~200K context, thinking mode)
CLI: stdin stream mode, auto-approve by default (breaking — use --require-approval to opt out), linux-arm64 support
18 bug fixes: orchestrator delegation reliability, chat history loss, condensation summary on resume, Windows checkpoint path mismatch, Gemini empty streams, and more

See full release notes v3.48.0
Roo Code 3.48.0 Release Notes (2026-02-17) | Roo Code Documentation
Roo Code 3.48.0 adds API config locking, Claude Sonnet 4.6 support, and a recursive subtask history tree, while removing browser automation, nine low-usage providers, built-in skills, and file-based system prompt overrides.
Roo Code 3.48.0 Release Notes (2026-02-17) | Roo Code Documentation
Hannes

Role icon, Roo Code Dev — 2/19/26, 1:33 AM
Roo Code 3.49.0 Release Updates
@everyone This release adds a file changes panel showing all edits made during a conversation, introduces per-workspace codebase indexing opt-in with a stop control, and fixes chat scroll and multi-window history reliability.

File Changes Panel
A new collapsible "X file(s) changed in this conversation" panel now appears below the chat messages in the task view. Each file that was edited and approved during the task appears in the panel — expand any entry to see its full diff without scrolling back through the conversation. Multiple edits to the same file are grouped by path, and the panel hides itself automatically when no files were changed. (thanks saneroen!)

Per-Workspace Indexing Control
Codebase indexing no longer silently auto-starts every time a workspace folder is opened. You now get explicit per-workspace control: a new toggle in the Codebase Indexing settings lets you enable indexing for the current folder, and that preference is saved across IDE restarts. A Stop Indexing button lets you cancel an in-progress scan at any time without losing partial results. In multi-root workspaces, enabling indexing for one folder no longer triggers indexing of every other folder. A bug where disabling the global "Enable Codebase Indexing" toggle during an active scan left the pulsing indicator running has also been fixed — the scan now stops immediately. (thanks JamesRobert20, thomas-mindruptive!)

Bug Fixes
Chat Scroll on Reopen: Fixes an issue where reopening an existing chat would not reliably scroll to the newest message, leaving users viewing older messages mid-conversation. Auto-scroll-to-bottom now consistently activates on chat open, the scroll-to-bottom button reliably re-anchors the view, and manually scrolling up to browse history properly pauses auto-follow without interference from nested scroll areas such as code blocks.
Multi-Window Task History Data Loss: Fixes silent task history data loss when Roo Code runs in two or more VS Code windows at the same time. Each task's history is now stored in its own file with cross-process file locking, so concurrent multi-window writes no longer overwrite each other. Migration from the previous storage format is automatic and backward-compatible.

See full release notes v3.49.0
