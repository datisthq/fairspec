import type { Resource } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import type { LoadTableOptions, SaveTableOptions, Table } from "../../models/table.ts"
import type { TablePlugin } from "../../plugin.ts"
import { loadParquetTable } from "./actions/table/load.ts"
import { saveParquetTable } from "./actions/table/save.ts"

export class ParquetPlugin implements TablePlugin {
  async loadTable(resource: Resource, options?: LoadTableOptions) {
    const fileDialect = await getSupportedFileDialect(resource, ["parquet"])
    if (!fileDialect) return undefined

    return await loadParquetTable(resource, options)
  }

  async saveTable(table: Table, options: SaveTableOptions) {
    const resource = { data: options.path, ...options }

    const fileDialect = await getSupportedFileDialect(resource, ["parquet"])
    if (!fileDialect) return undefined

    return await saveParquetTable(table, options)
  }

  async inferFileDialect(resource: Resource) {
    const fileDialect = await getSupportedFileDialect(resource, ["parquet"])
    if (!fileDialect) return undefined

    return { format: fileDialect.format }
  }
}
