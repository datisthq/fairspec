import type { FairspecError, Resource } from "@fairspec/metadata"
import { createReport, resolveTableSchema } from "@fairspec/metadata"
import { inspectTable } from "@fairspec/table"
import { inferTableSchema } from "../../actions/tableSchema/infer.ts"
import type { ValidateTableOptions } from "../../models/table.ts"
import { loadTable } from "./load.ts"

export async function validateTable(resource: Resource, options?: ValidateTableOptions) {
  const { maxErrors } = options ?? {}
  const errors: FairspecError[] = []

  let tableSchema = await resolveTableSchema(resource.tableSchema)
  if (!tableSchema && !options?.noInfer) {
    tableSchema = await inferTableSchema(resource, options)
  }

  if (tableSchema) {
    const table = await loadTable(resource, { denormalized: true })

    if (table) {
      const tableErrors = await inspectTable(table, { tableSchema, maxErrors })
      errors.push(...tableErrors)
    }

    if (!table) {
      errors.push({
        type: "resource/type",
        expectedResourceType: "table",
      })
    }
  }

  return createReport(errors, { maxErrors })
}
