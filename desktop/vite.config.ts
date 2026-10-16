import { basename, dirname, join } from "node:path"
import { coverageConfigDefaults, defineConfig } from "vite-plus"

const ignorePatterns = [
  "**/*.d.ts",
  "**/*.gen.*",
  "**/generated/**",
  "**/.livemark/build/**",
  "**/.tanstack/**",
]

export default defineConfig({
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
    rules: {
      "unicorn/no-single-promise-in-promise-methods": "off",
    },
    overrides: [
      {
        // The renderer must never reach into main-process code. The only legal
        // crossing is the type-only Router import in services/engine.ts.
        files: [
          "processes/renderer/**",
          "routes/**",
          "components/**",
          "elements/**",
          "hooks/**",
          "helpers/**",
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
  test: {
    include: ["**/*.unit.(ts|tsx)"],
    exclude: ["**/node_modules/**", "**/build/**", "**/compile/**", "**/.livemark/**"],
    env: { NODE_OPTIONS: "--no-warnings" },
    testTimeout: 60 * 1000,
    passWithNoTests: true,
    silent: "passed-only",
    coverage: {
      enabled: true,
      reporter: ["html", "json"],
      exclude: [
        ...coverageConfigDefaults.exclude,
        "**/@*",
        "**/*.gen.ts",
        "**/build/**",
        "**/compile/**",
        "**/coverage/**",
        "**/.livemark/**",
        "**/locales/**",
        "**/examples/**",
        "**/generated/**",
        "**/messages.js",
      ],
    },
    resolveSnapshotPath: (testPath: string, snapExtension: string) => {
      return (
        join(dirname(testPath), "fixtures", "generated", basename(testPath)) +
        snapExtension
      )
    },
  },
})
