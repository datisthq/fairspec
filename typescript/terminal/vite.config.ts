import { defineConfig } from "vite-plus"

// Native and wasm packages that load files beside them; everything else is inlined
const EXTERNAL = ["calamine-node", "nodejs-polars", "wasm-xlsxwriter"]

export default defineConfig({
  pack: {
    entry: { run: "entrypoints/run.ts" },
    outDir: "build/entrypoints",
    // tsgo's emit shares build/, and the library export in it is not bundled
    clean: false,
    format: "esm",
    platform: "node",
    target: "node24",
    fixedExtension: false,
    // Stable chunk names, so a rebuild overwrites the previous chunks instead of adding to them
    hash: false,
    dts: false,
    shims: true,
    sourcemap: true,
    deps: {
      neverBundle: EXTERNAL,
      alwaysBundle: id =>
        !EXTERNAL.some(name => id === name || id.startsWith(`${name}/`)),
      onlyImport: EXTERNAL,
      onlyBundle: false,
    },
  },
})
