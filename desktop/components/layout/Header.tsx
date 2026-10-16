import { Link } from "@tanstack/react-router"
import { Button } from "#elements/button.tsx"
import { useSidebar } from "#elements/sidebar.tsx"
import * as icons from "#icons.ts"
import { Close } from "./Close.tsx"
import { Logo } from "./Logo.tsx"
import { Maximize } from "./Maximize.tsx"

function MenuToggle() {
  const { toggleSidebar } = useSidebar()
  return (
    <Button
      data-sidebar="trigger"
      variant="ghost"
      size="icon-sm"
      onClick={toggleSidebar}
      aria-label="Toggle Sidebar"
    >
      <icons.Menu />
      <span className="sr-only">Toggle Sidebar</span>
    </Button>
  )
}

type IconComponent = React.ComponentType<{ className?: string }>

interface HeaderSection {
  key: string
  title: string
  icon?: IconComponent
  href: string
  external: boolean
  active?: boolean
}

const sections: readonly HeaderSection[] = [
  {
    key: "fairspec",
    title: "Fairspec",
    icon: icons.House,
    href: "https://fairspec.org",
    external: true,
  },
  {
    key: "standard",
    title: "Standard",
    icon: icons.Book,
    href: "https://fairspec.org/overview/",
    external: true,
  },
  {
    key: "python",
    title: "Python",
    icon: icons.Code,
    href: "https://python.fairspec.org",
    external: true,
  },
  {
    key: "typescript",
    title: "TypeScript",
    icon: icons.CodeXml,
    href: "https://typescript.fairspec.org",
    external: true,
  },
  {
    key: "mcp",
    title: "MCP Server",
    icon: icons.Sparkles,
    href: "https://fairspec.org/mcp-server/",
    external: true,
  },
  {
    key: "application",
    title: "Application",
    icon: icons.AppWindow,
    href: "/",
    external: false,
    active: true,
  },
  {
    key: "github",
    title: "GitHub",
    icon: icons.GitHub,
    href: "https://github.com/datisthq",
    external: true,
  },
]

function activeClass(active: boolean) {
  return active
    ? "font-medium border-b-2 border-foreground pb-0.5 -mb-0.5"
    : "opacity-80 hover:opacity-100 transition-opacity"
}

export function Header() {
  return (
    <header className="sticky top-0 z-20 flex h-16 shrink-0 items-center border-b bg-background">
      <div className="flex items-center self-stretch pl-4 pr-4">
        <div className="hidden lg:flex">
          <MenuToggle />
        </div>
        <Link to="/" className="lg:hidden flex items-center">
          <Logo />
        </Link>
      </div>
      <div className="hidden lg:flex flex-1 items-center gap-8 self-stretch px-6 text-sm">
        {sections.map(section => {
          const Icon = section.icon
          const classes = `inline-flex items-center gap-1.5 text-foreground ${activeClass(!!section.active)}`
          if (section.external) {
            return (
              <a key={section.key} href={section.href} className={classes}>
                {Icon && <Icon className="size-3.5" />}
                <span>{section.title}</span>
              </a>
            )
          }
          return (
            <Link key={section.key} to={section.href} className={classes}>
              {Icon && <Icon className="size-3.5" />}
              <span>{section.title}</span>
            </Link>
          )
        })}
      </div>
      <div className="lg:hidden ml-auto pr-2">
        <MenuToggle />
      </div>
      <div className="hidden lg:flex items-center gap-1 pr-4">
        <Maximize />
        <Close />
      </div>
    </header>
  )
}
