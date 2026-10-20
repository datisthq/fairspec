import { execFile } from "node:child_process"
import { basename, join } from "node:path"
import { promisify } from "node:util"
import type { Check } from "../profiles/types.ts"
import type { Tool } from "./types.ts"

export const RIVALS = join(import.meta.dirname, "..", "rivals")
export const RIVALS_PYTHON = join(RIVALS, ".venv", "bin", "python")

const POLARS_CAVEATS = [
  "date-time and boolean are matched by regex, since polars cannot cast text to them",
]
const TYPE_CAVEAT =
  "numeric columns are cast non-strictly first, so a bad number becomes null"

export const frictionlessTool = createRivalTool({
  name: "frictionless",
  title: "frictionless-py",
  distribution: "frictionless",
  unsupported: [],
  caveats: [],
})

export const panderaTool = createRivalTool({
  name: "pandera",
  title: "pandera (polars)",
  distribution: "pandera",
  unsupported: [],
  caveats: [
    ...POLARS_CAVEATS,
    "value checks do not run once any cell in the frame fails to coerce",
    "the whole table is read into memory, as pandera validates a LazyFrame one scan per check",
  ],
})

export const dataframelyTool = createRivalTool({
  name: "dataframely",
  title: "dataframely",
  distribution: "dataframely",
  unsupported: [],
  caveats: POLARS_CAVEATS,
})

export const pointblankTool = createRivalTool({
  name: "pointblank",
  title: "pointblank (polars)",
  distribution: "pointblank",
  unsupported: ["type"],
  caveats: [
    TYPE_CAVEAT,
    ...POLARS_CAVEATS,
    "the whole table is read into memory, as pointblank checks a LazyFrame one scan per step",
  ],
})

export const greatExpectationsTool = createRivalTool({
  name: "great-expectations",
  title: "Great Expectations (pandas)",
  distribution: "great-expectations",
  unsupported: ["type"],
  caveats: [
    TYPE_CAVEAT,
    "the whole table is loaded into pandas, its default engine; on Spark it ran 1.8x faster at 1GB with 2.6x the memory",
  ],
})

function createRivalTool(options: {
  name: string
  title: string
  distribution: string
  unsupported: Check[]
  caveats: string[]
}): Tool {
  const script = join(RIVALS, `validate_${options.name.replaceAll("-", "_")}.py`)
  return {
    name: options.name,
    title: options.title,
    unsupported: options.unsupported,
    caveats: options.caveats,
    getVersion: async () => {
      const { stdout } = await promisify(execFile)(RIVALS_PYTHON, [
        "-c",
        `import importlib.metadata; print(importlib.metadata.version("${options.distribution}"))`,
      ])
      return stdout.trim()
    },
    createCommand: async dataset => ({
      file: RIVALS_PYTHON,
      args: [script, basename(dataset.csvPath), basename(dataset.schemaPath)],
    }),
    countErrors: async ({ stdout }) =>
      JSON.parse(stdout.trim().split("\n").at(-1) ?? "").errorCount,
  }
}
