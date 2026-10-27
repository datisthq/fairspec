import { Buffer } from "node:buffer"
import { readdirSync, readFileSync } from "node:fs"
import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises"
import { tmpdir } from "node:os"
import { basename, dirname, join, resolve } from "node:path"
import type {
  DataSchema,
  Dataset,
  FileDialect,
  Resource,
  TableSchema,
} from "@fairspec/metadata"
import type { Table } from "@fairspec/table"
import { afterAll, describe, expect, it } from "vite-plus/test"
import {
  denormalizeDataset,
  describeFile,
  InferFileDialectOptions,
  Integrity,
  inferDataSchema,
  inferDataset,
  inferFileDialect,
  inferResourceName,
  inferTableSchema,
  inspectJson,
  loadDataset,
  loadTable,
  previewFile,
  queryDataset,
  queryTable,
  saveDataset,
  ValidateTableOptions,
  validateData,
  validateDataSchema,
  validateDataset,
  validateFile,
  validateTable,
  validateTableSchema,
  writeTempFile,
} from "../index.ts"

const SUITE = resolve(import.meta.dirname, "..", "..", "..", "conformance")
const TEST_SCHEMA = join(SUITE, "test-schema.json")

// "<operation>/<file>#<group>" to the open issue whose fix unblocks it; nothing else skips
const KNOWN_DIVERGENCES: Record<string, string> = {
  "table-query/order-sorting#a string categorical column with withOrder in ascending order":
    "https://github.com/datisthq/fairspec/issues/76",
  "table-query/order-sorting#a string categorical column with withOrder in descending order":
    "https://github.com/datisthq/fairspec/issues/76",
  "table-query/order-sorting#an integer categorical column with withOrder in ascending order":
    "https://github.com/datisthq/fairspec/issues/76",
  "table-query/order-sorting#an integer categorical column with withOrder in descending order":
    "https://github.com/datisthq/fairspec/issues/76",
  "table-query/order-sorting#an integer categorical column without withOrder":
    "https://github.com/datisthq/fairspec/issues/76",
  "table-query/order-sorting#an integer categorical column without withOrder in descending order":
    "https://github.com/datisthq/fairspec/issues/76",
  "table-query/order-sorting#a labelled string categorical column without withOrder":
    "https://github.com/datisthq/fairspec/issues/76",
}

const UNSPECIFIED_MESSAGE_ERRORS = ["data", "metadata", "cell/json"]
const TEMPORAL_FORMATS: Record<string, string> = {
  Date: "%Y-%m-%d",
  Time: "%H:%M:%S%.f",
  Datetime: "%Y-%m-%dT%H:%M:%S%.f",
  ZonedDatetime: "%Y-%m-%dT%H:%M:%S%.f%:z",
}

type Data = string | { base64: string } | unknown[] | Record<string, unknown>

type Group = {
  description: string
  format?: string
  inline?: boolean
  fileDialect?: FileDialect
  tableSchema?: TableSchema
  dataSchema?: DataSchema
  integrity?: Integrity
  textual?: boolean
  dataset?: Dataset
  datapackage?: Record<string, unknown>
  query?: string
  options?: Record<string, unknown>
  tests: Test[]
}

type Test = {
  description: string
  data: Data
  valid?: boolean
  errors?: unknown[]
  expected?: unknown
  throws?: true
}

const OPERATIONS: Record<string, (group: Group, test: Test) => Promise<unknown>> = {
  "data-infer-schema": async (group, test) =>
    inferDataSchema(await createResource(group, test)),
  "data-validate": async (group, test) => validateData(await createResource(group, test)),
  "data-validate-schema": async (_group, test) => {
    if (typeof test.data !== "object" || Array.isArray(test.data)) {
      throw new Error("data-validate-schema data must be a schema object")
    }
    const { valid, errors } = await validateDataSchema(test.data)
    return { valid, errors }
  },
  "dataset-copy": async (group, test) => {
    const path = await writeDataset(group, test)
    const target = join(dirname(path), "datapackage.json")
    await saveDataset(await loadDataset(path), { target })
    return JSON.parse(await readFile(target, "utf-8"))
  },
  "dataset-infer": async (group, test) => {
    const path = await writeDataset(group, test)
    const dataset = await inferDataset(
      await loadDataset(path),
      ValidateTableOptions.parse(group.options ?? {}),
    )
    return denormalizeDataset(dataset, { basepath: dirname(path) })
  },
  "dataset-list": async (group, test) => {
    const dataset = await loadDataset(await writeDataset(group, test))
    return (dataset.resources ?? []).map(
      (resource, index) =>
        resource.name ?? inferResourceName(resource, { resourceNumber: index + 1 }),
    )
  },
  "dataset-query": async (group, test) => {
    if (!group.query) return null
    return await readRows(
      await queryDataset(await writeDataset(group, test), group.query),
    )
  },
  "dataset-validate": async (group, test) =>
    validateDataset(await writeDataset(group, test)),
  "file-describe": async (group, test) =>
    describeFile(await writeData(group, test.data), {
      hashType: Integrity.shape.type.parse(group.options?.hashType ?? "sha256"),
    }),
  "file-preview": async (group, test) => {
    const { maxBytes, maxLines } = group.options ?? {}
    return previewFile(await writeData(group, test.data), {
      maxBytes: typeof maxBytes === "number" ? maxBytes : undefined,
      maxLines: typeof maxLines === "number" ? maxLines : undefined,
    })
  },
  "file-validate": async (group, test) => validateFile(await createResource(group, test)),
  "table-infer-dialect": async (group, test) =>
    inferFileDialect(
      await createResource(group, test),
      InferFileDialectOptions.parse(group.options ?? {}),
    ),
  "table-infer-schema": async (group, test) =>
    inferTableSchema(
      await createResource(group, test),
      ValidateTableOptions.parse(group.options ?? {}),
    ),
  "table-load": async (group, test) => {
    const table = await loadTable(
      await createResource(group, test),
      ValidateTableOptions.parse(group.options ?? {}),
    )
    return table ? await readRows(table) : null
  },
  "table-query": async (group, test) => {
    const table = await loadTable(
      await createResource(group, test),
      ValidateTableOptions.parse(group.options ?? {}),
    )
    if (!table || !group.query) return null
    return await readRows(queryTable(table, group.query))
  },
  "table-validate-schema": async (_group, test) => {
    if (typeof test.data !== "object" || Array.isArray(test.data)) {
      throw new Error("table-validate-schema data must be a schema object")
    }
    const { valid, errors } = await validateTableSchema(test.data)
    return { valid, errors }
  },
  "table-validate": async (group, test) =>
    validateTable(
      await createResource(group, test),
      ValidateTableOptions.parse(group.options ?? {}),
    ),
}

const tempDirs: string[] = []
afterAll(async () => {
  await Promise.all(tempDirs.map(dir => rm(dir, { recursive: true, force: true })))
})

for (const operation of readdirSync(join(SUITE, "tests")).sort()) {
  const run = OPERATIONS[operation]

  describe(operation, () => {
    it("is a known operation", () => {
      expect(Object.keys(OPERATIONS)).toContain(operation)
    })

    for (const file of readdirSync(join(SUITE, "tests", operation)).sort()) {
      const path = join(SUITE, "tests", operation, file)
      const groups: Group[] = JSON.parse(readFileSync(path, "utf-8"))

      describe(basename(file, ".json"), () => {
        it("matches the test schema", async () => {
          expect(await inspectJson(groups, { jsonSchema: TEST_SCHEMA })).toEqual([])
        })

        for (const group of groups) {
          const issue =
            KNOWN_DIVERGENCES[
              `${operation}/${basename(file, ".json")}#${group.description}`
            ]

          const runTest = issue ? it.skip : it

          describe(group.description, () => {
            for (const test of group.tests) {
              const name = issue ? `${test.description} (${issue})` : test.description
              runTest(name, async () => {
                if (!run) return

                if (test.throws) {
                  await expect(run(group, test)).rejects.toThrow()
                  return
                }

                const result = canonicalize(await run(group, test))

                if ("expected" in test) {
                  expect(result).toStrictEqual(test.expected)
                }

                if (test.valid !== undefined) {
                  expect(result).toMatchObject({ valid: test.valid })
                  if (test.errors || test.valid) {
                    expect(result).toStrictEqual({
                      valid: test.valid,
                      errors: test.errors ?? [],
                    })
                  }
                }
              })
            }
          })
        }
      })
    }
  })
}

async function createResource(group: Group, test: Test) {
  const resource: Resource = {
    data: group.inline ? test.data : await writeData(group, test.data),
    fileDialect: group.fileDialect,
    tableSchema: group.tableSchema,
    dataSchema: group.dataSchema,
    integrity: group.integrity,
    textual: group.textual,
  }
  return resource
}

async function writeData(group: Group, data: Data) {
  return await writeTempFile(encodeData(data), { format: group.format ?? "csv" })
}

async function writeDataset(group: Group, test: Test) {
  const dir = await mkdtemp(join(tmpdir(), "fairspec-conformance-"))
  tempDirs.push(dir)

  if (typeof test.data !== "object" || Array.isArray(test.data)) {
    throw new Error("dataset data must map file names to contents")
  }

  for (const [name, content] of Object.entries(test.data)) {
    await writeFile(join(dir, name), encodeData(content))
  }

  const path = join(dir, group.datapackage ? "datapackage.json" : "dataset.json")
  await writeFile(path, JSON.stringify(group.datapackage ?? group.dataset))
  return path
}

async function readRows(table: Table) {
  const frame = await table.collect()
  const columns = frame.columns.map(name => {
    const dtype = frame.getColumn(name).dtype
    const isZoned =
      "timeZone" in dtype && typeof dtype.timeZone === "string" && !!dtype.timeZone
    const format = TEMPORAL_FORMATS[isZoned ? "ZonedDatetime" : dtype.variant]
    return format ? `STRFTIME("${name}", '${format}') AS "${name}"` : `"${name}"`
  })
  const rendered = await queryTable(
    frame.lazy(),
    `SELECT ${columns.join(", ")} FROM self`,
  ).collect()
  return { columns: rendered.columns, rows: rendered.rows() }
}

function encodeData(data: unknown) {
  if (typeof data === "string") return data
  if (Array.isArray(data)) return JSON.stringify(data)
  if (
    typeof data === "object" &&
    data &&
    "base64" in data &&
    typeof data.base64 === "string"
  ) {
    return Buffer.from(data.base64, "base64")
  }
  throw new Error("data must be a string, base64 object or array")
}

function canonicalize(value: unknown): unknown {
  const json = value === undefined ? null : JSON.parse(JSON.stringify(value))
  if (json && typeof json === "object" && Array.isArray(json.errors)) {
    for (const error of json.errors) {
      if (UNSPECIFIED_MESSAGE_ERRORS.includes(error.type)) delete error.message
    }
  }
  return json
}
