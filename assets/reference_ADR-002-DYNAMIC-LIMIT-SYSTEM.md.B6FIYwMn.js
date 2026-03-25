import {
  _ as a,
  o as t,
  c as i,
  ae as s,
} from "./chunks/framework.Dn7Y7LSn.js";
const p = JSON.parse(
    '{"title":"ADR-002: Dynamic Thread Limit System","description":"","frontmatter":{},"headers":[],"relativePath":"reference/ADR-002-DYNAMIC-LIMIT-SYSTEM.md","filePath":"reference/ADR-002-DYNAMIC-LIMIT-SYSTEM.md"}',
  ),
  n = { name: "reference/ADR-002-DYNAMIC-LIMIT-SYSTEM.md" };
function r(l, e, o, d, c, h) {
  return (
    t(),
    i("div", null, [
      ...(e[0] ||
        (e[0] = [
          s(
            `<h1 id="adr-002-dynamic-thread-limit-system" tabindex="-1">ADR-002: Dynamic Thread Limit System <a class="header-anchor" href="#adr-002-dynamic-thread-limit-system" aria-label="Permalink to &quot;ADR-002: Dynamic Thread Limit System&quot;">​</a></h1><p><strong>Date</strong>: 2026-02-23<br><strong>Status</strong>: Proposed<br><strong>Author</strong>: heliosHarness Team</p><h2 id="context" tabindex="-1">Context <a class="header-anchor" href="#context" aria-label="Permalink to &quot;Context&quot;">​</a></h2><p>The current implementation uses fixed thread/concurrency limits. This leads to:</p><ul><li>Under-utilization when resources available</li><li>Overload when fixed limit too high</li><li>No adaptation to workload changes</li><li>Manual tuning required</li></ul><p>We need a dynamic system that:</p><ul><li>Scales with available resources (CPU, memory, FD, load average)</li><li>Uses hysteresis to prevent thrashing</li><li>Provides safety buffers (minimum + discretionary)</li><li>Supports prediction for growth</li></ul><h2 id="decision" tabindex="-1">Decision <a class="header-anchor" href="#decision" aria-label="Permalink to &quot;Decision&quot;">​</a></h2><p>We will implement a <strong>Resource-Based Dynamic Limit System</strong> with hysteresis control:</p><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>Resources ──▶ Sampling ──▶ Hysteresis ──▶ Limit Calc ──▶ Executor</span></span>
<span class="line"><span>(CPU, Mem,    (1-5 sec)   Controller    (5% min +    (Semaphore)</span></span>
<span class="line"><span> FD, Load)                 (upper/lower)  15% disc)</span></span></code></pre></div><h3 id="configuration" tabindex="-1">Configuration <a class="header-anchor" href="#configuration" aria-label="Permalink to &quot;Configuration&quot;">​</a></h3><table tabindex="0"><thead><tr><th>Parameter</th><th>Default</th><th>Description</th></tr></thead><tbody><tr><td>min_buffer</td><td>0.05</td><td>5% hard limit (prevents crashes)</td></tr><tr><td>discretionary_buffer</td><td>0.15</td><td>15% soft limit (allows scaling)</td></tr><tr><td>hysteresis_upper</td><td>0.80</td><td>Scale up threshold</td></tr><tr><td>hysteresis_lower</td><td>0.60</td><td>Scale down threshold</td></tr><tr><td>dwell_time</td><td>30s</td><td>Time to wait before changing limit</td></tr></tbody></table><h3 id="limit-calculation" tabindex="-1">Limit Calculation <a class="header-anchor" href="#limit-calculation" aria-label="Permalink to &quot;Limit Calculation&quot;">​</a></h3><div class="language- vp-adaptive-theme"><button title="Copy Code" class="copy"></button><span class="lang"></span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span>effective_limit = min(</span></span>
<span class="line"><span>    running_count,</span></span>
<span class="line"><span>    floor(available_resources * (1 - min_buffer)),</span></span>
<span class="line"><span>    floor(available_resources * (1 - discretionary_buffer))</span></span>
<span class="line"><span>)</span></span></code></pre></div><h2 id="consequences" tabindex="-1">Consequences <a class="header-anchor" href="#consequences" aria-label="Permalink to &quot;Consequences&quot;">​</a></h2><h3 id="positive" tabindex="-1">Positive <a class="header-anchor" href="#positive" aria-label="Permalink to &quot;Positive&quot;">​</a></h3><ul><li>Automatically scales with system capacity</li><li>Prevents crashes with minimum buffer</li><li>Hysteresis prevents thrashing</li><li>No manual tuning needed</li><li>Proven pattern from thegent (load_based_limits)</li></ul><h3 id="negative" tabindex="-1">Negative <a class="header-anchor" href="#negative" aria-label="Permalink to &quot;Negative&quot;">​</a></h3><ul><li>Slightly more complex than fixed limits</li><li>Requires reliable resource sampling</li><li>May need tuning for specific workloads</li></ul><h3 id="neutral" tabindex="-1">Neutral <a class="header-anchor" href="#neutral" aria-label="Permalink to &quot;Neutral&quot;">​</a></h3><ul><li>Platform-specific resource sampling (psutil / native)</li></ul><h2 id="alternatives-considered" tabindex="-1">Alternatives Considered <a class="header-anchor" href="#alternatives-considered" aria-label="Permalink to &quot;Alternatives Considered&quot;">​</a></h2><ol><li><strong>Fixed limit with manual adjustment</strong> - Current approach, not adaptive</li><li><strong>Purely reactive (queue-based)</strong> - Doesn&#39;t prevent overload</li><li><strong>ML-based prediction</strong> - Over-engineered for current needs</li></ol><h2 id="implementation-notes" tabindex="-1">Implementation Notes <a class="header-anchor" href="#implementation-notes" aria-label="Permalink to &quot;Implementation Notes&quot;">​</a></h2><ul><li>Use <code>psutil</code> for cross-platform resource sampling</li><li>Implement <code>thegent-resources</code> Rust binary for performance (optional)</li><li>Sample resources every 1-5 seconds</li><li>Apply hysteresis before changing limit</li></ul><h2 id="references" tabindex="-1">References <a class="header-anchor" href="#references" aria-label="Permalink to &quot;References&quot;">​</a></h2><ul><li>DYNAMIC_SCALING_AND_SELF_HEALING_PATTERNS.md (thegent)</li><li>load_based_limits_api.md (thegent)</li></ul>`,
            27,
          ),
        ])),
    ])
  );
}
const m = a(n, [["render", r]]);
export { p as __pageData, m as default };
