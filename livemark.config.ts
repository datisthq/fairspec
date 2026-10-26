import { defineConfig } from "livemark"

export default defineConfig({
  site: "https://fairspec.org",
  title: "Fairspec",
  description: "Data exchange format",
  logo: "/logo.svg",
  favicon: "/logo.png",
  include: [
    "standard/README.md",
    "standard/overview/**/*.md",
    "standard/specs.md",
    "standard/specs/**/*.md",
    "standard/extensions.md",
    "standard/extensions/**/*.md",
    "python/README.md",
    "python/project/docs/**/*.md",
    "typescript/README.md",
    "typescript/project/docs/**/*.md",
    "standard/CHANGELOG.md",
    "python/CHANGELOG.md",
    "typescript/CHANGELOG.md",
    "CONTRIBUTING.md",
  ],
  skills: ".skills/*/SKILL.md",
  sections: [
    {
      title: "Standard",
      prefix: "/",
      icon: "book-open",
    },
    {
      title: "Python",
      prefix: "/python/",
      icon: "code",
    },
    {
      title: "TypeScript",
      prefix: "/typescript/",
      icon: "code-xml",
    },
    {
      type: "custom",
      title: "CLI",
      url: "/cli-tool/",
      icon: "square-terminal",
    },
    {
      type: "custom",
      title: "MCP",
      url: "/mcp-server/",
      icon: "sparkles",
    },
    {
      type: "custom",
      title: "Desktop",
      url: "/desktop/",
      icon: "app-window",
    },
    {
      type: "custom",
      title: "GitHub",
      url: "https://github.com/datisthq/fairspec",
      icon: "github",
    },
  ],
  patches: [
    {
      file: "standard/README.md",
      article: {
        title: "Fairspec Standard",
        label: "Overview",
        path: "/overview/",
        order: 10,
        icon: "book-open",
      },
    },
    {
      file: "python/README.md",
      article: {
        title: "Fairspec Python",
        label: "Getting Started",
        description: "Data management framework",
        icon: "rocket",
        path: "/python/getting-started/",
        order: 0,
      },
    },
    {
      file: "typescript/README.md",
      article: {
        title: "Fairspec TypeScript",
        label: "Getting Started",
        description: "Data management framework",
        icon: "rocket",
        path: "/typescript/getting-started/",
        order: 0,
      },
    },
    {
      file: "CONTRIBUTING.md",
      article: {
        title: "Contributing",
        path: "/overview/contributing/",
        order: 99,
      },
    },
    {
      file: "standard/CHANGELOG.md",
      article: {
        title: "Changelog",
        path: "/changelog/",
        icon: "history",
        order: -1,
      },
    },
    {
      file: "python/CHANGELOG.md",
      article: {
        title: "Changelog",
        path: "/python/changelog/",
        icon: "history",
        order: -1,
      },
    },
    {
      file: "typescript/CHANGELOG.md",
      article: {
        title: "Changelog",
        path: "/typescript/changelog/",
        icon: "history",
        order: -1,
      },
    },
  ],
})
