import type { DatasetPlugin } from "@fairspec/dataset"
import type { Resource, TableSchema } from "@fairspec/metadata"
import type { InferTableSchemaOptions } from "./models/schema.ts"
import type { LoadTableOptions, SaveTableOptions, Table } from "./models/table.ts"

export interface TablePlugin extends DatasetPlugin {
  loadTable?(resource: Resource, options?: LoadTableOptions): Promise<Table | undefined>

  saveTable?(table: Table, options: SaveTableOptions): Promise<string | undefined>

  inferTableSchema?(
    resource: Resource,
    options?: InferTableSchemaOptions,
  ): Promise<TableSchema | undefined>
}
