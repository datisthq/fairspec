import { basename, dirname, join } from "node:path"
import { coverageConfigDefaults, defineConfig } from "vite-plus"

export default defineConfig({
  test: {
    include: ["**/*.unit.(ts|tsx)"],
    exclude: ["**/node_modules/**", "**/build/**", "**/compile/**", "**/.livemark/**"],
    env: { NODE_OPTIONS: "--no-warnings" },
    pool: "forks",
    // One fork per core re-imports every module in each, for no wall-time gain: import
    // time already dwarfs test time. Half the cores leaves the machine usable.
    maxWorkers: "50%",
    testTimeout: 60 * 1000,
    passWithNoTests: true,
    silent: "passed-only",
    coverage: {
      // Off so the run before landing stays cheap; `pnpm measure:coverage` opts in.
      enabled: false,
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
