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
    { title: "Standard", prefix: "/", icon: "book-open" },
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
      source: "https://github.com/fairspec/fairspec",
      version: true,
      icon: "history",
    },
    {
      type: "custom",
      title: "GitHub",
      url: "https://github.com/fairspec",
      icon: "github",
    },
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
