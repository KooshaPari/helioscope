Ramlord
ramlord
Do Not Disturb

Saoud RizwanRole icon, Cline — 8/15/25, 9:22 AM
v3.25 @everyone
We're excited to share with you 3 new features that make Cline even better at tackling long, complex tasks autonomously:
Focus Chain: Automatically creates and maintains todo lists as you work with Cline, breaking down tasks into manageable steps with real-time progress tracking. You can even edit the todo list when needed!
Auto Compact: More intelligently manages conversation context when it hits context window by automatically summarizing older messages and preserving important details. This significantly helps Cline stay on track for long running sessions!
Deep Planning: New /deep-planning slash command transforms Cline into an architect who investigates your codebase, asks clarifying questions, and creates a comprehensive plan before writing any code.

Deep dive on the Focus Chain: https://cline.bot/blog/focus-attention-isnt-enough
Read the full details: https://cline.bot/blog/cline-v3-25
Announcement videos: https://x.com/cline/status/1956382811558142018
Changelog: https://github.com/cline/cline/blob/main/CHANGELOG.md 
Image
Nick BaumannRole icon, Cline — 8/19/25, 9:19 PM
v3.26 @everyone

New stealth model in Cline: "Sonic"

Designed for coding (262k context window) & free to use via the Cline provider, because your usage helps improve the model while it's in alpha.

Here's what else is new in v3.26:
Added Z AI as a new API provider with GLM-4.5 and GLM-4.5 Air models, offering competitive performance with cost-effective pricing especially for Chinese language tasks (Thanks @jues!)
Improved support for local models via LM Studio & Ollama providers, now showing accurately display context windows

Official announcement: https://x.com/cline/status/1958017077362704537
Changelog: https://github.com/cline/cline/blob/main/CHANGELOG.md

If you have a chance to leave us a review in the VS Code Marketplace, it'd be greatly appreciated! ❤️
Image
Nick BaumannRole icon, Cline — 8/20/25, 5:25 PM
We’ve partnered with Groq to celebrate prompt caching  on the fastest provider of Kimi-k2 by giving away free credits! @everyone

The first 500 people to use the promo code PROMPTCACHE20 will get $20 in Groq credits to use in Cline.

👉 Redeem here: console.groq.com/settings/billing/promo

Docs on creating your account and using Groq in Cline: docs.cline.bot/provider-config/groq 
Saoud RizwanRole icon, Cline — 8/28/25, 12:36 PM
v3.26.6 @everyone 

Grok Code Fast 1 is live in Cline and free through Tuesday!
The latest coding model from xAI, formerly known as "Sonic", is built for coding and is a contender for the best value LLM on the market.
It's priced at $.20/$1.50 per M input/output tokens ($0.02 cached tokens) via BYOK. From the Cline provider, use grok-code-fast-1 to try this model for free!

More about our experience with this model: https://cline.bot/blog/grok-code-fast
xAI's announcement: https://x.ai/news/grok-code-fast-1

We've also been working with LM Studio to improve our support for local models. 
We recommend using Qwen3-Coder-30B in Cline via LM Studio, and using our new "Compact Prompt" option, which is a condensed (by 90%) version of our standard system prompt -- this dramatically improves the performance of local models!

Read more about our improved local model support here: https://cline.bot/blog/local-models
Saoud RizwanRole icon, Cline — 9/3/25, 9:33 AM
@everyone Cline for JetBrains is Here! 🎉

This has been the #1 most requested feature on our GitHub, and we're so excited to finally get it in your hands for testing 🚀

We need your help!  Please download the alpha, test it out, and open GitHub issues for any problems you find. Your testing and feedback are crucial for making this rock-solid before we roll it out to everyone.
Please keep in mind this is an alpha release and may have some rough edges, but it's essentially a port of our VS Code extension and should function exactly the same way. If you like what you see, we'd love it if you left a review!

Get Cline on the JetBrains Marketplace now: https://plugins.jetbrains.com/plugin/28247-cline/versions/eap
Having trouble? We've put together detailed instructions here: https://docs.cline.bot/getting-started/installing-cline-jetbrains
Report issues on our GitHub: https://github.com/cline/cline/issues/new?template=bug_report.yml

(We also created the ⁠unknown channel for folks to discuss the new plugin, share feedback, and help the Cline team with ideas to improve the product!)
Image
Juan Pa 💖

Role icon, Cline — 9/4/25, 8:24 PM
@everyone 🚀 Kimi K2-0905 just landed in Cline!

This new release pushes coding agents forward with bigger memory, sharper coding skills, and faster execution.

K2-0905 comes with three major upgrades:
✨ 256k context window
⚡ Improved tool calling
🎨 Enhanced front-end development

Now live on Groq, Moonshot, OpenRouter, Fireworks, and Cline.
👉 Full details in the blog: https://cline.ghost.io/moonshot-kimi-k2-0905/
Juan Pa 💖

Role icon, Cline — 9/12/25, 8:58 AM
v3.28 @everyone

Extended Grok promotion time! Free access continues beyond today's deadline. 🎉

• Extended Grok Promotion: Free grok-code-fast-1 access extended (was supposed to end today!) - performing well on diff edits
• GPT-5 Optimization: Improved prompts for multi-step coding tasks - we're really bullish on this model
• Synchronized Task History: Real-time sync across all Cline instances
• Deep Planning Improvements: Better Windows/PowerShell support and dependency exclusion
• ESC Key Shortcut: Cancel operations consistently throughout the interface
• Enhanced Provider Support: Ollama custom prompts, Dify.ai integration, DeepSeek-V3.1 on SambaNova
• Fixes: LiteLLM caching with Anthropic, extended thinking tokens, AutoApprove overflow, slash command text preservation

Update now via VS Code Marketplace!

Read the full details
Changelog

If you have a chance to support us by leaving a review, we'd greatly appreciate it. Thank you! 
Saoud RizwanRole icon, Cline — 9/17/25, 2:56 PM
@everyone Cline for JetBrains is GA! 🎉
JetBrains has been #1 most requested feature, and we're so excited to finally have it live for everyone, officially! 🚀
Huge thanks to everyone who tested the EAP and gave us feedback -- your input was crucial for making this rock-solid. We couldn't have done it without you! 🙏
Full feature parity with VS Code: Get the full agentic capabilities of the Cline you know and love right in JetBrains, with native diff views and integrations with the workspace and terminal
Available across all JetBrains IDEs: IntelliJ IDEA, PyCharm, WebStorm, Android Studio, GoLand, PhpStorm, and more
Cross-compatibility: Switch working on your tasks between VS Code and JetBrains (+ CLI soon as well!)

Download now via JetBrains Marketplace! https://plugins.jetbrains.com/plugin/28247-cline

Read the full details: https://cline.bot/blog/cline-for-jetbrains
Head to the ⁠unknown channel to share your experience and help us make it even better!
If you have a chance to support us by leaving a review, we'd greatly appreciate it. Thank you! ❤️
Saoud RizwanRole icon, Cline — 9/19/25, 1:13 PM
Free stealth model just dropped @everyone 🥷 code-supernova in Cline provider:
200k context window
Multi-modal (i.e. image inputs)
"Built for agentic coding" applications like Cline
Access via the Cline provider in the latest update v3.30.0: cline/code-supernova
What are we building this weekend??
Read the full details: https://cline.bot/blog/code-supernova-stealth-model
Docs: https://docs.cline.bot/
If you have a chance to support us by leaving a review, we'd greatly appreciate it. Thank you!
Nick BaumannRole icon, Cline — 9/23/25, 4:03 PM
GPT-5-Codex now live in Cline @everyone

OpenAI's coding-optimized GPT-5 variant is now available in Cline. Key highlights:

• Adaptive reasoning -- uses 93% fewer reasoning tokens on simple tasks, 2x more thinking on complex debugging/refactoring
• Built for coding agents -- trained on real engineering workflows, more steerable and terse than Claude
• Works with thinking slider -- max out the thinking slider and let gpt-5-codex decide how many thinking tokens to use

$1.25/$10 per million tokens, 400K context. Select gpt-5-codex from the Cline provider.

Full details: https://cline.bot/blog/gpt-5-codex
Saoud RizwanRole icon, Cline — 9/25/25, 12:58 PM
v3.31 @everyone
Voice Mode: Experimental feature that must be enabled in settings for hands-free coding
UI Improvements: New task header and focus chain design to take up less space for a cleaner experience
YOLO Mode: Enable in settings to let Cline approve all actions and automatically switch between plan/act mode

Read the full details: https://cline.bot/blog/cline-v3-31
Changelog: https://github.com/cline/cline/blob/main/CHANGELOG.md

If you have a chance to support us by leaving a review, we'd greatly appreciate it. Thank you!
Nick BaumannRole icon, Cline — 9/26/25, 12:42 PM
Stealth model drop 🥷 @everyone 

code-supernova now with 1 million token context:

1 million token context window (up from 200k)
same multi-modal support (i.e. image inputs)
still free access continues during alpha

Access via the Cline provider: code-supernova-1-million

Read the update: https://cline.bot/blog/code-supernova-1-million

If you have a chance to support us by leaving a review, we'd greatly appreciate it. Thank you!
Image
Nick BaumannRole icon, Cline — 9/29/25, 10:05 AM
Live in Cline: Claude Sonnet 4.5 @everyone

200k or 1M context window & the same $3/$15 pricing.

What's different from Sonnet 4:
more terse, less narration
maintains state across sessions via context files
enhanced capability across long tasks

Update now wherever you use Cline to access the latest iteration of the Claude Sonnet!

Read the full details: https://cline.bot/blog/claude-sonnet-4-5

If you have a chance to support us by leaving a review, we'd greatly appreciate it. Thank you!
Image
Saoud RizwanRole icon, Cline — 10/16/25, 10:02 AM
@everyone  v3.33.0 brings Cline CLI and Subagents! 🚀

We're releasing the primitives for AI coding -- Cline as scriptable infrastructure you can build on!
You can now run Cline anywhere: in your terminal, in parallel across tasks, piped through your toolchain, or embedded in your own apps via our open gRPC API.

Install now: npm install -g cline

What's possible:
• Script Cline into your workflows: git diff | cline "review this" -o -F json
• Run parallel Clines to tackle multiple GitHub issues at once
• Build Slack bots, GitHub Actions, or mobile apps on top
• Start tasks in terminal, continue in your IDE with same state

+ this even allows Cline in VS Code or Jetbrains to call the Cline CLI tool as a subagent, to do subtasks and keep the main context window from getting polluted. For example if Cline needs to explore a directory for information, he can spawn a sub Cline CLI instance to do it instead of reading files and grep'ing needless tokens into its own context window.

Cline CLI and Subagents are in preview release and experimental! We need your help testing and would love to see what you do with it. Jump into ⁠unknown  to let us know how you're using these new features (we may share it on our socials!)

Demo of subagents: https://x.com/sdrzn/status/1979014975743005101
Get started: https://docs.cline.bot/cline-cli/overview
Deep dive from @Andrei Eternal : https://cline.bot/blog/cline-cli-my-undying-love-of-cline-core
Report issues: https://github.com/cline/cline/issues

(P.S. You can even run man cline to explore all the capabilities!) 
Nick BaumannRole icon, Cline — 10/23/25, 10:13 AM
v3.34 @everyone

Two big updates today!

Cline for Teams is now FREE through 2025: Unlimited users, JetBrains support, RBAC, centralized billing and more.  https://cline.bot/pricing

Better open source models with :exacto routing: You can now use ":exacto" versions of open-source models like GLM-4.6, Kimi-K2, and Qwen3-Coder in the Cline provider. These filtered endpoints deliver the best balance of cost, speed, accuracy and tool-calling reliability.

The :exacto integration is part of our commitment to making open source models truly production-ready in Cline. Provider quality matters as much as model weights - bad endpoints were breaking good models. Watch in the attached demo how the vanilla version of GLM-4.6 puts a tool call in the thinking tags, resulting in a failed tool call. 

Read @kevin@cline's deep dive on how we're closing the gap: https://cline.bot/blog/cline-our-commitment-to-open-source-zai-glm-4-6
Juan Pa 💖

Role icon, Cline — 10/27/25, 5:53 PM
@everyone v3.34.1 adds MiniMax-M2 provider support!

🆕 MiniMax-M2 is now available in Cline, a new open source model built specifically for agentic workflows and coding tasks.

What makes it interesting:
• Currently FREE to use (limited time) - test it out while the free tier is available
• Strong agentic performance - excels at planning and executing complex tool-calling tasks across Shell, Browser, Python, and MCP tools
• Open source - full model weights available on HuggingFace for self-hosting with vLLM or SGLang

⚠️ Model Update: Code Supernova is no longer available in our model list. We're sad to see it go -- it was great while it lasted! Shoutout to everyone that helped test the model.

📺 Upcoming Stream: Join us this Thursday at 10am PST with Ara as we dive deep into Cline's Architecture! We'll cover core architecture, technical implementation details, design decisions, and take your questions.

Event: https://discord.gg/pW9CgEEG?event=1432523644591931612

Thank you!
Nick BaumannRole icon, Cline — 10/31/25, 10:27 AM
v3.35 @everyone

Native Tool Calling: Migrated to native APIs for Claude 4+, Gemini 2.5, Grok 4/code-fast-1, and GPT-5. Fewer errors, parallel file operations, 15% token reduction due to reduced system prompt size. Try gpt-5-codex in Cline, it should feel a lot smoother!

Auto-Approve Menu: Redesigned from popup to expanding inline menu that doesn't block your view. Shows consolidated actions (e.g., "Read (all)" instead of both "Read" and "Read (all)"). Always-on by default.

MiniMax M2: Free until November 7 with newly added support for  interleaved thinking - multiple thinking blocks throughout each request instead of just at the start. Try MiniMax M2 again if you did a couple days ago, the added support for interleaved thinking improves its performance in Cline.

Update now wherever you use Cline!

Read the full details: https://cline.bot/blog/cline-v3-35
Changelog: https://github.com/cline/cline/blob/main/CHANGELOG.md 
Nick BaumannRole icon, Cline — 11/6/25, 11:52 AM
v3.36 @everyone

Hooks: Inject custom logic into Cline's workflow with executable scripts. Eight hook types (PreToolUse, PostToolUse, UserPromptSubmit, TaskStart, TaskResume, TaskCancel, APIRequestStart, APIResponseReceived) let you validate code, query databases, enforce standards, and integrate external tools. macOS/Linux for now, with Windows on the way.

Get started with hooks: https://docs.cline.bot/features/hooks
Hooks demo: https://www.youtube.com/watch?v=KCN1ig3i2CM

kimi-k2-thinking: Moonshot's latest & best-performing model is live in the Cline provider

Improved performance for open-source models, including a refined GLM-4.6 system prompt and support for <think> tags

Update now wherever you use Cline!

Read the full details: https://cline.bot/blog/cline-v3-36-hooks
See more improvements and bug fixes in the changelog: https://github.com/cline/cline/blob/main/CHANGELOG.md 
Juan Pa 💖

Role icon, Cline — 11/13/25, 2:21 PM
@everyone  CLine v3.37 is live with some big updates:

GPT-5.1 Support
OpenAI's latest model is now available with optimized prompts. Handles complex refactors and multi-file edits significantly better than previous models. Enable Native Tool Calling for best results.

Livestream - Today at 5pm with the MiniMax Team
We are doing a livestream between Cline and MiniMax about their latest M2 model. The session will cover the model’s capabilities, design approach, and recent developments in agentic AI systems. The @MiniMax Team is here!!. Leave your questions in ⁠unknown for us to cover on the livestream!

https://discord.gg/cline?event=1438602065143140492

Enterprise Configuration System
Deploy Cline at scale with org-wide settings:
Multi-provider setup with failover
MCP marketplace allowlists
Global rules/workflows for coding standards

Nous Research + Hermes 4
Added Nous as a native provider. Strong coding performance with tailored prompts.

Smarter Deep Planning
Enhanced prompts for GPT-5.1, Claude 4, and Gemini 2.5. More thorough exploration, cleaner execution plans.

Also: better context window handling, improved checkpoint reliability, and Linux dictation support.

Full details on GPT 5.1: https://cline.bot/blog/openais-gpt-5-1

— Juan 🫡 
Cline
Cline
2,855 Online
22,350 Members
Est. Aug 2024
Share prompts and projects, get tech support, and chat with friends!

Go to Server
Nick BaumannRole icon, Cline — 11/18/25, 10:35 AM
Gemini 3 Pro is live in Cline @everyone

The most intelligent model from Google Gemini marks a significant jump from 2.5 Pro and is outperforming Sonnet 4.5 in most benchmarks.

Read the official announcement here: https://blog.google/technology/developers/gemini-3-developers/

Try it in Cline today!
Image
Juan Pa 💖

Role icon, Cline — 11/19/25, 12:12 PM
Cline 3.38.0 is out now!
 Gemini 3 Pro Preview: Support for Google's new flagship model. It offers better structure retention and frontend capabilities that rival other models.

Voice Coding with AquaVoice Avalon: Integrated a new voice model tuned specifically for developer terminology ("Vercel config", "gpt-5.1") in our dictation feature, achieving 97.4% accuracy on coding jargon.

Fixes & Improvements:
Fixed AWS Bedrock context truncation issues
Optimized new_task usage,
Added better native tool calling validation.

Update now wherever you use Cline!

Read the full details: Cline 3.38.0 Release
Docs: https://docs.cline.bot/
If you have a chance to support us by leaving a review, we'd greatly appreciate it.

@everyone
Juan Pa 💖

Role icon, Cline — 11/26/25, 9:09 PM
Cline v3.38.3 is live!
@everyone Access Anthropic's most capable model now in Bedrock. Hooks that automate your entire post-task workflow. And a UI to control it all.

Expanded Hooks
New TaskComplete hook: run scripts automatically when tasks finish (auto-commit, trigger builds, notifications)
Hooks UI: manage hooks directly in the Rules & Workflows modal

Other updates
Claude Opus 4.5: Anthropic's most capable model is now available in Bedrock's global endpoint support.
Grok 4.1 + Grok Code :XAI's latest models added. Grok Code is specifically tuned for coding tasks.
Thinking Level Controls: Configure reasoning budget for Gemini 3.0 Pro, Vertex, and Anthropic models. Balance speed vs. thoroughness.
Native Tool Calling: Now enabled for Baseten and Kimi K2 models.

Bug Fixes
Windows terminal fixes, slash commands parsing, Vertex provider, reasoning issues with native tool calling.

Full changelog: https://github.com/cline/cline/releases/tag/v3.38.3

— Juan 🫡
Juan Pa 💖

Role icon, Cline — 12/2/25, 9:37 AM
The new Cline v3.39.1 release is here!
Hey @everyone! The new Cline v3.39.1 release is here with several QoL improvements, new stealth models and a new way to help review your code!

Explain Changes (/explain-changes) Code review has become one of the biggest bottlenecks in AI-assisted development. Cline can generate multi-file changes in seconds, but understanding what was done still takes time. We're introducing /explain-changes to help you review faster. After Cline completes a task, you can now get inline explanations that appear directly in your diff. No more jumping between the chat and your code to understand what changed. You can ask follow-up questions right in the comments, and it works on any git diff: commits, PRs, branches. Let us know what you think!

We wrote a deep dive on the thinking behind this feature and how to get the most out of it: Explain Changes Blog

New Stealth Model: Microwave We're happy to introduce Microwave—a new model available through the Cline provider. It has a 256k context window, is built specifically for agentic coding, and is free during alpha. It comes from a lab you know and will be excited to hear from. We've been testing it internally and have been impressed with the results.

cline:cline/microwave

Other New Features

Use /commands anywhere in your message, not just at the start
Tabbed model picker makes it easier to find Recommended or Free models without scrolling
View and edit .clinerules from remote repos without leaving your editor
Sticky headers let you jump back to any prompt in long conversations instantly

Bug Fixes & QoL

Fixed task opening issues with Cline accounts
Smarter LiteLLM validation (checks for API key before fetching models)
Better context handling with auto-compaction improvements
Cleaner auto-approve menu UI

New Contributors

luna-asksage made their first contribution in #7329
dltechy made their first contribution in #7630
reneehuang1 made their first contribution in #7589
Richardmsbr made their first contribution in #7776

Update now in your favorite IDE!

– Juan 🫡
Nick BaumannRole icon, Cline — 12/11/25, 2:20 PM
Cline v3.41 is live!
@everyone GPT-5.2, Devstral 2, and a redesigned model picker are here.

GPT-5.2
OpenAI's latest frontier model. 80% on SWE-bench Verified, major improvements in tool calling and long-context reasoning. Enable "thinking" to use GPT-5.2 Thinking.

Devstral 2
The stealth model "Microwave" revealed: Devstral 2 from Mistral AI. 72.2% on SWE-bench Verified, 7x more cost-efficient than Sonnet, free during launch.
mistralai/devstral-2512

Faster model switching
New model picker by the chat: see only your configured providers, toggle Plan/Act with a sparkle icon, enable thinking with one click.

Other updates
Codex Responses API: gpt-5.1-codex and gpt-5.1-codex-max now support Responses API (uenable Native Tool Calling)
Amazon Nova 2 Lite now available.
DeepSeek 3.2 added to native tool calling allow list.

Bug Fixes: Non-blocking checkpoint commits, Gemini Vertex thinking errors, Ollama streaming abort.

Full blog: https://cline.bot/blog/cline-v3-41-adds-gpt-5-2-devstral-2-ergonomic-model-switching

Changelog: https://github.com/cline/cline/blob/main/CHANGELOG.md

-Nick 🫡 
Juan Pa 💖

Role icon, Cline — 1/9/26, 6:20 PM
Cline 3.48.0 is out
Hey @everyone, this release adds Skills compatibility and websearch tooling. 

Added
• Skills compatibility - Package domain expertise into modular instructions that Cline loads only when relevant
• Websearch and web fetch tools for Cline provider users - Search the web and retrieve content as text (faster than browser screenshots)
• Gemini thinking support
• Katcoder model added to model list
• zai-glm-4.7 available on Cerebras

QoL
• Vercel AI Gateway model refresh and improved reasoning support

Fixed
• Regression from 3.47.0 affecting diff view and document truncation
• Extension crash when using context menu selector

Read more: https://cline.bot/blog/cline-3-48-0-skills-and-websearch-make-cline-smarter

---

Discord cleanup

We're reducing the number of channels to make it easier to find important information. Most channels are moving to the Public Archive category as read-only for reference. This should help keep discussions focused and make it simpler to navigate the server.
Image
Cline
Cline 3.48.0: Skills and websearch make Cline smarter - Cline Blog
Cline 3.48.0 adds Skills compatibility. If you've already built Skills, you can now use them in Cline. This release also adds websearch tooling through the Cline provider, giving Cline access to real-time information when you need it.


Skills

Think about how you'd onboard a new team member. You wouldn't dump every document on them at once. Yo...
Cline 3.48.0: Skills and websearch make Cline smarter - Cline Blog
Juan Pa 💖

Role icon, Cline — 1/28/26, 12:29 PM
Cline 3.55.0 Release
Hey @everyone :PikachuHi: ! 
A few updates on Cline 3.55.0. Open models are catching up to closed-source options. This release adds two worth trying: a free US-built model and an open-source benchmark leader.

Arcee Trinity Large is free, 
US-built, and Apache 2.0 licensed. It's a 400B parameter MoE model (13B active at inference) with 128K context. Benchmarks: MMLU Pro 82, GPQA Diamonds 75. Good for general coding, refactoring, and working with large codebases without worrying about API costs.

:youtube:  https://www.youtube.com/watch?v=bIhxLegqqcw

Kimi K2.5 
Open source and competitive with closed-source options. 1T parameter MoE, 256K context. Scores 76.8% on SWE-bench and beats Opus 4.5 on Humanity's Last Exam (50.2%). Particularly strong for visual coding: drop a screenshot and get working UI code with layout, animations, and interactions. It can also inspect its own output and self-correct.

Also a reminder in this release: ChatGPT Plus/Pro subscribers can use GPT-5 models in Cline via OAuth (no API key needed and no per-token costs related). Grok Code Fast 1 and Devstral free promotions have ended :ClineSad: .

See you around!

Full details: https://cline.bot/blog/cline-3-55-0-arcee-trinity-and-kimi-k2-5-now-in-cline 
YouTube
Arcee AI
Code Reviews at Scale: Trinity Large Preview + Cline + OpenRouter
Image
Cline
Cline 3.55.0: Arcee Trinity and Kimi K2.5 now in Cline - Cline Blog
Cline 3.55 adds two open models worth paying attention to. Arcee Trinity Large is free, US-built, and licensed under Apache 2.0. Kimi K2.5 is an open-source model that outperforms Opus 4.5 on certain benchmarks.
Cline 3.55.0: Arcee Trinity and Kimi K2.5 now in Cline - Cline Blog
Juan Pa 💖

Role icon, Cline — 2/3/26, 11:28 AM
CLI 2.0 is out!
Hey @everyone  👋
The team has been working hard on Cline CLI and we're excited to share some updates:

Redesigned UI: We rebuilt the CLI from the ground up to make it look and feel like the Cline you're used to in VS Code, making it easier to transition from the IDE to the terminal. Plan/Act modes, easy Auto-approve toggle, and powerful slash commands.
Parallel Agents: Run multiple Cline instances at the same time. Different agents working on different parts of your project, while you stay aware of where each one is at and when they need your input.
Headless Pipelines: Cline CLI is now fully scriptable. Run it as part of your build and deployment workflows, skip permission prompts for full automation, and get structured output for your scripts.
ACP Support: Use Cline in Zed, Neovim, Emacs, and more. Your coding agent shouldn't lock you into a specific editor.
Free Kimi K2.5 Access (and more to come…) We added support for Kimi K2.5, Moonshot's open-source model. Free access is temporary, so make the most of it while you can.

📄 Docs: https://docs.cline.bot/cline-cli/overview

Let us know what you think!

Check our our blog for more on our latest release. If you want to hop right in,  npm install -g cline
Juan Pa 💖

Role icon, Cline — 2/5/26, 4:39 PM
Claude Opus 4.6 is now available in Cline 3.57 !
@everyone Anthropic just dropped their most intelligent model yet and it's ready to use in Cline v3.57.

What's new

Opus 4.6 is a big jump in reasoning, long context, and agentic task execution. Some highlights from the system card:

80.8% on SWE-Bench Verified
65.4% on Terminal-Bench 2.0 (state of the art)
68.8% on ARC-AGI-2 (nearly double from Opus 4.5)
1M token context window (first Opus model with long context)

How it feels

This model gets what you mean. You can be vague, half-explain what you want, and it fills in the gaps correctly. The responses are clean too. No fluff, no walls of text. You get what you need to stay on top of the task.

It also has better taste for UI and design work. If you're generating frontends, the output is more cohesive and modern.

Where it really shines: the CLI

Autonomous decision-making is a step above. Enable auto-approve, give it a goal, and it breaks the work down without babysitting. Shorter prompts, better results. This is the model you want for running things hands-off.

One thing to know

Opus 4.6 takes longer to think on complex tasks. That's the deeper reasoning doing its job, not the model being stuck. For quick iterative work, Sonnet is still the move.

How to use it

Select claude-opus-4-6 from the model picker.
Works in your terminal, JetBrains, VS Code, Zed, Neovim, and Emacs.

Let us know how it goes. Drop your feedback in ⁠ai-models . 
Image
Juan Pa 💖

Role icon, Cline — 2/12/26, 12:16 AM
Cline v3.58.0: native subagents, GLM-5 support, and hands-off task completion
Hey @everyone
The team has been working hard on a few new features, models and fixes for Cline. Here is what's new:

 Native Subagents (experimental)
Cline can now spin up parallel sub-tasks that run independently with their own context. Currently subagents can read files, search codebases, and use skills. Multiple agents exploring your project at the same time instead of going one task at a time. Available in VSCode and the CLI.

GLM-5 support
ZAI's new flagship model (744B params, 40B active) is now available in Cline. Built for coding and agentic workflows, uses DeepSeek Sparse Attention for efficient long-context handling, and a new async RL infrastructure called Slime for better post-training. Best-in-class among open-source models on reasoning, coding, and agentic benchmarks. Expected MIT license release.

Hands-off task completion
• Auto-approval for attempt_completion so tasks can finish without you approving every step
• Double-check completion (experimental) adds a verification step before completing
• YOLO mode now auto-approves MCP tools too

CLI improvements
• --thinking flag accepts a custom token budget
• --max-consecutive-mistakes stops tasks before they spiral
• More shortcuts in help output

Provider & enterprise
• Amazon Bedrock parallel tool calling
• Opus 4.6 with 1M context on Vertex and Claude Code providers
• MCP server management improvements (synced deletion, header support)
• Remote config UI with test buttons
• Bundled endpoints.json for offline support

Fixes
Terminal commands now surface exit codes, tuned timeout strategy for long-running tasks, reasoning behavior parity restored, CI environment support for headless runs, OAuth callback fixes, input text persists on remount, and more.

Full changelog: https://github.com/cline/cline/releases/tag/v3.58.0. Let us know how it goes with the new models and subagents!
Image
Image
Juan Pa 💖

Role icon, Cline — 2/17/26, 3:38 PM
Claude Sonnet 4.6 is now available in Cline v3.64.0 and its free until Feb 18 at noon PST
Hey @everyone!
Anthropic just shipped their best Sonnet model and it's live in Cline right now. Update to v3.64.0 to access it. Here's what stood out in early testing:

🔹 Speed: noticeably faster
🔹 Transparency:  gives you good context on what it's working on as it goes
🔹 Library integration: pulls in the right libraries and actually integrates them properly into your project
🔹 Subagents: fast, precise, and at Sonnet pricing it makes parallel workflows way more practical

Anthropic's numbers: devs preferred it over Sonnet 4.5 ~70% of the time and over Opus 4.5 59% of the time. 1M token context window. Same pricing as Sonnet 4.5 after the free period.

Try it out and drop your impressions

---

📋 Usability study: We're looking for volunteers for a quick interview on the CLI and subagents. $50 USD in credits for anyone who participates. Schedule a call here: https://calendar.app.google/91ReAvjDkHa3VVBw8

-- Juan Pa 🫡 
Image

