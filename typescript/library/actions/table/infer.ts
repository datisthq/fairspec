import type { Resource } from "@fairspec/metadata"
import { resolveTableSchema } from "@fairspec/metadata"
import { inferTableSchemaFromTable } from "@fairspec/table"
import { inferFileDialect } from "../../actions/fileDialect/infer.ts"
import { loadTable } from "./load.ts"

export async function inferTable(resource: Resource) {
  let fileDialect = resource.fileDialect

  if (!fileDialect) {
    fileDialect = await inferFileDialect(resource)
  }

  const table = await loadTable({ ...resource, fileDialect }, { denormalized: true })

  if (!table) {
    return undefined
  }

  let tableSchema = await resolveTableSchema(resource.tableSchema)
  if (!tableSchema) {
    tableSchema = await inferTableSchemaFromTable(table)
  }

  return { fileDialect, table, tableSchema }
}
