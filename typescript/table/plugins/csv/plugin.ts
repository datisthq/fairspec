import type { Resource } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import type { LoadTableOptions, SaveTableOptions, Table } from "../../models/table.ts"
import type { TablePlugin } from "../../plugin.ts"
import { inferCsvFileDialect } from "./actions/fileDialect/infer.ts"
import { loadCsvTable } from "./actions/table/load.ts"
import { saveCsvTable } from "./actions/table/save.ts"

// TODO: splig csv/tsv and similar plugins?
// TODO: improve nameing loadCsvTable/saveCsvTable (tsv?)

export class CsvPlugin implements TablePlugin {
  async loadTable(resource: Resource, options?: LoadTableOptions) {
    const fileDialect = await getSupportedFileDialect(resource, ["csv", "tsv"])
    if (!fileDialect) return undefined

    return await loadCsvTable({ ...resource, fileDialect }, options)
  }

  async saveTable(table: Table, options: SaveTableOptions) {
    const resource = { data: options.path, ...options }

    const fileDialect = await getSupportedFileDialect(resource, ["csv", "tsv"])
    if (!fileDialect) return undefined

    return await saveCsvTable(table, { ...options, fileDialect })
  }

  async inferFileDialect(resource: Resource) {
    const fileDialect = await getSupportedFileDialect(resource, ["csv", "tsv"])
    if (!fileDialect) return undefined

    return await inferCsvFileDialect(resource)
  }
}
