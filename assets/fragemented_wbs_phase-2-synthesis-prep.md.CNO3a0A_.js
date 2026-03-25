import {
  _ as a,
  o as s,
  c as t,
  ae as i,
} from "./chunks/framework.Dn7Y7LSn.js";
const u = JSON.parse(
    '{"title":"Phase-2 Synthesis Prep","description":"","frontmatter":{},"headers":[],"relativePath":"fragemented/wbs/phase-2-synthesis-prep.md","filePath":"fragemented/wbs/phase-2-synthesis-prep.md"}',
  ),
  o = { name: "fragemented/wbs/phase-2-synthesis-prep.md" };
function r(c, e, n, d, l, h) {
  return (
    s(),
    t("div", null, [
      ...(e[0] ||
        (e[0] = [
          i(
            '<h1 id="phase-2-synthesis-prep" tabindex="-1">Phase-2 Synthesis Prep <a class="header-anchor" href="#phase-2-synthesis-prep" aria-label="Permalink to &quot;Phase-2 Synthesis Prep&quot;">​</a></h1><h2 id="output-artifacts" tabindex="-1">Output artifacts <a class="header-anchor" href="#output-artifacts" aria-label="Permalink to &quot;Output artifacts&quot;">​</a></h2><ul><li><code>wbs/phase-2.json</code> (this file was generated)</li><li><code>research/phase-2-reports/agent-a-core-repo-harden.md</code></li><li><code>research/phase-2-reports/agent-b-candidate-expansion.md</code></li><li><code>research/phase-2-reports/agent-c-governance-strictness.md</code></li><li><code>research/phase-2-reports/agent-d-harness-architecture.md</code></li><li><code>research/phase-2-reports/agent-e-validation-automation.md</code></li><li><code>research/phase-2-reports/agent-f-closeout-delivery.md</code></li><li><code>artifacts/phase-2-closeout.md</code></li></ul><h2 id="phase-2-execution-guidance" tabindex="-1">Phase-2 execution guidance <a class="header-anchor" href="#phase-2-execution-guidance" aria-label="Permalink to &quot;Phase-2 execution guidance&quot;">​</a></h2><ul><li>Treat the new <code>phase-2.json</code> as the single source of truth for task IDs and dependencies.</li><li>Maintain non-blocking semantics except where <code>BLOCK</code>-class governance decisions are explicit in task evidence.</li><li>Start by validating lane manifests and clone health (lane G dependencies).</li></ul><h2 id="validation-checklist" tabindex="-1">Validation checklist <a class="header-anchor" href="#validation-checklist" aria-label="Permalink to &quot;Validation checklist&quot;">​</a></h2><ul><li>Clone readiness: <code>goose</code> now copied from <code>/API/research/goose</code> due prior remote clone instability.</li><li>Gate for next phase: all <code>A</code>/<code>B</code>/<code>C</code> evidence files created and status <code>in_progress</code>/<code>pending</code> as planned.</li></ul>',
            7,
          ),
        ])),
    ])
  );
}
const f = a(o, [["render", r]]);
export { u as __pageData, f as default };
