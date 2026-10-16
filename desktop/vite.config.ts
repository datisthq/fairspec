import { basename, dirname, join } from "node:path"
import { coverageConfigDefaults, defineConfig } from "vite-plus"

export default defineConfig({
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
