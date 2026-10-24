import { formatBytes } from "../generate/size.ts"
import type { Result } from "../harness/measure.ts"
import { getProfile } from "../profiles/index.ts"

export type Machine = {
  cpu: string
  cpus: number
  memory: string
  os: string
  memoryMax?: string
}

export type Report = {
  measuredAt: string
  machine: Machine
  results: Result[]
}

const FAIRSPEC = new Set(["fairspec-python", "fairspec-typescript", "fairspec-rust"])

export function renderReport(report: Report) {
  const groups = groupResults(report.results)
  const headline = groups
    .filter(group => group.results.some(result => FAIRSPEC.has(result.tool)))
    .sort((left, right) => right.bytes - left.bytes)[0]
  const rivals = new Set(
    report.results
      .filter(result => !FAIRSPEC.has(result.tool) && !result.isPartial)
      .map(result => result.tool),
  ).size

  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Validation Benchmark</title>
<style>${STYLE}</style>
</head>
<body>
<main>
<header>
  <p class="eyebrow">Fairspec benchmark</p>
  <h1>How fast does a ${headline ? formatBytes(headline.bytes) : "large"} table validate?</h1>
  <p class="lede">Fairspec's Python and TypeScript libraries against ${rivals} other validators, all
  checking the same generated CSV against the same Table Schema. Measured
  ${escapeHtml(report.measuredAt.slice(0, 10))} on ${escapeHtml(describeMachine(report.machine))}.</p>
</header>
${headline ? renderHeadline(headline) : ""}
${groups.map(renderGroup).join("\n")}
${renderTools(report.results)}
${renderMethod()}
</main>
</body>
</html>
`
}

type Group = {
  key: string
  caseName: string
  size: string
  rows: number
  columns: number
  bytes: number
  results: Result[]
}

function groupResults(results: Result[]) {
  const groups = new Map<string, Group>()
  for (const result of results) {
    const key = `${result.case} ${result.size}`
    const group = groups.get(key) ?? {
      key,
      caseName: result.case,
      size: result.size,
      rows: result.rows,
      columns: getProfile(result.profile).columns.length,
      bytes: result.bytes,
      results: [],
    }
    group.results.push(result)
    groups.set(key, group)
  }
  return [...groups.values()].sort(
    (left, right) =>
      left.caseName.localeCompare(right.caseName) || left.bytes - right.bytes,
  )
}

function renderHeadline(group: Group) {
  const correct = group.results.filter(result => result.correct && result.wallMs)
  const fastestRival = correct
    .filter(result => !FAIRSPEC.has(result.tool) && !result.isPartial)
    .sort((left, right) => (left.wallMs?.median ?? 0) - (right.wallMs?.median ?? 0))[0]
  const failed = group.results.filter(result => result.outcome !== "ok").length
  const tiles = [
    ...group.results
      .filter(result => FAIRSPEC.has(result.tool))
      .map(result => [result.title, formatOutcome(result), formatMemory(result)]),
    fastestRival
      ? [`Fastest correct rival`, formatOutcome(fastestRival), fastestRival.title]
      : undefined,
    [
      `Did not finish`,
      `${failed} of ${group.results.length}`,
      "timed out, ran out of memory or crashed",
    ],
  ].filter(tile => tile !== undefined)

  return `<section aria-label="Headline">
  <p class="caption">${escapeHtml(group.caseName)}, ${formatBytes(group.bytes)} · ${formatShape(group)}</p>
  <div class="tiles">${tiles
    .map(
      ([label, value, note]) =>
        `<div class="tile"><span class="label">${escapeHtml(label ?? "")}</span><span class="value">${escapeHtml(value ?? "")}</span><span class="note">${escapeHtml(note ?? "")}</span></div>`,
    )
    .join("")}</div>
</section>`
}

function renderGroup(group: Group) {
  const results = group.results.toSorted(compareResults)
  const maxTime = Math.max(...results.map(result => result.wallMs?.median ?? 0), 1)
  const maxMemory = Math.max(...results.map(result => result.peakMemoryMb ?? 0), 1)

  return `<section>
  <h2>${escapeHtml(group.caseName)} · ${formatBytes(group.bytes)}</h2>
  <p class="caption">${formatShape(group)} · median of ${results[0]?.runs || 1} run(s) · shorter is better</p>
  <div class="legend"><span><i class="swatch fairspec"></i>Fairspec</span><span><i class="swatch rival"></i>Other tools</span><span><i class="swatch partial"></i>Not full validators</span></div>
  <div class="charts">
    ${renderBars("Time", results, result => result.wallMs?.median, maxTime, formatTime)}
    ${renderBars("Peak memory", results, result => result.peakMemoryMb, maxMemory, formatMemoryValue)}
  </div>
  <div class="table-wrap"><table>
    <thead><tr><th>Tool</th><th>Status</th><th class="num">Time</th><th class="num">Memory</th><th class="num">Errors found</th></tr></thead>
    <tbody>${results
      .map(
        result =>
          `<tr><td>${escapeHtml(formatTitle(result))}</td><td>${renderStatus(result)}</td><td class="num">${escapeHtml(formatOutcome(result))}</td><td class="num">${escapeHtml(formatMemory(result))}</td><td class="num">${result.errorCount ?? "–"} of ${result.expectedErrors}</td></tr>`,
      )
      .join("")}</tbody>
  </table></div>
</section>`
}

function renderBars(
  title: string,
  results: Result[],
  measure: (result: Result) => number | undefined,
  max: number,
  format: (value: number) => string,
) {
  const rows = results.map(result => {
    const value = measure(result)
    const kind = FAIRSPEC.has(result.tool)
      ? "fairspec"
      : result.isPartial
        ? "partial"
        : "rival"
    const bar =
      value === undefined
        ? `<span class="missing">${escapeHtml(formatOutcome(result))}</span>`
        : `<span class="bar ${kind}" style="width:${Math.max((value / max) * 100, 0.5).toFixed(1)}%"></span><span class="value">${escapeHtml(format(value))}</span>`
    return `<div class="row" data-tip="${escapeHtml(`${formatTitle(result)} ${result.version}: ${value === undefined ? formatOutcome(result) : format(value)}`)}"><span class="name">${escapeHtml(formatTitle(result))}</span><span class="track">${bar}</span></div>`
  })
  return `<figure><figcaption>${title}</figcaption>${rows.join("")}</figure>`
}

function renderStatus(result: Result) {
  if (result.outcome === "oom")
    return `<span class="chip critical">✕ out of memory</span>`
  if (result.outcome === "timeout")
    return `<span class="chip critical">✕ timed out</span>`
  if (result.outcome === "crash") return `<span class="chip critical">✕ crashed</span>`
  if (!result.correct) return `<span class="chip serious">≠ wrong count</span>`
  return `<span class="chip good">✓ correct</span>`
}

function renderTools(results: Result[]) {
  const tools = new Map(results.map(result => [result.tool, result]))
  return `<section>
  <h2>What each tool checks</h2>
  <p class="caption">Every tool reads the CSV as text and validates types, required values, ranges,
  enums, patterns and the primary key. Differences from Fairspec:</p>
  <div class="table-wrap"><table>
    <thead><tr><th>Tool</th><th>Version</th><th>Cannot check</th><th>Caveats</th></tr></thead>
    <tbody>${[...tools.values()]
      .toSorted(
        (left, right) =>
          Number(Boolean(left.isPartial)) - Number(Boolean(right.isPartial)),
      )
      .map(
        result =>
          `<tr><td>${escapeHtml(formatTitle(result))}</td><td>${escapeHtml(result.version)}</td><td>${escapeHtml(result.unsupported.join(", ") || "–")}</td><td>${result.caveats.map(escapeHtml).join("<br>") || "–"}</td></tr>`,
      )
      .join("")}</tbody>
  </table></div>
</section>`
}

function renderMethod() {
  return `<section>
  <h2>Method</h2>
  <ul>
    <li>Data is synthetic and seeded, so every run reads the same bytes. The <code>nyc-311</code>
    profile has the 41 columns of NYC 311 Service Requests with plausible value spreads.</li>
    <li>Each file carries a fixed number of bad cells spread over the whole file, each breaking
    exactly one check: a wrong type, a missing required value, an out-of-range number, a value
    outside an enum, and a pattern mismatch.</li>
    <li>A tool is <em>correct</em> when it reports exactly the bad cells it is able to detect.
    A fast wrong answer is not a result.</li>
    <li>Every column type in the schema is one Fairspec checks inside polars. Types it parses row
    by row (durations, geometries, JSON) would be slower, and are not measured here.</li>
    <li>Every tool runs as its own process from a warm file cache. Time is clocked by the script
    calling a library, after its imports; qsv and CSV Validator, two CLIs, are timed whole. Memory is
    the peak anonymous memory of the tool's processes, sampled every 50 ms: memory-mapped pages
    of the input file are excluded, since the kernel can drop them at any time. Each tool is
    capped at the memory the report header names; the machine was shared with other work.</li>
    <li>Reproduce: <code>pnpm -C benchmark setup:rivals</code>, then
    <code>pnpm -C benchmark measure --case nyc-311</code> and <code>pnpm -C benchmark show:report</code>.</li>
  </ul>
</section>`
}

function compareResults(left: Result, right: Result) {
  const leftTime = left.wallMs?.median ?? Number.POSITIVE_INFINITY
  const rightTime = right.wallMs?.median ?? Number.POSITIVE_INFINITY
  return (
    Number(Boolean(left.isPartial)) - Number(Boolean(right.isPartial)) ||
    leftTime - rightTime ||
    left.title.localeCompare(right.title)
  )
}

function formatTitle(result: Result) {
  return result.isPartial ? `${result.title} (not a full validator)` : result.title
}

function formatShape(group: Group) {
  const count = new Intl.NumberFormat("en", {
    notation: "compact",
    maximumSignificantDigits: 3,
  })
  return `${count.format(group.rows)} rows · ${group.columns} columns · ${count.format(group.rows * group.columns)} cells`
}

function formatOutcome(result: Result) {
  if (result.wallMs) return formatTime(result.wallMs.median)
  const after = result.failedAfterMs ? ` after ${formatTime(result.failedAfterMs)}` : ""
  if (result.outcome === "oom") return `out of memory${after}`
  if (result.outcome === "timeout") return `timed out${after}`
  return `crashed${after}`
}

function formatMemory(result: Result) {
  return result.peakMemoryMb === undefined ? "–" : formatMemoryValue(result.peakMemoryMb)
}

function formatMemoryValue(megabytes: number) {
  return megabytes >= 1024
    ? `${(megabytes / 1024).toFixed(1)} GB`
    : `${Math.round(megabytes)} MB`
}

function formatTime(milliseconds: number) {
  const seconds = milliseconds / 1000
  if (seconds < 60) return `${seconds.toFixed(seconds < 10 ? 2 : 1)} s`
  if (seconds < 3600)
    return `${Math.floor(seconds / 60)} min ${Math.round(seconds % 60)} s`
  return `${Math.floor(seconds / 3600)} h ${Math.round((seconds % 3600) / 60)} min`
}

function describeMachine(machine: Machine) {
  const cap = machine.memoryMax ? `, capped at ${machine.memoryMax} per tool` : ""
  return `${machine.cpu} (${machine.cpus} threads), ${machine.memory} RAM${cap}, ${machine.os}`
}

function escapeHtml(text: string) {
  return text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
}

const STYLE = `
:root {
  color-scheme: light;
  --ground: #f6f5f2; --panel: #fcfcfb; --ink: #0b0b0b; --muted: #52514e; --rule: #e2e1dc;
  --fairspec: #2a78d6; --rival: #b4b3ab; --partial: #dcdbd6;
  --good: #0ca30c; --serious: #ec835a; --critical: #d03b3b;
}
@media (prefers-color-scheme: dark) {
  :root:not([data-theme="light"]) {
    color-scheme: dark;
    --ground: #121211; --panel: #1a1a19; --ink: #f0efec; --muted: #c3c2b7; --rule: #383835;
    --fairspec: #3987e5; --rival: #6b6a64; --partial: #42413d;
  }
}
:root[data-theme="dark"] {
  color-scheme: dark;
  --ground: #121211; --panel: #1a1a19; --ink: #f0efec; --muted: #c3c2b7; --rule: #383835;
  --fairspec: #3987e5; --rival: #6b6a64; --partial: #42413d;
}
* { box-sizing: border-box; }
body { margin: 0; background: var(--ground); color: var(--ink);
  font: 16px/1.55 system-ui, -apple-system, "Segoe UI", sans-serif; }
main { max-width: 980px; margin: 0 auto; padding: 40px 16px 64px; }
h1 { font-size: clamp(1.6rem, 4vw, 2.3rem); line-height: 1.15; margin: 4px 0 12px; }
h2 { font-size: 1.25rem; margin: 0 0 4px; }
.eyebrow { margin: 0; color: var(--muted); text-transform: uppercase; letter-spacing: .08em; font-size: .78rem; }
.lede { color: var(--muted); max-width: 70ch; margin: 0; }
section { background: var(--panel); border: 1px solid var(--rule); border-radius: 12px; padding: 20px; margin-top: 20px; }
.caption { color: var(--muted); font-size: .9rem; margin: 0 0 12px; }
.tiles { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(190px, 100%), 1fr)); gap: 12px; }
.tile { border: 1px solid var(--rule); border-radius: 10px; padding: 14px; display: flex; flex-direction: column; gap: 2px; }
.tile .label { color: var(--muted); font-size: .85rem; }
.tile .value { font-size: 1.6rem; font-weight: 650; font-variant-numeric: tabular-nums; }
.tile .note { color: var(--muted); font-size: .85rem; }
.legend { display: flex; gap: 16px; font-size: .85rem; color: var(--muted); margin-bottom: 8px; }
.swatch { display: inline-block; width: 10px; height: 10px; border-radius: 2px; margin-right: 6px; vertical-align: -1px; }
.swatch.fairspec, .bar.fairspec { background: var(--fairspec); }
.swatch.rival, .bar.rival { background: var(--rival); }
.swatch.partial, .bar.partial { background: var(--partial); }
.charts { display: grid; grid-template-columns: repeat(auto-fit, minmax(min(300px, 100%), 1fr)); gap: 24px; margin-bottom: 16px; }
figure { margin: 0; min-width: 0; }
figcaption { font-weight: 600; font-size: .9rem; margin-bottom: 6px; }
.row { display: grid; grid-template-columns: minmax(0, min(11rem, 38%)) minmax(0, 1fr); gap: 8px; align-items: center; padding: 3px 0; position: relative; }
.row:hover { background: color-mix(in srgb, var(--rule) 45%, transparent); border-radius: 4px; }
.row:hover::after { content: attr(data-tip); position: absolute; left: 0; bottom: 100%; z-index: 1;
  background: var(--ink); color: var(--panel); font-size: .8rem; padding: 4px 8px; border-radius: 6px; white-space: nowrap; pointer-events: none; }
.name { font-size: .85rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.track { display: flex; align-items: center; gap: 6px; min-width: 0; }
.bar { height: 14px; border-radius: 0 4px 4px 0; flex: none; max-width: calc(100% - 4.5rem); }
.value, .missing { font-size: .8rem; color: var(--muted); font-variant-numeric: tabular-nums; white-space: nowrap; }
.missing { font-style: italic; }
.table-wrap { overflow-x: auto; }
table { width: 100%; border-collapse: collapse; font-size: .88rem; }
th, td { text-align: left; padding: 7px 8px; border-bottom: 1px solid var(--rule); vertical-align: top; }
th { color: var(--muted); font-weight: 600; }
.num { text-align: right; font-variant-numeric: tabular-nums; white-space: nowrap; }
.chip { display: inline-block; padding: 1px 8px; border-radius: 999px; font-size: .8rem; white-space: nowrap; border: 1px solid; }
.chip.good { color: var(--good); border-color: var(--good); }
.chip.serious { color: var(--serious); border-color: var(--serious); }
.chip.critical { color: var(--critical); border-color: var(--critical); }
code { font-size: .88em; }
ul { padding-left: 1.2rem; margin: 0; }
li + li { margin-top: 6px; }
`
