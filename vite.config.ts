import { defineConfig } from "vite-plus"

const ignorePatterns = [
  "**/CHANGELOG.md",
  "**/*.d.ts",
  "**/*.gen.*",
  "**/generated/**",
  "**/.livemark/build/**",
  "**/.livemark/public/profiles/**",
  "**/.tanstack/**",
  "**/.venv/**",
  "**/build/**",
  "**/compile/**",
  "**/coverage/**",
  "**/htmlcov/**",
  "**/node_modules/**",
]

export default defineConfig({
  test: {
    include: [".scripts/-test/*.unit.ts"],
  },
  fmt: {
    semi: false,
    printWidth: 90,
    arrowParens: "avoid",
    ignorePatterns,
  },
  lint: {
    ignorePatterns,
    options: {
      typeAware: false,
      typeCheck: false,
    },
    overrides: [
      {
        files: ["desktop/**"],
        rules: {
          "unicorn/no-single-promise-in-promise-methods": "off",
        },
      },
      {
        // The renderer must never reach into main-process code. The only legal
        // crossing is the type-only Router import in services/engine.ts.
        files: [
          "desktop/processes/renderer/**",
          "desktop/routes/**",
          "desktop/components/**",
          "desktop/elements/**",
          "desktop/hooks/**",
          "desktop/helpers/**",
        ],
        rules: {
          "no-restricted-imports": [
            "error",
            {
              patterns: [
                "#processes/main/**",
                "#actions/**",
                "#endpoints/**",
                "#middlewares/**",
                "electron",
                "node:*",
              ],
            },
          ],
        },
      },
    ],
  },
})
