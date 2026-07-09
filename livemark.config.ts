import { defineConfig } from "livemark"

export default defineConfig({
  site: "https://fairspec.org",
  title: "Fairspec",
  description: "FAIR data exchange format",
  logo: "/logo.svg",
  favicon: "/logo.png",
  include: [
    "README.md",
    "overview/**/*.md",
    "specs.md",
    "specs/**/*.md",
    "extensions.md",
    "extensions/**/*.md",
    "CONTRIBUTING.md",
  ],
  sections: [
    {
      type: "custom",
      title: "Fairspec",
      icon: "house",
      url: "/",
    },
    {
      title: "Standard",
      prefix: "/",
      icon: "book-open",
      siteTitle: "Fairspec Standard",
      siteDescription: "Data exchange specifications",
      siteLink: "/overview/",
    },
    {
      type: "custom",
      title: "Python",
      url: "https://python.fairspec.org",
      icon: "code",
    },
    {
      type: "custom",
      title: "TypeScript",
      url: "https://typescript.fairspec.org",
      icon: "code-xml",
    },
    {
      type: "custom",
      title: "MCP Server",
      url: "/mcp-server/",
      icon: "sparkles",
      siteTitle: "Fairspec MCP Server",
      siteDescription: "AI-assisted data management",
      siteLink: "/mcp-server/",
    },
    {
      type: "custom",
      title: "Application",
      url: "https://application.fairspec.org",
      icon: "app-window",
    },
    {
      title: "Changelog",
      prefix: "/changelog/",
      type: "changelog",
      source: "https://github.com/datisthq/fairspec",
      version: true,
      icon: "history",
    },
    {
      type: "custom",
      title: "GitHub",
      url: "https://github.com/datisthq",
      icon: "github",
    },
  ],
  patches: [
    {
      file: "README.md",
      article: {
        title: "Fairspec Standard",
        label: "Overview",
        path: "/overview/",
        order: 10,
        icon: "book-open",
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
  ],
})
