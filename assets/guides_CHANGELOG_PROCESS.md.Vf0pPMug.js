import { _ as a, o, c as l, ae as i } from "./chunks/framework.Dn7Y7LSn.js";
const p = JSON.parse(
    '{"title":"Changelog Process","description":"","frontmatter":{},"headers":[],"relativePath":"guides/CHANGELOG_PROCESS.md","filePath":"guides/CHANGELOG_PROCESS.md"}',
  ),
  t = { name: "guides/CHANGELOG_PROCESS.md" };
function r(c, e, n, s, d, h) {
  return (
    o(),
    l("div", null, [
      ...(e[0] ||
        (e[0] = [
          i(
            '<h1 id="changelog-process" tabindex="-1">Changelog Process <a class="header-anchor" href="#changelog-process" aria-label="Permalink to &quot;Changelog Process&quot;">​</a></h1><p>This project uses <a href="https://keepachangelog.com/en/1.1.0/" target="_blank" rel="noreferrer">Keep a Changelog</a>.</p><h2 id="scope" tabindex="-1">Scope <a class="header-anchor" href="#scope" aria-label="Permalink to &quot;Scope&quot;">​</a></h2><ul><li>Update <code>CHANGELOG.md</code> for every user-visible change.</li><li>Keep entries in <code>## [Unreleased]</code> until a release is cut.</li><li>Move unreleased entries into a versioned section at release time.</li></ul><h2 id="entry-types" tabindex="-1">Entry Types <a class="header-anchor" href="#entry-types" aria-label="Permalink to &quot;Entry Types&quot;">​</a></h2><p>Use only these headings under each release section:</p><ul><li><code>### Added</code></li><li><code>### Changed</code></li><li><code>### Deprecated</code></li><li><code>### Removed</code></li><li><code>### Fixed</code></li><li><code>### Security</code></li></ul><h2 id="authoring-rules" tabindex="-1">Authoring Rules <a class="header-anchor" href="#authoring-rules" aria-label="Permalink to &quot;Authoring Rules&quot;">​</a></h2><ul><li>Write concise, behavior-focused entries.</li><li>Prefer one bullet per meaningful change.</li><li>Include code/document path context when useful.</li><li>Do not include internal-only noise (format-only churn, refactors with no behavior change).</li></ul><h2 id="release-cut-checklist" tabindex="-1">Release Cut Checklist <a class="header-anchor" href="#release-cut-checklist" aria-label="Permalink to &quot;Release Cut Checklist&quot;">​</a></h2><ol><li>Confirm <code>## [Unreleased]</code> is accurate and complete.</li><li>Create a new section in <code>CHANGELOG.md</code>: <code>## [X.Y.Z] - YYYY-MM-DD</code>.</li><li>Move unreleased entries under that new section.</li><li>Add an empty <code>## [Unreleased]</code> section back at the top.</li><li>Verify README links still point to: <ul><li><code>CHANGELOG.md</code></li><li><code>docs/guides/CHANGELOG_PROCESS.md</code></li><li><code>docs/reference/CHANGELOG_ENTRY_TEMPLATE.md</code></li></ul></li></ol><h2 id="template" tabindex="-1">Template <a class="header-anchor" href="#template" aria-label="Permalink to &quot;Template&quot;">​</a></h2><p>Use <code>docs/reference/CHANGELOG_ENTRY_TEMPLATE.md</code> for new entries.</p>',
            13,
          ),
        ])),
    ])
  );
}
const m = a(t, [["render", r]]);
export { p as __pageData, m as default };
