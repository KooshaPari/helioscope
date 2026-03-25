import {
  _ as a,
  o as r,
  c as o,
  j as e,
  a as s,
} from "./chunks/framework.Dn7Y7LSn.js";
const m = JSON.parse(
    '{"title":"Tutorials","description":"","frontmatter":{},"headers":[],"relativePath":"tutorials/index.md","filePath":"tutorials/index.md"}',
  ),
  i = { name: "tutorials/index.md" };
function n(d, t, l, u, c, p) {
  return (
    r(),
    o("div", null, [
      ...(t[0] ||
        (t[0] = [
          e(
            "h1",
            { id: "tutorials", tabindex: "-1" },
            [
              s("Tutorials "),
              e(
                "a",
                {
                  "class": "header-anchor",
                  "href": "#tutorials",
                  "aria-label": 'Permalink to "Tutorials"',
                },
                "​",
              ),
            ],
            -1,
          ),
          e("p", null, "Add guided onboarding and first-run flows here.", -1),
        ])),
    ])
  );
}
const x = a(i, [["render", n]]);
export { m as __pageData, x as default };
