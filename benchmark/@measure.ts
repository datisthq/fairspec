import { mkdir, writeFile } from "node:fs/promises"
import { cpus, release, totalmem } from "node:os"
import { join } from "node:path"
import { parseArgs } from "node:util"
import { describeDataset, generateDataset } from "./generate/generate.ts"
import { formatBytes } from "./generate/size.ts"
import { loadCases } from "./harness/case.ts"
import type { Result } from "./harness/measure.ts"
import { measureTool } from "./harness/measure.ts"
import { getProfile } from "./profiles/index.ts"
import { selectTools } from "./tools/index.ts"

const { values } = parseArgs({
  options: {
    case: { type: "string", multiple: true },
    size: { type: "string", multiple: true },
    tool: { type: "string", multiple: true },
    runs: { type: "string", default: "3" },
    warmup: { type: "string", default: "1" },
    timeout: { type: "string", default: "3600" },
    "memory-max": { type: "string" },
  },
})

const CACHE = join(import.meta.dirname, ".cache")
const startedAt = new Date().toISOString()
const resultsPath = join(CACHE, "results", `${startedAt.replaceAll(":", "-")}.json`)
const machine = {
  cpu: cpus()[0]?.model.trim() ?? "unknown",
  cpus: cpus().length,
  memory: formatBytes(totalmem()),
  os: `${process.platform} ${release()}`,
  memoryMax: values["memory-max"],
}
const results: Result[] = []
await mkdir(join(CACHE, "results"), { recursive: true })

for (const benchmarkCase of await loadCases(values.case)) {
  const sizes = benchmarkCase.sizes.filter(
    size => !values.size?.length || values.size.includes(String(size)),
  )
  for (const size of sizes) {
    const dataset = await generateDataset({
      profile: getProfile(benchmarkCase.profile),
      size,
      directory: join(CACHE, "data"),
      seed: benchmarkCase.seed,
      faults: benchmarkCase.faults,
    })
    console.log(`\n${benchmarkCase.name}: ${describeDataset(dataset)}`)

    for (const tool of selectTools(values.tool)) {
      const result = await measureTool({
        caseName: benchmarkCase.name,
        tool,
        dataset,
        runs: Number(values.runs),
        warmup: Number(values.warmup),
        timeoutMs: Number(values.timeout) * 1000,
        memoryMax: values["memory-max"],
      })
      results.push(result)
      await writeFile(
        resultsPath,
        `${JSON.stringify({ startedAt, machine, results }, null, 2)}\n`,
      )
      console.log(formatResult(result))
    }
  }
}

console.log(`\n${resultsPath}`)

function formatResult(result: Result) {
  const time = result.wallMs
    ? `${(result.wallMs.median / 1000).toFixed(2)}s`
    : result.outcome
  const memory = result.peakMemoryMb ? `${Math.round(result.peakMemoryMb)}MB` : "-"
  const errors = `${result.errorCount ?? "-"}/${result.expectedErrors}`
  const verdict =
    result.outcome !== "ok"
      ? `  ${result.outcome.toUpperCase()} (${result.exit})`
      : result.correct
        ? ""
        : "  WRONG COUNT"
  return `  ${result.title.padEnd(28)} ${time.padStart(9)} ${memory.padStart(8)} ${errors.padStart(9)}${verdict}`
}
