import { resolve } from "node:path"
import { lingui, linguiTransformerBabelPreset } from "@lingui/vite-plugin"
import babel from "@rolldown/plugin-babel"
import tailwind from "@tailwindcss/vite"
import { tanstackRouter } from "@tanstack/router-plugin/vite"
import react from "@vitejs/plugin-react"
import { defineConfig } from "electron-vite"
import svgr from "vite-plugin-svgr"

const root = import.meta.dirname

export default defineConfig({
  main: {
    build: {
      lib: { entry: "processes/main/main.ts" },
      outDir: "build/main",
    },
  },
  preload: {
    build: {
      lib: {
        entry: "processes/preload/preload.ts",
        formats: ["cjs"],
      },
      outDir: "build/preload",
      externalizeDeps: false,
      rollupOptions: {
        external: ["electron"],
        output: {
          entryFileNames: "preload.js",
        },
      },
    },
  },
  renderer: {
    root: resolve(root, "processes/renderer"),
    envDir: root,
    publicDir: resolve(root, "public"),
    server: {
      port: 8050,
      watch: {
        ignored: [
          "**/build/**",
          "**/compile/**",
          "**/coverage/**",
          "**/.tanstack/**",
          "**/.private/**",
        ],
      },
    },
    build: {
      outDir: "build/renderer",
      rollupOptions: {
        input: { index: resolve(root, "processes/renderer/index.html") },
      },
    },
    plugins: [
      tailwind(),
      tanstackRouter({
        target: "react",
        autoCodeSplitting: true,
        routesDirectory: resolve(root, "routes"),
        generatedRouteTree: resolve(root, "routeTree.gen.ts"),
      }),
      react(),
      lingui(),
      babel({
        presets: [linguiTransformerBabelPreset()],
      }),
      svgr(),
    ],
  },
})
