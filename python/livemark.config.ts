import { defineConfig } from "livemark"

export default defineConfig({
  site: "https://python.fairspec.org",
  title: "Fairspec Python",
  description: "Data management framework",
  logo: "/logo.svg",
  favicon: "/logo.png",
  include: ["README.md", "project/docs/**/*.md", "CONTRIBUTING.md"],
  sections: [
    {
      type: "custom",
      title: "Fairspec",
      icon: "house",
      url: "https://fairspec.org",
    },
    {
      type: "custom",
      title: "Standard",
      url: "https://fairspec.org/overview/",
      icon: "book-open",
    },
    { title: "Python", prefix: "/", icon: "code" },
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
      source: "CHANGELOG.md",
      version: true,
      icon: "history",
    },
    {
      type: "custom",
      title: "GitHub",
      url: "https://github.com/datisthq/fairspec-python",
      icon: "github",
    },
  ],
  patches: [
    {
      file: "README.md",
      article: {
        title: "Fairspec Python",
        label: "Getting Started",
        description: "Data management framework",
        icon: "rocket",
        path: "/",
        order: 0,
      },
    },
    {
      file: "CONTRIBUTING.md",
      article: {
        title: "Contributing",
        description:
          "How to set up the repository, propose changes, and ship a release.",
        icon: "heart-handshake",
        path: "/contributing/",
        order: -1,
      },
    },
  ],
})
