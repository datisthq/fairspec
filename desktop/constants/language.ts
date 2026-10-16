export type Language = (typeof Languages)[LanguageId]
export type LanguageId = keyof typeof Languages

export const LanguageIdDefault = "en"

export const Languages = {
  en: { id: "en", title: "English" },
  de: { id: "de", title: "Deutsch" },
  es: { id: "es", title: "Español" },
  fr: { id: "fr", title: "Français" },
  it: { id: "it", title: "Italiano" },
  pt: { id: "pt", title: "Português" },
  ru: { id: "ru", title: "Русский" },
  uk: { id: "uk", title: "Українська" },
} as const satisfies Record<string, AbstractLanguage>

interface AbstractLanguage {
  id: string
  title: string
}
