import type { Resource } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import type { LoadTableOptions, SaveTableOptions, Table } from "../../models/table.ts"
import type { TablePlugin } from "../../plugin.ts"
import { inferXlsxFileDialect } from "./actions/fileDialect/infer.ts"
import { loadXlsxTable } from "./actions/table/load.ts"
import { saveXlsxTable } from "./actions/table/save.ts"

export class XlsxPlugin implements TablePlugin {
  async loadTable(resource: Resource, options?: LoadTableOptions) {
    const fileDialect = await getSupportedFileDialect(resource, ["xlsx", "ods"])
    if (!fileDialect) return undefined

    return await loadXlsxTable({ ...resource, fileDialect }, options)
  }

  async saveTable(table: Table, options: SaveTableOptions) {
    const resource = { data: options.path, ...options }

    const fileDialect = await getSupportedFileDialect(resource, ["xlsx", "ods"])
    if (!fileDialect) return undefined

    return await saveXlsxTable(table, { ...options, fileDialect })
  }

  async inferFileDialect(resource: Resource) {
    const fileDialect = await getSupportedFileDialect(resource, ["xlsx", "ods"])
    if (!fileDialect) return undefined

    return await inferXlsxFileDialect(resource)
  }
}
