import type { Resource } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import type { LoadTableOptions, SaveTableOptions, Table } from "../../models/table.ts"
import type { TablePlugin } from "../../plugin.ts"
import { loadSqliteTable } from "./actions/table/load.ts"
import { saveSqliteTable } from "./actions/table/save.ts"

export class SqlitePlugin implements TablePlugin {
  async loadTable(resource: Resource, options?: LoadTableOptions) {
    const fileDialect = await getSupportedFileDialect(resource, ["sqlite"])
    if (!fileDialect) return undefined

    return await loadSqliteTable(resource, options)
  }

  async saveTable(table: Table, options: SaveTableOptions) {
    const resource = { data: options.path, ...options }

    const fileDialect = await getSupportedFileDialect(resource, ["sqlite"])
    if (!fileDialect) return undefined

    return await saveSqliteTable(table, options)
  }

  async inferFileDialect(resource: Resource) {
    const fileDialect = await getSupportedFileDialect(resource, ["sqlite"])
    if (!fileDialect) return undefined

    return { format: fileDialect.format }
  }
}
