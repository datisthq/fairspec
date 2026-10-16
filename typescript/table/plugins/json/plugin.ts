import type { Resource } from "@fairspec/metadata"
import { getSupportedFileDialect } from "@fairspec/metadata"
import type { LoadTableOptions, SaveTableOptions, Table } from "../../models/table.ts"
import type { TablePlugin } from "../../plugin.ts"
import { inferJsonFileDialect } from "./actions/fileDialect/infer.ts"
import { loadJsonTable } from "./actions/table/load.ts"
import { saveJsonTable } from "./actions/table/save.ts"

export class JsonPlugin implements TablePlugin {
  async loadTable(resource: Resource, options?: LoadTableOptions) {
    const fileDialect = await getSupportedFileDialect(resource, ["json", "jsonl"])
    if (!fileDialect) return undefined

    return await loadJsonTable({ ...resource, fileDialect }, options)
  }

  async saveTable(table: Table, options: SaveTableOptions) {
    const resource = { data: options.path, ...options }

    const fileDialect = await getSupportedFileDialect(resource, ["json", "jsonl"])
    if (!fileDialect) return undefined

    return await saveJsonTable(table, { ...options, fileDialect })
  }

  async inferFileDialect(resource: Resource) {
    return await inferJsonFileDialect(resource)
  }
}
