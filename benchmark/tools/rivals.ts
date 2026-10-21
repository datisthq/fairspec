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

export const panderaIbisTool = createRivalTool({
  name: "pandera-ibis",
  title: "pandera (DuckDB)",
  distribution: "pandera",
  unsupported: ["type"],
  caveats: [
    TYPE_CAVEAT,
    "date-time and boolean are matched by regex",
    "the CSV is loaded into an on-disk DuckDB table first, as pandera re-reads a view over it once per check",
  ],
})

export const patitoTool = createRivalTool({
  name: "patito",
  title: "patito (polars)",
  distribution: "patito",
  isPartial: true,
  unsupported: ["type"],
  caveats: [
    "reports a count per check in its messages, not the failing rows",
    TYPE_CAVEAT,
    ...POLARS_CAVEATS,
    "enums are checked as a custom constraint, as patito's own enum check reports values, not rows",
    "the whole table is read into memory, as patito validates a DataFrame",
  ],
})

export const sodaTool = createRivalTool({
  name: "soda",
  title: "Soda Core (DuckDB)",
  distribution: "soda-core",
  isPartial: true,
  unsupported: [],
  caveats: [
    "types are matched by regex over text, and ranges checked on a TRY_CAST of the column",
    "failing rows are counted per check, not per cell",
  ],
})

export const datacontractTool = createRivalTool({
  name: "datacontract",
  title: "datacontract-cli (DuckDB)",
  distribution: "datacontract-cli",
  isPartial: true,
  unsupported: [],
  caveats: [
    "reports a count per check, not the failing rows",
    "the CSV is read as text through a DuckDB view, since its local CSV server aborts the whole run on one unparsable cell",
    "types are matched by regex, as datacontract checks column types only and skips them for CSV",
    "ranges are checked on a second view that casts numbers non-strictly, read separately from the first",
  ],
})

function createRivalTool(options: {
  name: string
  title: string
  distribution: string
  unsupported: Check[]
  caveats: string[]
  isPartial?: boolean
}): Tool {
  const script = join(RIVALS, `validate_${options.name.replaceAll("-", "_")}.py`)
  return {
    name: options.name,
    title: options.title,
    unsupported: options.unsupported,
    caveats: options.caveats,
    isPartial: options.isPartial,
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
    countErrors: async ({ stdout }) => readScriptResult(stdout).errorCount,
    readElapsedMs: stdout => readScriptResult(stdout).elapsedMs,
  }
}

/** Reads the `{"errorCount", "elapsedMs"}` line an adapter script prints last. */
export function readScriptResult(stdout: string): {
  errorCount: number
  elapsedMs: number
} {
  return JSON.parse(stdout.trim().split("\n").at(-1) ?? "")
}
