import { QueryClient } from "@tanstack/react-query"
import { createHashHistory, createRouter } from "@tanstack/react-router"
import { DefaultCatchBoundary } from "#components/system/DefaultCatchBoundary.tsx"
import { NotFound } from "#components/system/NotFound.tsx"
import { routeTree } from "#routeTree.gen.ts"

export const queryClient = new QueryClient()

export function getRouter() {
  const router = createRouter({
    routeTree,
    history: createHashHistory(),
    context: { queryClient },
    defaultPreload: "intent",
    defaultViewTransition: true,
    defaultErrorComponent: DefaultCatchBoundary,
    defaultNotFoundComponent: () => <NotFound />,
    scrollRestoration: true,
  })

  return router
}

declare module "@tanstack/react-router" {
  interface Register {
    router: ReturnType<typeof getRouter>
  }
}
