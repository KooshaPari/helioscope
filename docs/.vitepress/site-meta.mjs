export function createSiteMeta({ base = '/' } = {}) {
  return {
    base,
    title: 'heliosCLI',
    description: 'heliosCLI documentation',
    themeConfig: {
      nav: [
        { text: 'Home', link: base || '/' },
        { text: 'Guide', link: '/guide/' },
      ],
    },
  }
}
