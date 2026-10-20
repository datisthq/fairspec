import { execFile } from "node:child_process"
import { readFile } from "node:fs/promises"
import { basename, join } from "node:path"
import { promisify } from "node:util"
import type { Dataset } from "../generate/generate.ts"
import type { Tool } from "./types.ts"

const ROOT = join(import.meta.dirname, "..", "..")

export const fairspecPythonTool = createFairspecTool({
  name: "fairspec-python",
  title: "Fairspec Python",
  section: "python",
  file: join(ROOT, "python", ".venv", "bin", "fairspec"),
  args: [],
})

export const fairspecTypescriptTool = createFairspecTool({
  name: "fairspec-typescript",
  title: "Fairspec TypeScript",
  section: "typescript",
  file: process.execPath,
  args: [join(ROOT, "typescript", "terminal", "build", "entrypoints", "run.js")],
})

function createFairspecTool(options: {
  name: string
  title: string
  section: "python" | "typescript"
  file: string
  args: string[]
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
      return `${manifest[options.section]}+${stdout.trim()}`
    },
    createCommand: async (dataset: Dataset) => ({
      file: options.file,
      args: [
        ...options.args,
        "table",
        "validate",
        basename(dataset.csvPath),
        "--schema",
        basename(dataset.schemaPath),
        "--json",
      ],
    }),
    countErrors: async ({ stdout }) => JSON.parse(stdout).errors.length,
  }
}
