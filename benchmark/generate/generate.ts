import { createWriteStream, existsSync } from "node:fs"
import { mkdir, readFile, rename, stat, writeFile } from "node:fs/promises"
import { join } from "node:path"
import { once } from "node:events"
import type { Check, Fault, Profile } from "../profiles/types.ts"
import { createRandom } from "./random.ts"
import { type Size, formatBytes, parseSize } from "./size.ts"

export type Dataset = {
  profile: string
  size: Size
  seed: number
  rows: number
  bytes: number
  csvPath: string
  schemaPath: string
  /** The errors a correct validator reports, by check. */
  expected: Record<Check, number>
  faults: { rowNumber: number; column: string; check: Check }[]
}

const ESTIMATE_ROWS = 5000
const CHUNK_BYTES = 1 << 20

/** Writes the CSV, its Table Schema and a manifest, or reuses them when already generated. */
export async function generateDataset(options: {
  profile: Profile
  size: Size
  directory: string
  seed?: number
  faults?: number
}) {
  const { profile, size, directory, seed = 1, faults = 100 } = options
  const name = `${profile.name}-${typeof size === "number" ? `${size}rows` : size}-s${seed}-f${faults}`
  const manifestPath = join(directory, `${name}.manifest.json`)
  const csvPath = join(directory, `${name}.csv`)
  const schemaPath = join(directory, `${profile.name}.schema.json`)

  await mkdir(directory, { recursive: true })
  await writeFile(schemaPath, `${JSON.stringify(profile.tableSchema, null, 2)}\n`)

  if (existsSync(manifestPath) && existsSync(csvPath)) {
    const dataset: Dataset = JSON.parse(await readFile(manifestPath, "utf8"))
    // the stored paths go stale when the cache moves, as it does with a removed worktree
    if ((await stat(csvPath)).size === dataset.bytes)
      return { ...dataset, csvPath, schemaPath }
  }

  const rows = countRows(profile, size, seed)
  const plan = planFaults(profile, rows, Math.min(faults, rows), seed)
  const bytes = await writeCsv(profile, rows, seed, plan, csvPath)

  const dataset: Dataset = {
    profile: profile.name,
    size,
    seed,
    rows,
    bytes,
    csvPath,
    schemaPath,
    expected: countChecks(plan),
    faults: [...plan].map(([rowNumber, fault]) => ({
      rowNumber,
      column: fault.column,
      check: fault.check,
    })),
  }
  await writeFile(manifestPath, `${JSON.stringify(dataset, null, 2)}\n`)
  return dataset
}

export function describeDataset(dataset: Dataset) {
  return `${dataset.profile} ${formatBytes(dataset.bytes)} (${dataset.rows.toLocaleString("en")} rows)`
}

function countRows(profile: Profile, size: Size, seed: number) {
  const target = parseSize(size)
  if (target.rows !== undefined) return target.rows

  const random = createRandom(seed)
  let bytes = 0
  for (let row = 1; row <= ESTIMATE_ROWS; row++) {
    bytes += Buffer.byteLength(formatRow(profile, random, row))
  }
  const header = Buffer.byteLength(formatHeader(profile))
  return Math.max(1, Math.round(((target.bytes - header) * ESTIMATE_ROWS) / bytes))
}

/** Spreads the faults over the whole file, so a validator that stops early cannot pass. */
function planFaults(profile: Profile, rows: number, count: number, seed: number) {
  const random = createRandom(seed ^ 0x9e3779b9)
  const plan = new Map<number, Fault>()
  const stride = rows / count
  for (let index = 0; index < count; index++) {
    const start = Math.floor(index * stride)
    const end = Math.max(start, Math.floor((index + 1) * stride) - 1)
    const fault = profile.faults[index % profile.faults.length]
    plan.set(random.integer(start, end) + 1, fault)
  }
  return plan
}

async function writeCsv(
  profile: Profile,
  rows: number,
  seed: number,
  plan: Map<number, Fault>,
  path: string,
) {
  const random = createRandom(seed)
  const partial = `${path}.partial`
  const stream = createWriteStream(partial)
  const indexes = new Map(profile.columns.map((column, index) => [column.name, index]))

  let chunk = formatHeader(profile)
  let bytes = 0
  for (let row = 1; row <= rows; row++) {
    const fault = plan.get(row)
    if (fault) {
      const cells = profile.columns.map(column => column.generate(random, row))
      cells[indexes.get(fault.column) ?? -1] = fault.value
      chunk += `${cells.map(formatCell).join(",")}\n`
    } else {
      chunk += formatRow(profile, random, row)
    }

    if (chunk.length >= CHUNK_BYTES || row === rows) {
      bytes += Buffer.byteLength(chunk)
      if (!stream.write(chunk)) await once(stream, "drain")
      chunk = ""
    }
  }

  stream.end()
  await once(stream, "finish")
  await rename(partial, path)
  return bytes
}

function countChecks(plan: Map<number, Fault>) {
  const counts: Record<Check, number> = {
    type: 0,
    required: 0,
    range: 0,
    enum: 0,
    pattern: 0,
  }
  for (const fault of plan.values()) counts[fault.check]++
  return counts
}

function formatHeader(profile: Profile) {
  return `${profile.columns.map(column => formatCell(column.name)).join(",")}\n`
}

function formatRow(
  profile: Profile,
  random: ReturnType<typeof createRandom>,
  row: number,
) {
  return `${profile.columns.map(column => formatCell(column.generate(random, row))).join(",")}\n`
}

function formatCell(value: string) {
  return /[",\n\r]/.test(value) ? `"${value.replaceAll('"', '""')}"` : value
}
