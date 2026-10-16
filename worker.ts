const ORIGIN = "https://fairspec.org"

const SUBDOMAINS: Record<string, string> = {
  "python.fairspec.org": "python",
  "typescript.fairspec.org": "typescript",
  "application.fairspec.org": "desktop",
}

const SECTION_HOMES: Record<string, string> = {
  "/python/": "/python/getting-started/",
  "/typescript/": "/typescript/getting-started/",
}

interface Env {
  ASSETS: { fetch: (request: Request) => Promise<Response> }
}

export default {
  async fetch(request: Request, env: Env) {
    const url = new URL(request.url)
    const section = SUBDOMAINS[url.hostname]
    if (!section) {
      const home = SECTION_HOMES[url.pathname]
      if (!home) return env.ASSETS.fetch(request)
      return Response.redirect(new URL(home + url.search, ORIGIN).toString(), 301)
    }

    const target = new URL(mapPath(section, url.pathname), ORIGIN)
    target.search = url.search
    return Response.redirect(target.toString(), 301)
  },
}

/**
 * Maps a path on a retired subdomain to its page on the single site
 */
export function mapPath(section: string, pathname: string) {
  const path = `/${pathname.replace(/^\/+|\/+$/g, "")}/`.replace(/^\/\/$/, "/")

  const home = section === "desktop" ? "/desktop/" : `/${section}/getting-started/`

  if (path === "/") return home
  if (path.startsWith("/changelog/")) {
    return section === "desktop" ? "/desktop/" : `/${section}/changelog/`
  }
  if (path === "/contributing/") return "/overview/contributing/"
  if (section === "desktop") return home
  if (path.startsWith("/mcp-server/")) return path
  if (path.startsWith("/terminal/")) return `/${section}${path}`
  if (path.startsWith(`/${section}/`)) {
    return `/${section}/library/${path.slice(section.length + 2)}`
  }

  return home
}
