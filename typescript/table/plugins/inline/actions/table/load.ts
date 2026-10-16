import type { Resource } from "@fairspec/metadata"
import { resolveTableSchema } from "@fairspec/metadata"
import { getDataRecords } from "@fairspec/metadata"
import type { LoadTableOptions } from "../../../../models/table.ts"
import { inferTableSchemaFromTable } from "../../../../actions/tableSchema/infer.ts"
import { normalizeTable } from "../../../../actions/table/normalize.ts"
import * as pl from "nodejs-polars"

export async function loadInlineTable(resource: Resource, options?: LoadTableOptions) {
  const dataRecords = getDataRecords(resource)
  if (!dataRecords) {
    throw new Error("Resource data is not defined or tabular")
  }

  let table = pl.DataFrame(dataRecords).lazy()

  if (!options?.denormalized) {
    let tableSchema = await resolveTableSchema(resource.tableSchema)
    if (!tableSchema) tableSchema = await inferTableSchemaFromTable(table, options)
    table = await normalizeTable(table, tableSchema)
  }

  return table
}
