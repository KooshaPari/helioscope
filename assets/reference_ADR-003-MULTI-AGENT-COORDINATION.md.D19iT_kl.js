import {
  _ as a,
  o as i,
  c as t,
  ae as l,
} from "./chunks/framework.Dn7Y7LSn.js";
const h = JSON.parse(
    '{"title":"ADR-003: Multi-Agent Coordination Strategy","description":"","frontmatter":{},"headers":[],"relativePath":"reference/ADR-003-MULTI-AGENT-COORDINATION.md","filePath":"reference/ADR-003-MULTI-AGENT-COORDINATION.md"}',
  ),
  n = { name: "reference/ADR-003-MULTI-AGENT-COORDINATION.md" };
function r(s, e, o, c, u, p) {
  return (
    i(),
    t("div", null, [
      ...(e[0] ||
        (e[0] = [
          l(
            `<h1 id="adr-003-multi-agent-coordination-strategy" tabindex="-1">ADR-003: Multi-Agent Coordination Strategy <a class="header-anchor" href="#adr-003-multi-agent-coordination-strategy" aria-label="Permalink to &quot;ADR-003: Multi-Agent Coordination Strategy&quot;">​</a></h1><p><strong>Date</strong>: 2026-02-23<br><strong>Status</strong>: Proposed<br><strong>Author</strong>: heliosHarness Team</p><h2 id="context" tabindex="-1">Context <a class="header-anchor" href="#context" aria-label="Permalink to &quot;Context&quot;">​</a></h2><p>When multiple teammate agents work concurrently, we need to:</p><ol><li>Prevent Git lock conflicts</li><li>Isolate resource consumption</li><li>Handle conflicts gracefully</li><li>Ensure result aggregation</li></ol><h2 id="decision" tabindex="-1">Decision <a class="header-anchor" href="#decision" aria-label="Permalink to &quot;Decision&quot;">​</a></h2><p>We will implement a <strong>Layered Coordination Strategy</strong>:</p><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>┌─────────────────────────────────────────────┐</span></span>
<span class="line"><span>│            High-Level Coordination           │</span></span>
<span class="line"><span>│  (Task Queue, Priority, Backpressure)      │</span></span>
<span class="line"><span>├─────────────────────────────────────────────┤</span></span>
<span class="line"><span>│            Mid-Level Protection             │</span></span>
<span class="line"><span>│  (Circuit Breaker, Bulkhead)                │</span></span>
<span class="line"><span>├─────────────────────────────────────────────┤</span></span>
<span class="line"><span>│            Low-Level Isolation             │</span></span>
<span class="line"><span>│  (Git Worktrees, Private Index, Merge)     │</span></span>
<span class="line"><span>└─────────────────────────────────────────────┘</span></span></code></pre></div><h3 id="_1-task-queue-high-level" tabindex="-1">1. Task Queue (High-Level) <a class="header-anchor" href="#_1-task-queue-high-level" aria-label="Permalink to &quot;1. Task Queue (High-Level)&quot;">​</a></h3><ul><li>Priority-based queue (CRITICAL &gt; HIGH &gt; NORMAL &gt; LOW)</li><li>Backpressure when queue &gt; 75% full</li><li>Graceful degradation: reject lower priority first</li></ul><h3 id="_2-circuit-breaker-bulkhead-mid-level" tabindex="-1">2. Circuit Breaker &amp; Bulkhead (Mid-Level) <a class="header-anchor" href="#_2-circuit-breaker-bulkhead-mid-level" aria-label="Permalink to &quot;2. Circuit Breaker &amp; Bulkhead (Mid-Level)&quot;">​</a></h3><p><strong>Circuit Breaker</strong>:</p><ul><li>Track failures per teammate type</li><li>States: CLOSED → OPEN → HALF_OPEN → CLOSED</li><li>Configurable: failure_threshold=5, timeout=60s</li></ul><p><strong>Bulkhead</strong>:</p><ul><li>Separate pools per resource type (CPU, I/O, DB)</li><li>Semaphore-based concurrency control</li><li>Prevents one agent from monopolizing resources</li></ul><h3 id="_3-git-isolation-low-level" tabindex="-1">3. Git Isolation (Low-Level) <a class="header-anchor" href="#_3-git-isolation-low-level" aria-label="Permalink to &quot;3. Git Isolation (Low-Level)&quot;">​</a></h3><p><strong>Option A: Private GIT_INDEX_FILE</strong></p><ul><li>Each agent uses separate index file</li><li><code>GIT_INDEX_FILE=&lt;agent&gt;.index codex exec ...</code></li><li>Requires CAS reference updates</li></ul><p><strong>Option B: Git Worktrees</strong></p><ul><li>Full directory isolation per agent</li><li><code>git worktree add &lt;path&gt; &lt;branch&gt;</code></li><li>Best for heavy modifications</li></ul><p><strong>Option C: Mergiraf</strong></p><ul><li>AST-aware conflict resolution</li><li>Better than text-based 3-way merge</li><li>For when both modify same file</li></ul><h2 id="consequences" tabindex="-1">Consequences <a class="header-anchor" href="#consequences" aria-label="Permalink to &quot;Consequences&quot;">​</a></h2><h3 id="positive" tabindex="-1">Positive <a class="header-anchor" href="#positive" aria-label="Permalink to &quot;Positive&quot;">​</a></h3><ul><li>Multiple safety nets prevent cascading failures</li><li>Flexible isolation levels (choose appropriate)</li><li>Proven patterns from thegent</li></ul><h3 id="negative" tabindex="-1">Negative <a class="header-anchor" href="#negative" aria-label="Permalink to &quot;Negative&quot;">​</a></h3><ul><li>Complexity increases with each layer</li><li>Must configure for workload</li></ul><h3 id="neutral" tabindex="-1">Neutral <a class="header-anchor" href="#neutral" aria-label="Permalink to &quot;Neutral&quot;">​</a></h3><ul><li>Can start simple, add layers as needed</li></ul><h2 id="references" tabindex="-1">References <a class="header-anchor" href="#references" aria-label="Permalink to &quot;References&quot;">​</a></h2><ul><li>SWARM_PROCESS_AUTOMATION_DEEP_RESEARCH.md (thegent)</li><li>DYNAMIC_SCALING_AND_SELF_HEALING_PATTERNS.md (thegent)</li></ul>`,
            31,
          ),
        ])),
    ])
  );
}
const g = a(n, [["render", r]]);
export { h as __pageData, g as default };
