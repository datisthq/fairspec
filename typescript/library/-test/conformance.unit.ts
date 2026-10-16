import { Buffer } from "node:buffer"
import { readdirSync, readFileSync } from "node:fs"
import { mkdtemp, rm, writeFile } from "node:fs/promises"
import { tmpdir } from "node:os"
import { basename, join, resolve } from "node:path"
import type { Dataset, FileDialect, Integrity, TableSchema } from "@fairspec/metadata"
import { afterAll, describe, expect, it } from "vite-plus/test"
import {
  describeFile,
  InferFileDialectOptions,
  inferFileDialect,
  inferTableSchema,
  inspectJson,
  loadTable,
  ValidateTableOptions,
  validateDataset,
  validateFile,
  validateTable,
  writeTempFile,
} from "../index.ts"

const SUITE = resolve(import.meta.dirname, "..", "..", "..", "conformance")
const TEST_SCHEMA = join(SUITE, "test-schema.json")

type Data = string | { base64: string } | unknown[] | Record<string, string>

type Group = {
  description: string
  format?: string
  fileDialect?: FileDialect
  tableSchema?: TableSchema
  integrity?: Integrity
  dataset?: Dataset
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
  "file-describe": async (group, test) => describeFile(await writeData(group, test.data)),
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
    if (!table) return null
    const frame = await table.collect()
    return { columns: frame.columns, rows: frame.rows() }
  },
  "table-validate": async (group, test) =>
    validateTable(
      await createResource(group, test),
      ValidateTableOptions.parse(group.options ?? {}),
    ),
  "dataset-validate": async (group, test) =>
    validateDataset(await writeDataset(group, test)),
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
          describe(group.description, () => {
            for (const test of group.tests) {
              it(test.description, async () => {
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
  return {
    data: await writeData(group, test.data),
    fileDialect: group.fileDialect,
    tableSchema: group.tableSchema,
    integrity: group.integrity,
  }
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

  const path = join(dir, "dataset.json")
  await writeFile(path, JSON.stringify(group.dataset))
  return path
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
  return value === undefined ? null : JSON.parse(JSON.stringify(value))
}
