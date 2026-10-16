const ORIGIN = "https://fairspec.org"

const SUBDOMAINS: Record<string, string> = {
  "python.fairspec.org": "python",
  "typescript.fairspec.org": "typescript",
  "application.fairspec.org": "desktop",
}

interface Env {
  ASSETS: { fetch: (request: Request) => Promise<Response> }
}

export default {
  async fetch(request: Request, env: Env) {
    const url = new URL(request.url)
    const section = SUBDOMAINS[url.hostname]
    if (!section) return env.ASSETS.fetch(request)

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

  if (path === "/") return `/${section}/`
  if (path.startsWith("/changelog/")) {
    return section === "desktop" ? "/desktop/" : `/${section}/changelog/`
  }
  if (path === "/contributing/") return "/overview/contributing/"
  if (section === "desktop") return "/desktop/"
  if (path.startsWith("/mcp-server/")) return path
  if (path.startsWith("/terminal/")) return `/${section}${path}`
  if (path.startsWith(`/${section}/`)) {
    return `/${section}/library/${path.slice(section.length + 2)}`
  }

  return `/${section}/`
}
