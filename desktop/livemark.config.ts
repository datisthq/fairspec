import { defineConfig } from "livemark"

export default defineConfig({
  site: "https://application.fairspec.org",
  title: "Fairspec Application",
  description: "Visual data validation",
  logo: "/logo.svg",
  favicon: "/logo.png",
  include: ["docs/**/*.md"],
  sections: [
    { type: "custom", title: "Fairspec", url: "https://fairspec.org", icon: "house" },
    {
      type: "custom",
      title: "Standard",
      url: "https://fairspec.org/overview/",
      icon: "book-open",
    },
    { type: "custom", title: "Python", url: "https://python.fairspec.org", icon: "code" },
    {
      type: "custom",
      title: "TypeScript",
      url: "https://typescript.fairspec.org",
      icon: "code-xml",
    },
    {
      type: "custom",
      title: "MCP Server",
      url: "https://fairspec.org/mcp-server/",
      icon: "sparkles",
    },
    { type: "custom", title: "Application", url: "/", icon: "app-window" },
    {
      title: "Changelog",
      prefix: "/changelog/",
      type: "changelog",
      source: "CHANGELOG.md",
      version: true,
      icon: "history",
    },
    {
      type: "custom",
      title: "GitHub",
      url: "https://github.com/datisthq/fairspec-application",
      icon: "github",
    },
  ],
})
