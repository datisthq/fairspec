import { defineConfig } from "vite-plus"

const ignorePatterns = [
  "CHANGELOG.md",
  "**/.livemark/build/**",
  "**/.livemark/public/profiles/**",
  "**/.tanstack/**",
  "**/build/**",
  "**/node_modules/**",
]

export default defineConfig({
  fmt: {
    semi: false,
    printWidth: 80,
    arrowParens: "avoid",
    ignorePatterns,
  },
  lint: {
    ignorePatterns,
    options: {
      typeAware: false,
      typeCheck: false,
    },
  },
})
