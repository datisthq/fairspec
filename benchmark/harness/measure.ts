import { rm } from "node:fs/promises"
import { dirname } from "node:path"
import type { Dataset } from "../generate/generate.ts"
import type { Check } from "../profiles/types.ts"
import type { Tool } from "../tools/types.ts"
import { type Outcome, runCommand } from "./run.ts"

export type Result = {
  case: string
  tool: string
  title: string
  version: string
  profile: string
  size: string
  rows: number
  bytes: number
  runs: number
  wallMs?: { median: number; min: number; max: number }
  /** Peak anonymous memory; mapped file pages are excluded, so mmap readers compare fairly. */
  peakMemoryMb?: number
  peakRssMb?: number
  outcome: Outcome
  /** How the failed run ended, for example `code 120` or `signal 9`. */
  exit?: string
  /** How long the failed run lasted before it timed out, ran out of memory or crashed. */
  failedAfterMs?: number
  errorCount?: number
  expectedErrors: number
  /** False when the tool reported a different number of errors than it can detect. */
  correct: boolean
  unsupported: Check[]
  caveats: string[]
  isPartial?: boolean
  stderr?: string
}

/** Runs one tool on one dataset `warmup + runs` times; the first failed run ends the series. */
export async function measureTool(options: {
  caseName: string
  tool: Tool
  dataset: Dataset
  runs: number
  warmup: number
  timeoutMs: number
  memoryMax?: string
}) {
  const { caseName, tool, dataset, runs, warmup, timeoutMs, memoryMax } = options
  const expectedErrors = Object.entries(dataset.expected)
    .filter(([check]) => !tool.unsupported.some(unsupported => unsupported === check))
    .reduce((sum, [, count]) => sum + count, 0)

  const timings: number[] = []
  const memories: number[] = []
  const residents: number[] = []
  const counts: number[] = []
  let outcome: Outcome = "ok"
  let stderr: string | undefined
  let failedAfterMs: number | undefined
  let exit: string | undefined

  for (let index = 0; index < warmup + runs; index++) {
    const command = await tool.createCommand(dataset)
    const run = await runCommand(command, {
      cwd: dirname(dataset.csvPath),
      timeoutMs,
      memoryMax,
    })

    let count: number | undefined
    if (run.outcome === "ok") {
      count = await tool
        .countErrors({ stdout: run.stdout, dataset })
        .catch(() => undefined)
      if (count === undefined) run.outcome = "crash"
    }
    for (const path of command.outputs ?? []) await rm(path, { force: true })

    if (run.outcome !== "ok" || count === undefined) {
      outcome = run.outcome
      failedAfterMs = run.wallMs
      exit = run.exit
      stderr = run.stderr.slice(-2000)
      break
    }
    if (index < warmup) continue
    timings.push(tool.readElapsedMs?.(run.stdout) ?? run.wallMs)
    if (run.peakMemoryMb !== undefined) memories.push(run.peakMemoryMb)
    if (run.peakRssMb !== undefined) residents.push(run.peakRssMb)
    counts.push(count)
  }

  const errorCount = counts.at(-1)
  const result: Result = {
    case: caseName,
    tool: tool.name,
    title: tool.title,
    version: await tool.getVersion().catch(() => "unknown"),
    profile: dataset.profile,
    size: String(dataset.size),
    rows: dataset.rows,
    bytes: dataset.bytes,
    runs: timings.length,
    wallMs: timings.length ? summarize(timings) : undefined,
    peakMemoryMb: memories.length ? Math.max(...memories) : undefined,
    peakRssMb: residents.length ? Math.max(...residents) : undefined,
    outcome,
    exit,
    failedAfterMs,
    errorCount,
    expectedErrors,
    correct: outcome === "ok" && counts.every(count => count === expectedErrors),
    unsupported: tool.unsupported,
    caveats: tool.caveats,
    isPartial: tool.isPartial,
    stderr,
  }
  return result
}

function summarize(values: number[]) {
  const sorted = values.toSorted((left, right) => left - right)
  const middle = Math.floor(sorted.length / 2)
  const median =
    sorted.length % 2
      ? (sorted[middle] ?? 0)
      : ((sorted[middle - 1] ?? 0) + (sorted[middle] ?? 0)) / 2
  return { median, min: sorted[0] ?? 0, max: sorted.at(-1) ?? 0 }
}
