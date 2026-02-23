import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'heliosHarness',
  description: 'Harness docs',
  srcExclude: ['fragemented/research/**'],
  themeConfig: {
    nav: [
      { text: 'Start Here', link: '/index' },
      { text: 'Tutorials', link: '/tutorials/' },
      { text: 'How-to', link: '/how-to/' },
      { text: 'Explanation', link: '/explanation/' },
      { text: 'Operations', link: '/operations/' },
      { text: 'API', link: '/api/' }
    ]
  }
})
