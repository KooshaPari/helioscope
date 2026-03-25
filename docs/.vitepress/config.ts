import { defineConfig } from 'vitepress'

// Base path for standalone deployment. Override via DOCS_BASE env var
// (e.g. '/heliosCLI/' for GitHub Pages project-site deployment).
const docsBase = (process.env.DOCS_BASE ?? '/') as `/${string}/` | '/'

// Supported locales: en, zh-CN, zh-TW, fa, fa-Latn
const locales = {
  root: { label: "English", lang: "en", title: 'heliosHarness', description: 'Harness docs' },
  "zh-CN": { label: "\u7b80\u4f53\u4e2d\u6587", lang: "zh-CN", title: 'heliosHarness', description: 'Harness \u6587\u6863' },
  "zh-TW": { label: "\u7e41\u9ad4\u4e2d\u6587", lang: "zh-TW", title: 'heliosHarness', description: 'Harness \u6587\u6888' },
  fa: { label: "\u0641\u0627\u0631\u0633\u06cc", lang: "fa", title: 'heliosHarness', description: '\u0645\u0633\u062a\u0646\u062f\u0627\u062a Harness' },
  "fa-Latn": { label: "Pinglish", lang: "fa-Latn", title: 'heliosHarness', description: 'Harness docs (Latin)' }
}

export default defineConfig({
  title: 'heliosHarness',
  description: 'Harness docs',
  base: docsBase,
  srcExclude: ['fragemented/research/**'],
  locales,
  themeConfig: {
    nav: [
      { text: 'Start Here', link: '/index' },
      { text: 'Tutorials', link: '/tutorials/' },
      { text: 'How-to', link: '/how-to/' },
      { text: 'Explanation', link: '/explanation/' },
      { text: 'Operations', link: '/operations/' },
      { text: 'API', link: '/api/' },
      {
        text: "Language",
        items: [
          { text: "English", link: "/" },
          { text: "\u7b80\u4f53\u4e2d\u6587", link: "/zh-CN/" },
          { text: "\u7e41\u9ad4\u4e2d\u6587", link: "/zh-TW/" },
          { text: "\u0641\u0627\u0631\u0633\u06cc", link: "/fa/" },
          { text: "Pinglish", link: "/fa-Latn/" }
        ]
      }
    ]
  }
})
