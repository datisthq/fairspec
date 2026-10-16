import { QueryClientProvider } from "@tanstack/react-query"
import { RouterProvider } from "@tanstack/react-router"
import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import { activateLocale, detectClientLanguage } from "#helpers/locale.ts"
import { activateTheme, detectClientTheme } from "#helpers/theme.ts"
import { getRouter, queryClient } from "#router.tsx"
import "#styles/general.css"

const language = await detectClientLanguage()
await activateLocale(language.id)

const theme = await detectClientTheme()
activateTheme(theme)

const router = getRouter()

const rootElement = document.getElementById("root")
if (!rootElement) {
  throw new Error("Root element not found")
}

createRoot(rootElement).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </StrictMode>,
)
