import type { Resource } from "@fairspec/metadata"
import { resolveTableSchema } from "@fairspec/metadata"
import { prefetchFiles } from "@fairspec/dataset"
import type { LoadTableOptions } from "../../../../models/table.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"
import * as pl from "nodejs-polars"

export async function loadParquetTable(resource: Resource, options?: LoadTableOptions) {
  const [firstPath, ...restPaths] = await prefetchFiles(resource)
  if (!firstPath) {
    throw new Error("Resource data is not defined")
  }

  let table = pl.scanParquet(firstPath)
  if (restPaths.length) {
    table = pl.concat([table, ...restPaths.map(path => pl.scanParquet(path))])
  }

  if (!options?.denormalized) {
    let tableSchema = await resolveTableSchema(resource.tableSchema)
    if (!tableSchema) tableSchema = await inferTableSchemaFromTable(table, options)
    table = await normalizeTable(table, tableSchema)
  }

  return table
}
