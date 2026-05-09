import { useLocation } from "@tanstack/react-router"
import { config } from "livemark:virtual"

export function SiteTitle() {
  const { pathname } = useLocation()
  const isMcp = pathname.startsWith("/mcp-server")
  const isStandard =
    pathname.startsWith("/overview") ||
    pathname.startsWith("/specs") ||
    pathname.startsWith("/extensions")
  const title = isMcp
    ? "Fairspec MCP Server"
    : isStandard
      ? "Fairspec Standard"
      : config.title
  const description = isMcp
    ? "AI-assisted data management"
    : isStandard
      ? "Data exchange specifications"
      : config.description
  return (
    <div className="flex items-end gap-2.5 text-sm">
      {config.logo && (
        <img src={config.logo} alt={title} className="size-6.5 mb-1.25" />
      )}
      <div className="flex flex-col gap-0.5 leading-none">
        <span className="font-semibold">{title}</span>
        {description && (
          <span className="text-xs opacity-80">{description}</span>
        )}
      </div>
    </div>
  )
}
