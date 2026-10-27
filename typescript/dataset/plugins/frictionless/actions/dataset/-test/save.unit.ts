import { mkdir, readFile, writeFile } from "node:fs/promises"
import { join } from "node:path"
import { describe, expect, it } from "vite-plus/test"
import { getTempFolderPath } from "../../../../../actions/folder/temp.ts"
import { loadDatasetFromFrictionless } from "../load.ts"
import { saveDatasetToFrictionless } from "../save.ts"

describe("saveDatasetToFrictionless", () => {
  it("should write paths relative to the descriptor", async () => {
    const folder = getTempFolderPath()
    await mkdir(folder, { recursive: true })

    await saveDatasetToFrictionless(
      { resources: [{ name: "items", data: join(folder, "items.csv") }] },
      { target: join(folder, "datapackage.json") },
    )

    const descriptor = JSON.parse(
      await readFile(join(folder, "datapackage.json"), "utf-8"),
    )
    expect(descriptor.resources).toEqual([{ name: "items", path: "items.csv" }])
  })

  it("should round-trip a package", async () => {
    const folder = getTempFolderPath()
    await mkdir(folder, { recursive: true })
    const original = {
      $schema: "https://datapackage.org/profiles/2.0/datapackage.json",
      title: "Fruit",
      licenses: [{ name: "CC0-1.0", title: "CC0-1.0" }],
      contributors: [{ title: "Ann", roles: ["author"] }],
      resources: [
        {
          name: "fruit",
          path: "fruit.csv",
          type: "table",
          format: "csv",
          dialect: { delimiter: ";" },
          schema: {
            fields: [
              { name: "id", type: "integer", constraints: { required: true } },
              { name: "price", type: "number", decimalChar: "," },
            ],
            primaryKey: ["id"],
          },
        },
      ],
    }
    const path = join(folder, "datapackage.json")
    await writeFile(path, JSON.stringify(original))

    await saveDatasetToFrictionless(await loadDatasetFromFrictionless(path), {
      target: path,
      overwrite: true,
    })

    expect(JSON.parse(await readFile(path, "utf-8"))).toEqual(original)
  })
})
