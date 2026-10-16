import { validateFile } from "@fairspec/dataset"
import type { Resource } from "@fairspec/metadata"
import { createReport } from "@fairspec/metadata"
import { validateData } from "../../actions/data/validate.ts"
import { validateTable } from "../../actions/table/validate.ts"
import type { ValidateTableOptions } from "../../models/table.ts"

export async function validateResource(
  resource: Resource,
  options?: ValidateTableOptions,
) {
  const fileReport = await validateFile(resource)
  if (!fileReport.valid) {
    return fileReport
  }

  const dataReport = await validateData(resource)
  if (!dataReport.valid) {
    return dataReport
  }

  const tableReport = await validateTable(resource, options)
  if (!tableReport.valid) {
    return tableReport
  }

  return createReport()
}
