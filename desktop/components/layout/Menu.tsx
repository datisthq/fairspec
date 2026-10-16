import { useLingui } from "@lingui/react/macro"
import { Link, useLocation } from "@tanstack/react-router"
import { ChevronRight } from "lucide-react"
import { useState } from "react"
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "#elements/collapsible.tsx"
import {
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
  useSidebar,
} from "#elements/sidebar.tsx"
import * as icons from "#icons.ts"

export function Menu() {
  const { t } = useLingui()
  const { setOpenMobile } = useSidebar()
  const pathname = useLocation({ select: l => l.pathname })
  const isActive = (itemPath: string) => {
    return pathname === itemPath || pathname === `${itemPath}/`
  }

  const [openStates, setOpenStates] = useState<Record<string, boolean>>({
    dataset: true,
    table: true,
    data: true,
    file: true,
  })

  const menuItems = [
    {
      id: "dataset",
      label: t`Dataset`,
      icon: icons.Dataset,
      path: "/dataset",
      items: [
        {
          label: t`Validate Dataset`,
          path: "/dataset/validate",
        },
        { label: t`Infer Dataset`, path: "/dataset/infer" },
      ],
    },
    {
      id: "table",
      label: t`Table`,
      icon: icons.Table,
      path: "/table",
      items: [
        { label: t`Preview Table`, path: "/table/preview" },
        { label: t`Validate Table`, path: "/table/validate" },
        {
          label: t`Infer Schema`,
          path: "/table/infer-schema",
        },
        {
          label: t`Infer Dialect`,
          path: "/file/infer-dialect",
        },
      ],
    },
    {
      id: "data",
      label: t`Data`,
      icon: icons.Data,
      path: "/data",
      items: [
        { label: t`Validate Data`, path: "/data/validate" },
        { label: t`Infer Schema`, path: "/data/infer-schema" },
        {
          label: t`Infer Dialect`,
          path: "/file/infer-dialect",
        },
      ],
    },
    {
      id: "file",
      label: t`File`,
      icon: icons.File,
      path: "/file",
      items: [
        { label: t`Validate File`, path: "/file/validate" },
        {
          label: t`Infer Dialect`,
          path: "/file/infer-dialect",
        },
      ],
    },
  ]

  return (
    <SidebarGroup>
      <SidebarGroupLabel className="uppercase font-mono text-xs tracking-widest">
        {t`Tools`}
      </SidebarGroupLabel>
      <SidebarGroupContent>
        <SidebarMenu>
          {menuItems.map(menuItem => {
            const Icon = menuItem.icon
            const parentActive = isActive(menuItem.path)
            return (
              <Collapsible
                key={menuItem.id}
                open={openStates[menuItem.id]}
                onOpenChange={(open: boolean) =>
                  setOpenStates(prev => ({ ...prev, [menuItem.id]: open }))
                }
                className="group/collapsible"
              >
                <SidebarMenuItem>
                  <SidebarMenuButton
                    isActive={parentActive}
                    className={parentActive ? "" : "opacity-75"}
                    render={
                      <Link to={menuItem.path} onClick={() => setOpenMobile(false)} />
                    }
                  >
                    <Icon className="size-4" />
                    <span>{menuItem.label}</span>
                  </SidebarMenuButton>
                  <CollapsibleTrigger
                    aria-label={`Toggle ${menuItem.label} submenu`}
                    className="absolute right-1 top-1.5 p-1 rounded-md hover:bg-sidebar-accent"
                  >
                    <ChevronRight className="size-3.5 transition-transform group-data-[state=open]/collapsible:rotate-90" />
                  </CollapsibleTrigger>
                  <CollapsibleContent>
                    <SidebarMenuSub>
                      {menuItem.items.map(item => {
                        const active = isActive(item.path)
                        return (
                          <SidebarMenuSubItem key={item.path}>
                            <SidebarMenuSubButton
                              isActive={active}
                              className={active ? "" : "opacity-75"}
                              render={
                                <Link
                                  to={item.path}
                                  onClick={() => setOpenMobile(false)}
                                />
                              }
                            >
                              <span>{item.label}</span>
                            </SidebarMenuSubButton>
                          </SidebarMenuSubItem>
                        )
                      })}
                    </SidebarMenuSub>
                  </CollapsibleContent>
                </SidebarMenuItem>
              </Collapsible>
            )
          })}
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarGroup>
  )
}
