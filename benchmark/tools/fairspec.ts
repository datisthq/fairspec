import { execFile } from "node:child_process"
import { readFile } from "node:fs/promises"
import { basename, join } from "node:path"
import { promisify } from "node:util"
import type { Dataset } from "../generate/generate.ts"
import { readScriptResult } from "./rivals.ts"
import type { Tool } from "./types.ts"

const ROOT = join(import.meta.dirname, "..", "..")
const SCRIPTS = join(import.meta.dirname, "..", "fairspec")

export const fairspecPythonTool = createFairspecTool({
  name: "fairspec-python",
  title: "Fairspec Python",
  section: "python",
  file: join(ROOT, "python", ".venv", "bin", "python"),
  script: join(SCRIPTS, "validate.py"),
})

export const fairspecTypescriptTool = createFairspecTool({
  name: "fairspec-typescript",
  title: "Fairspec TypeScript",
  section: "typescript",
  file: process.execPath,
  script: join(SCRIPTS, "validate.ts"),
})

const RUST = join(ROOT, "rust")

export const fairspecRustlangTool: Tool = {
  ...createFairspecTool({
    name: "fairspec-rust",
    title: "Fairspec Rust",
    section: "rust",
    file: join(RUST, "target", "dist-release", "validate"),
  }),
  // cargo is a no-op when the build is fresh, and runs before the clock starts
  createCommand: async (dataset: Dataset) => {
    await promisify(execFile)("pnpm", ["build:dist", "-p", "fairspec-benchmark"], {
      cwd: RUST,
      maxBuffer: 64 * 1024 * 1024,
    })
    return {
      file: join(RUST, "target", "dist-release", "validate"),
      args: [basename(dataset.csvPath), basename(dataset.schemaPath)],
    }
  },
}

function createFairspecTool(options: {
  name: string
  title: string
  section: "python" | "typescript" | "rust"
  file: string
  script?: string
}): Tool {
  return {
    name: options.name,
    title: options.title,
    unsupported: [],
    caveats: [],
    getVersion: async () => {
      const manifest = JSON.parse(
        await readFile(join(ROOT, ".release", "manifest.json"), "utf8"),
      )
      const { stdout } = await promisify(execFile)(
        "git",
        ["rev-parse", "--short", "HEAD"],
        {
          cwd: ROOT,
        },
      )
      return `${manifest[options.section] ?? "0.0.0"}+${stdout.trim()}`
    },
    createCommand: async (dataset: Dataset) => ({
      file: options.file,
      args: [
        ...(options.script ? [options.script] : []),
        basename(dataset.csvPath),
        basename(dataset.schemaPath),
      ],
    }),
    countErrors: async ({ stdout }) => readScriptResult(stdout).errorCount,
    readElapsedMs: stdout => readScriptResult(stdout).elapsedMs,
  }
}
