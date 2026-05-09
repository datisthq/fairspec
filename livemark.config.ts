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
    { title: "Docs", prefix: "/" },
    {
      title: "Changelog",
      prefix: "/changelog/",
      type: "changelog",
      source: "https://github.com/fairspec/fairspec",
      version: true,
    },
  ],
  links: [
    { url: "https://python.fairspec.org", title: "Python" },
    { url: "https://typescript.fairspec.org", title: "TypeScript" },
    { url: "/mcp-server/", title: "MCP Server" },
    { url: "https://application.fairspec.org", title: "Application" },
    { url: "https://github.com/fairspec", title: "GitHub", icon: "github" },
  ],
  patches: [
    {
      file: "README.md",
      article: {
        title: "Fairspec",
        label: "Overview",
        path: "/overview/",
        order: 10,
        icon: "book-open",
        group: "Articles",
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
