import type { FairspecError, Resource } from "@fairspec/metadata"
import { createReport, FairspecException, resolveTableSchema } from "@fairspec/metadata"
import {
  getIsLongRowError,
  getIsPartHeaderException,
  inspectTable,
  inspectTableSchemaPatterns,
} from "@fairspec/table"
import { inferTableSchema } from "../../actions/tableSchema/infer.ts"
import type { ValidateTableOptions } from "../../models/table.ts"
import { loadTable } from "./load.ts"

export async function validateTable(resource: Resource, options?: ValidateTableOptions) {
  try {
    return await validateLoadableTable(resource, options)
  } catch (error) {
    if (getIsPartHeaderException(error) && error instanceof FairspecException) {
      return createReport(error.report?.errors)
    }
    if (!getIsLongRowError(error)) throw error
    const errors: FairspecError[] = [
      { type: "resource/type", expectedResourceType: "table" },
    ]
    return createReport(errors)
  }
}

async function validateLoadableTable(resource: Resource, options?: ValidateTableOptions) {
  const { maxErrors, checks } = options ?? {}
  const errors: FairspecError[] = []

  let tableSchema = await resolveTableSchema(resource.tableSchema)
  if (!tableSchema && !options?.noInfer) {
    tableSchema = await inferTableSchema(resource, options)
  }

  const patternErrors = tableSchema ? inspectTableSchemaPatterns(tableSchema) : []
  if (patternErrors.length) {
    return createReport(patternErrors, { maxErrors })
  }

  if (tableSchema || checks?.length) {
    const table = await loadTable(resource, { denormalized: true })

    if (table) {
      const tableErrors = await inspectTable(table, { tableSchema, checks, maxErrors })
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
