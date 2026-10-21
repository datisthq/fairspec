import { execFile } from "node:child_process"
import { readFile, writeFile } from "node:fs/promises"
import { basename, join } from "node:path"
import { promisify } from "node:util"
import type { Tool } from "./types.ts"

export const CSV_VALIDATOR_VERSION = "1.4.4"
export const CSV_VALIDATOR_URL = `https://github.com/digital-preservation/csv-validator/releases/download/${CSV_VALIDATOR_VERSION}/csv-validator-distribution-${CSV_VALIDATOR_VERSION}-bin.zip`
export const CSV_VALIDATOR_PATH = join(
  import.meta.dirname,
  "..",
  ".cache",
  "bin",
  `csv-validator-${CSV_VALIDATOR_VERSION}`,
  `csv-validator-cmd-${CSV_VALIDATOR_VERSION}.jar`,
)

const INTEGER_PATTERN = "^[+-]?[0-9]+$"
const NUMBER_PATTERN = "^[+-]?([0-9]+(\\.[0-9]*)?|\\.[0-9]+)([eE][+-]?[0-9]+)?$"
const BOOLEAN_VALUES = ["true", "false"]
const DATE_TIME_PATTERN = "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}$"
const ERROR_CELL = /^Error:.* fails for row: (\d+), column: (.*?), value: /

export const csvValidatorTool: Tool = {
  name: "csv-validator",
  title: "CSV Validator",
  unsupported: [],
  caveats: [
    "local-date-time is matched by regex, as are integer, number and boolean",
    "errors are counted once per cell; a bad number fails both its type regex and its range",
  ],
  getVersion: async () => {
    const { stdout } = await promisify(execFile)("java", [
      "-jar",
      CSV_VALIDATOR_PATH,
      "--help",
    ])
    return /Command Line (\S+)/.exec(stdout)?.[1] ?? ""
  },
  createCommand: async dataset => {
    const schemaPath = dataset.schemaPath.replace(/\.schema\.json$/, ".csvs")
    const tableSchema = JSON.parse(await readFile(dataset.schemaPath, "utf8"))
    await writeFile(schemaPath, convertTableSchema(tableSchema))

    return {
      file: "java",
      args: [
        "-jar",
        CSV_VALIDATOR_PATH,
        "--fail-fast",
        "false",
        basename(dataset.csvPath),
        basename(schemaPath),
      ],
      okExitCodes: [0, 3],
    }
  },
  countErrors: async ({ stdout }) => {
    const lines = stdout.split("\n").filter(line => line.startsWith("Error:"))
    // every failing rule of a cell prints its own line
    return new Set(
      lines.map(line => {
        const match = ERROR_CELL.exec(line)
        return match ? `${match[1]}\t${match[2]}` : line
      }),
    ).size
  },
}

export function convertTableSchema(tableSchema: {
  properties: Record<string, Record<string, unknown>>
  required?: string[]
  primaryKey?: string[]
}) {
  const required = new Set(tableSchema.required ?? [])
  const primaryKey = tableSchema.primaryKey ?? []
  const names = Object.keys(tableSchema.properties)
  const lines = ["version 1.1", `@totalColumns ${names.length}`]

  for (const [name, property] of Object.entries(tableSchema.properties)) {
    const type = [property.type].flat().find(item => item !== "null")
    const rules: string[] = []
    if (required.has(name)) rules.push("notEmpty")

    if (type === "integer") rules.push(`regex(${quote(INTEGER_PATTERN)})`)
    if (type === "number") rules.push(`regex(${quote(NUMBER_PATTERN)})`)
    if (type === "boolean") rules.push(`any(${BOOLEAN_VALUES.map(quote).join(",")})`)
    if (property.format === "local-date-time") {
      rules.push(`regex(${quote(DATE_TIME_PATTERN)})`)
    }
    if (property.format === "date-time") rules.push("xDateTimeTz")

    const { minimum, maximum, minLength, maxLength, pattern, enum: values } = property
    if (typeof minimum === "number" || typeof maximum === "number") {
      rules.push(`range(${formatBound(minimum)},${formatBound(maximum)})`)
    }
    if (typeof minLength === "number" || typeof maxLength === "number") {
      rules.push(`length(${formatBound(minLength)},${formatBound(maxLength)})`)
    }
    if (typeof pattern === "string") rules.push(`regex(${quote(anchorPattern(pattern))})`)
    if (Array.isArray(values))
      rules.push(`any(${values.map(String).map(quote).join(",")})`)

    if (primaryKey.length === 1 && primaryKey[0] === name) rules.push("unique")
    if (primaryKey.length > 1 && primaryKey[0] === name) {
      rules.push(
        `unique(${primaryKey.map(key => `$${formatIdentifier(key)}`).join(",")})`,
      )
    }

    if (!required.has(name)) rules.push("@optional")
    lines.push(`${formatIdentifier(name)}: ${rules.join(" ")}`.trimEnd())
  }
  return `${lines.join("\n")}\n`
}

function formatBound(value: unknown) {
  return typeof value === "number" ? String(value) : "*"
}

// a JSON Schema pattern searches, while CSV Schema's regex must match the whole value
function anchorPattern(pattern: string) {
  if (pattern.startsWith("^") && pattern.endsWith("$")) return pattern
  return `(?s).*(?:${pattern}).*`
}

// CSV Schema string literals have no escapes
function quote(value: string) {
  if (value.includes('"')) throw new Error(`CSV Schema cannot quote ${value}`)
  return `"${value}"`
}

function formatIdentifier(name: string) {
  return /^[A-Za-z0-9_\-.]+$/.test(name) ? name : quote(name)
}
