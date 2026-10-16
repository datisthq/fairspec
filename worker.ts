const ORIGIN = "https://fairspec.org"

const SUBDOMAINS: Record<string, string> = {
  "python.fairspec.org": "python",
  "typescript.fairspec.org": "typescript",
  "application.fairspec.org": "desktop",
}

const MOVED: Record<string, string> = {
  "/python/": "/python/getting-started/",
  "/typescript/": "/typescript/getting-started/",
  "/extensions/overview/": "/extensions/",
  "/overview/introduction/": "/overview/",
  "/specs/table/": "/specs/table-schema/",
}

interface Env {
  ASSETS: { fetch: (request: Request) => Promise<Response> }
}

export default {
  async fetch(request: Request, env: Env) {
    const url = new URL(request.url)
    const section = SUBDOMAINS[url.hostname]
    const path = section ? mapPath(section, url.pathname) : movedPath(url.pathname)
    if (!path) return env.ASSETS.fetch(request)

    const target = new URL(path, ORIGIN)
    target.search = url.search
    return Response.redirect(target.toString(), 301)
  },
}

/**
 * Maps a path on a retired subdomain to its page on the single site
 */
export function mapPath(section: string, pathname: string) {
  const path = normalizePath(pathname)

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

/**
 * Maps a retired path on fairspec.org to its current page, if it moved
 */
export function movedPath(pathname: string) {
  const path = normalizePath(pathname)

  const changelog = path.match(/^(\/(?:python\/|typescript\/)?changelog\/).+/)
  if (changelog) return changelog[1]

  return MOVED[path]
}

function normalizePath(pathname: string) {
  return `/${pathname.replace(/^\/+|\/+$/g, "")}/`.replace(/^\/\/$/, "/")
}
