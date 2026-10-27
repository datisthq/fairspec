import { mkdir, writeFile } from "node:fs/promises"
import { join } from "node:path"
import {
  FairspecException,
  resolveFileDialect,
  resolveTableSchema,
} from "@fairspec/metadata"
import { describe, expect, it } from "vite-plus/test"
import { getTempFolderPath } from "../../../../../actions/folder/temp.ts"
import { loadDatasetFromFrictionless } from "../load.ts"

describe("loadDatasetFromFrictionless", () => {
  it("should load a package with schema and dialect paths", async () => {
    const path = await writePackage({
      "datapackage.json": {
        resources: [
          {
            name: "my-items",
            path: "items.csv",
            schema: "schema.json",
            dialect: "dialect.json",
          },
        ],
      },
      "schema.json": { fields: [{ name: "id", type: "integer" }] },
      "dialect.json": { delimiter: ";" },
    })

    const dataset = await loadDatasetFromFrictionless(path)
    const resource = dataset.resources?.[0]

    expect(resource?.name).toBe("my_items")
    expect(await resolveFileDialect(resource?.fileDialect)).toEqual({
      format: "csv",
      delimiter: ";",
    })
    expect(await resolveTableSchema(resource?.tableSchema)).toEqual({
      properties: { id: { type: ["integer", "null"] } },
    })
  })

  it("should load a Fairspec dataset named datapackage.json as it is", async () => {
    const path = await writePackage({
      "datapackage.json": {
        $schema: "https://fairspec.org/profiles/latest/dataset.json",
        resources: [{ name: "items", data: [{ id: 1 }] }],
      },
    })

    const dataset = await loadDatasetFromFrictionless(path)

    expect(dataset.resources).toEqual([{ name: "items", data: [{ id: 1 }] }])
  })

  it("should raise metadata errors for an invalid package", async () => {
    const path = await writePackage({
      "datapackage.json": { resources: [{ bytes: "x" }] },
    })

    const error = await loadDatasetFromFrictionless(path).catch(error => error)

    expect(error).toBeInstanceOf(FairspecException)
    expect(error.report.errors).toEqual([
      expect.objectContaining({ type: "metadata", jsonPointer: "/resources/0/bytes" }),
    ])
  })
})

async function writePackage(files: Record<string, unknown>) {
  const folder = getTempFolderPath()
  await mkdir(folder, { recursive: true })
  for (const [name, content] of Object.entries(files)) {
    await writeFile(join(folder, name), JSON.stringify(content))
  }
  return join(folder, "datapackage.json")
}
