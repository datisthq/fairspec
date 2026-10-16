import type { Resource } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import type { LoadTableOptions, SaveTableOptions, Table } from "../../models/table.ts"
import type { TablePlugin } from "../../plugin.ts"
import { loadArrowTable } from "./actions/table/load.ts"
import { saveArrowTable } from "./actions/table/save.ts"

export class ArrowPlugin implements TablePlugin {
  async loadTable(resource: Resource, options?: LoadTableOptions) {
    const fileDialect = await getSupportedFileDialect(resource, ["arrow"])
    if (!fileDialect) return undefined

    return await loadArrowTable({ ...resource, fileDialect }, options)
  }

  async saveTable(table: Table, options: SaveTableOptions) {
    const resource = { data: options.path, ...options }

    const fileDialect = await getSupportedFileDialect(resource, ["arrow"])
    if (!fileDialect) return undefined

    return await saveArrowTable(table, { ...options, fileDialect })
  }

  async inferFileDialect(resource: Resource) {
    const fileDialect = await getSupportedFileDialect(resource, ["arrow"])
    if (!fileDialect) return undefined

    return { format: fileDialect.format }
  }
}
