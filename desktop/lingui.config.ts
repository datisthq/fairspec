import { defineConfig } from "@lingui/cli"
import { formatter } from "@lingui/format-po"
import { LanguageIdDefault, Languages } from "#constants/language.ts"

export default defineConfig({
  sourceLocale: LanguageIdDefault,
  locales: Object.keys(Languages),
  fallbackLocales: { default: LanguageIdDefault },
  catalogs: [
    {
      path: "<rootDir>/locales/{locale}/messages",
      include: ["<rootDir>"],
      exclude: [
        "**/node_modules/**",
        "**/build/**",
        "**/compile/**",
        "**/processes/**",
        "**/.livemark/**",
        "**/actions/**",
        "**/endpoints/**",
        "**/middlewares/**",
        "**/*.gen.ts",
        "**/*.d.ts",
      ],
    },
  ],
  format: formatter({ lineNumbers: false }),
})
