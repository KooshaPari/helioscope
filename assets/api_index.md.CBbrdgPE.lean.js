import {
  _ as t,
  o as n,
  c as r,
  j as e,
  a as i,
} from "./chunks/framework.Dn7Y7LSn.js";
const x = JSON.parse(
    '{"title":"API","description":"","frontmatter":{},"headers":[],"relativePath":"api/index.md","filePath":"api/index.md"}',
  ),
  s = { name: "api/index.md" };
function o(d, a, c, l, p, m) {
  return (
    n(),
    r("div", null, [
      ...(a[0] ||
        (a[0] = [
          e(
            "h1",
            { id: "api", tabindex: "-1" },
            [
              i("API "),
              e(
                "a",
                {
                  "class": "header-anchor",
                  "href": "#api",
                  "aria-label": 'Permalink to "API"',
                },
                "​",
              ),
            ],
            -1,
          ),
          e("p", null, "Document internal and external API surfaces.", -1),
        ])),
    ])
  );
}
const _ = t(s, [["render", o]]);
export { x as __pageData, _ as default };
