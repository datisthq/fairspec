import type { LanguageId } from "#constants/language.ts"

export interface DesktopAPI {
  engineIpc: string

  openFileDialog: (options: {
    filters?: { name: string; extensions: string[] }[]
  }) => Promise<string | undefined>

  saveFileDialog: (options: {
    defaultPath?: string
    filters?: { name: string; extensions: string[] }[]
  }) => Promise<string | undefined>

  writeFile: (options: {
    filePath: string
    content: string
  }) => Promise<string>

  getTheme: () => Promise<"light" | "dark">
  setTheme: (theme: "light" | "dark") => Promise<"light" | "dark">

  getLanguage: () => Promise<LanguageId | undefined>
  setLanguage: (languageId: LanguageId) => Promise<LanguageId>

  toggleMaximize: () => Promise<void>
  isFullScreen: () => Promise<boolean>
  onFullScreenChange: (callback: (isFullScreen: boolean) => void) => () => void
}

declare global {
  var desktop: DesktopAPI
}
