import { i18n } from "@lingui/core"
import { I18nProvider } from "@lingui/react"
import { TanStackDevtools } from "@tanstack/react-devtools"
import { ReactQueryDevtoolsPanel } from "@tanstack/react-query-devtools"
import { createRootRoute, Outlet } from "@tanstack/react-router"
import { TanStackRouterDevtoolsPanel } from "@tanstack/react-router-devtools"
import type * as React from "react"
import { Layout } from "#components/layout/Layout.tsx"
import { DefaultCatchBoundary } from "#components/system/DefaultCatchBoundary.tsx"
import { NotFound } from "#components/system/NotFound.tsx"
import { Toaster } from "#elements/sonner.tsx"

export const Route = createRootRoute({
  // TODO: improve NotFound page
  notFoundComponent: () => <NotFound />,
  errorComponent: props => {
    return (
      <Document>
        <DefaultCatchBoundary {...props} />
      </Document>
    )
  },
  component: () => {
    return (
      <Document>
        <Outlet />
      </Document>
    )
  },
})

function Document(props: { children: React.ReactNode }) {
  return (
    <I18nProvider i18n={i18n}>
      <Layout>{props.children}</Layout>
      <Toaster position="top-center" />
      <TanStackDevtools
        config={{ hideUntilHover: true }}
        plugins={[
          {
            name: "TanStack Query",
            render: <ReactQueryDevtoolsPanel />,
            defaultOpen: true,
          },
          {
            name: "TanStack Router",
            render: <TanStackRouterDevtoolsPanel />,
            defaultOpen: false,
          },
        ]}
      />
    </I18nProvider>
  )
}
