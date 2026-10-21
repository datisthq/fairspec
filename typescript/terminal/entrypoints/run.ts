#!/usr/bin/env node
import { enableCompileCache } from "node:module"

enableCompileCache()

process.removeAllListeners("warning")
process.on("warning", warning => {
  if (warning.name === "ExperimentalWarning") {
    return
  }
  console.warn(warning)
})

// Dynamic, so the bundle loads the program in its own chunk after the filter above is set
const { program } = await import("../program.ts")
program.parse()
