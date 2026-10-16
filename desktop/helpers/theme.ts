export type Theme = "light" | "dark"

export async function detectClientTheme() {
  const theme = await globalThis.desktop.getTheme()
  return theme || "light"
}

export function activateTheme(theme: Theme) {
  if (theme === "dark") {
    globalThis.document.documentElement.classList.add("dark")
  } else {
    globalThis.document.documentElement.classList.remove("dark")
  }
}

export function getCurrentTheme() {
  return globalThis.document?.documentElement.classList.contains("dark")
    ? "dark"
    : "light"
}

export async function setTheme(theme: Theme) {
  activateTheme(theme)
  await globalThis.desktop.setTheme(theme)
}
