import { i18n } from "@lingui/core"
import { type LanguageId, LanguageIdDefault, Languages } from "#constants/language.ts"

export async function activateLocale(languageId: LanguageId) {
  const { messages } = await import(`../locales/${languageId}/messages.po`)

  i18n.load(languageId, messages)
  i18n.activate(languageId)

  globalThis.document.documentElement.lang = languageId
}

export async function detectClientLanguage() {
  const languageId = await globalThis.desktop.getLanguage()

  const language =
    Object.values(Languages).find(language => language.id === languageId) ??
    Languages[LanguageIdDefault]

  return language
}

export async function setLanguage(languageId: LanguageId) {
  await globalThis.desktop.setLanguage(languageId)
}
