import { useHotkey } from "@tanstack/react-hotkeys"
import { Link } from "@tanstack/react-router"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarRail,
  useSidebar,
} from "#elements/sidebar.tsx"
import { Content } from "./Content.tsx"
import { Header } from "./Header.tsx"
import { Language } from "./Language.tsx"
import { Logo } from "./Logo.tsx"
import { Menu } from "./Menu.tsx"
import { Theme } from "./Theme.tsx"

function SidebarHotkey() {
  const { toggleSidebar } = useSidebar()
  useHotkey("S", toggleSidebar)
  return null
}

export function Layout(props: { children?: React.ReactNode }) {
  return (
    <SidebarProvider defaultOpen={true}>
      <SidebarHotkey />
      <Sidebar collapsible="offcanvas" side="left">
        <SidebarHeader>
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton size="lg" render={<Link to="/" />}>
                <Logo />
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarHeader>
        <SidebarContent>
          <Menu />
        </SidebarContent>
        <SidebarFooter className="p-4 space-y-2">
          <Language />
          <Theme />
        </SidebarFooter>
        <SidebarRail />
      </Sidebar>
      <SidebarInset>
        <Header />
        <main className="flex-1">
          <Content>{props.children}</Content>
        </main>
      </SidebarInset>
    </SidebarProvider>
  )
}
