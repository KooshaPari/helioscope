import { defineConfig } from 'vitepress'
import { resolveDocsBase } from "../../../docs-hub/.vitepress/base.config"

const docsBase = resolveDocsBase()

// Supported locales: en, zh-CN, zh-TW, fa, fa-Latn
const locales = {
  root: { label: "English", lang: "en", title: 'heliosHarness', description: 'Harness docs' },
  "zh-CN": { label: "简体中文", lang: "zh-CN", title: 'heliosHarness', description: 'Harness 文档' },
  "zh-TW": { label: "繁體中文", lang: "zh-TW", title: 'heliosHarness', description: 'Harness 文檔' },
  fa: { label: "فارسی", lang: "fa", title: 'heliosHarness', description: 'مستندات Harness' },
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
        text: "🌐 Language",
        items: [
          { text: "English", link: "/" },
          { text: "简体中文", link: "/zh-CN/" },
          { text: "繁體中文", link: "/zh-TW/" },
          { text: "فارسی", link: "/fa/" },
          { text: "Pinglish", link: "/fa-Latn/" }
        ]
      }
    ]
  }
})
