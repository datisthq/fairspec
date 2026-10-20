import { execSync, spawn, spawnSync } from "node:child_process"
import { mkdtempSync, rmSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"

const ROOT = join(import.meta.dirname, "../../..")
const SAMPLES = join(import.meta.dirname, "samples")
const OUT = join(ROOT, ".livemark/public/screenshots")
const CHROME = process.env.CHROME ?? "google-chrome"
const FAIRSPEC = `uv run --quiet --project ${join(ROOT, "python")} fairspec`

type Line = string

const palette = {
  light: {
    bg: "#ffffff",
    text: "#4c4f69",
    dim: "#8c8fa1",
    mauve: "#8839ef",
    blue: "#1e66f5",
    green: "#40a02b",
    peach: "#fe640b",
    teal: "#179299",
    red: "#d20f39",
    yellow: "#df8e1d",
  },
  dark: {
    bg: "#0a0a0a",
    text: "#cdd6f4",
    dim: "#7f849c",
    mauve: "#cba6f7",
    blue: "#89b4fa",
    green: "#a6e3a1",
    peach: "#fab387",
    teal: "#94e2d5",
    red: "#f38ba8",
    yellow: "#f9e2af",
  },
}

const slides: Record<string, Line[]> = {
  standard: [
    ...command("cat dataset.json", "json"),
    ...command("fairspec dataset validate dataset.json", "report"),
  ],
  validate: [
    ...command("fairspec table validate broken.csv --schema schema.json", "report"),
    ...command(
      "fairspec table validate broken.csv --schema schema.json --json | jq -c '.errors[]'",
      "jsonl",
    ),
  ],
  infer: [
    ...command(
      "fairspec table infer-schema members.csv --json | jq -c .properties",
      "jsonl",
    ),
    ...command("fairspec table validate members.csv --schema schema.json", "report"),
  ],
  preview: command("fairspec table preview members.csv", "raw"),
  query: command(
    `fairspec table query members.csv "SELECT name, score FROM self WHERE score > 80"`,
    "raw",
  ),
  "mcp-validate": await agent({
    prompt: "Check broken.csv against schema.json.",
    intro: "Validating the table with the Fairspec MCP server.",
    tool: "validateTableTool",
    args: { resource: { data: "broken.csv", tableSchema: "schema.json" } },
    outro:
      "Two cells fail: id is not an integer in row 3, and 2024-02-30 is not a date in row 2.",
  }),
  "mcp-infer": await agent({
    prompt: "Describe the columns of members.csv.",
    intro: "Inferring a table schema from the file.",
    tool: "inferTableSchemaTool",
    args: { resource: { data: "members.csv" } },
    outro: "Four columns: an integer id, a name, a joined date and a numeric score.",
  }),
  "mcp-query": await agent({
    prompt: "Who in members.csv scored above 80?",
    intro: "Querying the table with SQL.",
    tool: "queryTableTool",
    args: {
      resource: { data: "members.csv" },
      query: "SELECT name, score FROM self WHERE score > 80",
    },
    outro: "Ada with 91.5 and Grace with 88.",
  }),
  skills: skills(),
}

const work = mkdtempSync(join(tmpdir(), "fairspec-screens-"))
for (const [id, lines] of Object.entries(slides)) {
  for (const [theme, colors] of Object.entries(palette)) {
    const name = theme === "dark" ? `${id}-dark` : id
    const html = join(work, `${name}.html`)
    const png = join(work, `${name}.png`)
    writeFileSync(html, renderPage(lines, colors))
    execSync(
      `${CHROME} --headless=new --hide-scrollbars --force-device-scale-factor=2 --window-size=960,600 --virtual-time-budget=4000 --screenshot=${png} file://${html}`,
      { stdio: "ignore" },
    )
    execSync(`convert ${png} -quality 90 ${join(OUT, `${name}.webp`)}`)
  }
}
rmSync(work, { recursive: true })

function command(cmd: string, kind: "json" | "jsonl" | "report" | "raw"): Line[] {
  const out = run(cmd)
  const body =
    kind === "json"
      ? highlightJson(out)
      : kind === "jsonl"
        ? out.split("\n").flatMap(line => highlightJson(wrapJson(line, 74).join("\n")))
        : kind === "report"
          ? out.split("\n").map(highlightReport)
          : [`<span class="tight">${escapeHtml(out)}</span>`]
  return [renderPrompt(cmd), ...body, ""]
}

async function agent(props: {
  prompt: string
  intro: string
  tool: string
  args: Record<string, unknown>
  outro: string
}): Promise<Line[]> {
  const result = await callTool(props.tool, props.args)
  const args = JSON.stringify(props.args)
  return [
    `${span("mauve b", "›")} ${span("b", props.prompt)}`,
    "",
    say(props.intro),
    `  ${span("blue b", "fairspec")} ${span("dim", "·")} ${span("teal", props.tool)}`,
    ...wrapJson(args, 70).map(part => `    ${highlightJson(part)[0]}`),
    ...wrapJson(JSON.stringify(result), 70).map(
      (part, i) => `  ${span("dim", i === 0 ? "⎿ " : "  ")}${highlightJson(part)[0]}`,
    ),
    "",
    ...wrapText(props.outro, 74).map((part, i) =>
      i === 0 ? say(part) : `  ${escapeHtml(part)}`,
    ),
  ]
}

function skills(): Line[] {
  const cmd = "fairspec table validate broken.csv --schema schema.json"
  return [
    `${span("mauve b", "›")} ${span("b", "Why does broken.csv fail its schema?")}`,
    "",
    say("Loading the fairspec-validation skill."),
    `  ${span("blue b", "Skill")}${span("dim", "(")}${span("teal", "fairspec-validation")}${span("dim", ")")}`,
    "",
    say("Running the check it recommends."),
    `  ${renderPrompt(cmd)}`,
    ...run(cmd)
      .split("\n")
      .map(line => `  ${span("dim", "⎿ ")}${highlightReport(line)}`),
    "",
    say("Both are cell/type errors: fix the value, or the schema's type or format."),
  ]
}

function run(cmd: string) {
  return spawnSync("bash", ["-c", cmd.replace(/^fairspec/, FAIRSPEC)], {
    cwd: SAMPLES,
    encoding: "utf8",
    env: { ...process.env, COLUMNS: "200" },
  })
    .stdout.split("\n")
    .map(line => line.trimEnd())
    .join("\n")
    .trim()
}

async function callTool(name: string, args: Record<string, unknown>) {
  const server = spawn("bash", ["-c", `${FAIRSPEC} mcp`], {
    cwd: SAMPLES,
    stdio: ["pipe", "pipe", "ignore"],
  })
  const pending = new Map<
    number,
    (message: { result: { structuredContent: unknown } }) => void
  >()
  let buffer = ""
  server.stdout.on("data", chunk => {
    buffer += chunk
    for (let i = buffer.indexOf("\n"); i >= 0; i = buffer.indexOf("\n")) {
      const line = buffer.slice(0, i)
      buffer = buffer.slice(i + 1)
      if (!line.trim()) continue
      const message = JSON.parse(line)
      pending.get(message.id)?.(message)
    }
  })
  let id = 0
  const request = (method: string, params: unknown) =>
    new Promise<{ result: { structuredContent: unknown } }>(resolve => {
      id += 1
      pending.set(id, resolve)
      server.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`)
    })
  await request("initialize", {
    protocolVersion: "2025-06-18",
    capabilities: {},
    clientInfo: { name: "landing-screenshots", version: "0" },
  })
  server.stdin.write(
    `${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`,
  )
  const response = await request("tools/call", { name, arguments: args })
  server.kill()
  return response.result.structuredContent
}

function renderPage(lines: Line[], colors: Record<string, string>) {
  const css = Object.entries(colors)
    .map(([key, value]) => `.${key}{color:${value}}`)
    .join("")
  return `<!doctype html><html><head><meta charset="utf-8">
<link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;700&display=swap" rel="stylesheet">
<style>
html,body{margin:0;width:960px;height:600px;overflow:hidden;background:${colors.bg};color:${colors.text}}
pre{margin:0;padding:26px 30px;font:13px/1.6 "JetBrains Mono",ui-monospace,monospace;white-space:pre;font-variant-ligatures:none}
.b{font-weight:700}.tight{display:block;line-height:1.2}${css}
</style></head><body><pre>${lines.join("\n").trimEnd()}</pre></body></html>`
}

function renderPrompt(cmd: string) {
  const tokens = (cmd.match(/"[^"]*"|'[^']*'|\s+|[^\s"']+/g) ?? []).map((token, i) => {
    if (i === 0) return span("blue b", token)
    if (token.startsWith("-")) return span("teal", token)
    if (token.startsWith('"') || token.startsWith("'")) return span("green", token)
    if (token === "|") return span("dim", token)
    if (token === "jq") return span("blue", token)
    return escapeHtml(token)
  })
  return `${span("red b", "$")} ${tokens.join("")}`
}

function highlightReport(line: string) {
  if (line.startsWith("✔")) return `${span("green b", "✔")}${escapeHtml(line.slice(1))}`
  if (line.startsWith("✖")) return `${span("red b", "✖")}${escapeHtml(line.slice(1))}`
  return escapeHtml(line)
}

function highlightJson(text: string): Line[] {
  const pattern =
    /("(?:\\.|[^"\\])*")(\s*:)?|(-?\d+(?:\.\d+)?|true|false|null)|([{}[\],:])/g
  return text.split("\n").map(line => {
    let out = ""
    let last = 0
    for (const match of line.matchAll(pattern)) {
      out += escapeHtml(line.slice(last, match.index))
      if (match[1] && match[2]) out += span("blue", match[1]) + span("dim", match[2])
      else if (match[1]) out += span("green", match[1])
      else if (match[3]) out += span("peach", match[3])
      else out += span("dim", match[0])
      last = (match.index ?? 0) + match[0].length
    }
    return out + escapeHtml(line.slice(last))
  })
}

function wrapJson(text: string, width: number) {
  const lines: string[] = []
  let line = ""
  for (const part of text.split(/(?<=,)/)) {
    if (line && line.length + part.length > width) {
      lines.push(line)
      line = ""
    }
    line += part
  }
  return [...lines, line]
}

function wrapText(text: string, width: number) {
  const lines: string[] = []
  let line = ""
  for (const word of text.split(" ")) {
    if (line && line.length + word.length + 1 > width) {
      lines.push(line)
      line = ""
    }
    line = line ? `${line} ${word}` : word
  }
  return [...lines, line]
}

function say(text: string) {
  return `${span("peach", "●")} ${escapeHtml(text)}`
}

function span(cls: string, text: string) {
  return `<span class="${cls}">${escapeHtml(text)}</span>`
}

function escapeHtml(text: string) {
  return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;")
}
