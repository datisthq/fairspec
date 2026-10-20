import { execFile } from "node:child_process"
import { readFile, rm, symlink, writeFile } from "node:fs/promises"
import { basename, join } from "node:path"
import { promisify } from "node:util"
import type { Tool } from "./types.ts"

export const QSV_VERSION = "24.0.0"
export const QSV_PATH = join(
  import.meta.dirname,
  "..",
  ".cache",
  "bin",
  `qsv-${QSV_VERSION}`,
  "qsv",
)

const DATE_TIME_PATTERN = "^\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}$"

export const qsvTool: Tool = {
  name: "qsv",
  title: "qsv",
  unsupported: [],
  caveats: [
    "date-time is matched by regex, since JSON Schema date-time requires a time zone",
    "its copy of the valid rows is written to /dev/null, not to disk",
    "errors are counted once per cell; qsv reports an empty required enum cell twice",
  ],
  getVersion: async () => {
    const { stdout } = await promisify(execFile)(QSV_PATH, ["--version"])
    return stdout.split(/[\s-]/)[1] ?? ""
  },
  createCommand: async dataset => {
    const schemaPath = dataset.schemaPath.replace(/\.schema\.json$/, ".qsv.json")
    const tableSchema = JSON.parse(await readFile(dataset.schemaPath, "utf8"))
    await writeFile(
      schemaPath,
      `${JSON.stringify(convertTableSchema(tableSchema), null, 2)}\n`,
    )

    const valid = `${dataset.csvPath}.valid`
    await rm(valid, { force: true })
    await symlink("/dev/null", valid)

    return {
      file: QSV_PATH,
      args: ["validate", basename(dataset.csvPath), basename(schemaPath)],
      outputs: [
        valid,
        `${dataset.csvPath}.invalid`,
        `${dataset.csvPath}.validation-errors.tsv`,
      ],
    }
  },
  countErrors: async ({ dataset }) => {
    const path = `${dataset.csvPath}.validation-errors.tsv`
    const text = await readFile(path, "utf8").catch(() => "")
    const lines = text
      .split("\n")
      .filter(line => line && !line.startsWith("row_number\t"))
    // an empty required enum cell fails both type and enum; others report one error per cell
    return new Set(lines.map(line => line.split("\t", 2).join("\t"))).size
  },
}

export function convertTableSchema(tableSchema: {
  properties: Record<string, Record<string, unknown>>
  required?: string[]
  primaryKey?: string[]
}) {
  const required = new Set(tableSchema.required ?? [])
  const properties: Record<string, Record<string, unknown>> = {}
  for (const [name, property] of Object.entries(tableSchema.properties)) {
    const types = [property.type].flat().filter(type => type !== "null")
    const { format, ...rest } = property
    properties[name] = {
      ...rest,
      type: required.has(name) ? types[0] : [...types, "null"],
      ...(format === "date-time" ? { pattern: DATE_TIME_PATTERN } : {}),
    }
  }
  return {
    $schema: "https://json-schema.org/draft/2020-12/schema",
    type: "object",
    properties,
    required: Object.keys(properties),
    ...(tableSchema.primaryKey ? { uniqueCombinedWith: tableSchema.primaryKey } : {}),
  }
}
