import type { FairspecError, Resource } from "@fairspec/metadata"
import {
  createReport,
  getDataPaths,
  inspectJson,
  resolveDataSchema,
} from "@fairspec/metadata"
import { loadData } from "./load.ts"

export async function validateData(resource: Resource) {
  const errors: FairspecError[] = []

  const dataSchema = await resolveDataSchema(resource.dataSchema)

  if (dataSchema && getDataPaths(resource).length > 1) {
    errors.push({
      type: "metadata",
      message: "a data schema needs single-part data",
      jsonPointer: "/dataSchema",
    })
  } else if (dataSchema) {
    const data = await loadData(resource)

    if (data !== undefined) {
      const dataErrors = await inspectJson(data, { jsonSchema: dataSchema })
      errors.push(
        ...dataErrors.map(error => ({
          type: "data" as const,
          ...error,
        })),
      )
    }

    if (data === undefined) {
      errors.push({
        type: "resource/type",
        expectedResourceType: "data",
      })
    }
  }

  return createReport(errors)
}
