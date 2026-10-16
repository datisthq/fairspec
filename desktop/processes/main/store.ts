import type { LanguageId } from "#constants/language.ts"
import Store from "electron-store"

interface StoreState {
  lastOpenedFolder?: string
  languageId?: LanguageId | null
  zoomFactor?: number
  theme?: "light" | "dark"
}

export const store = new Store<StoreState>()
