import type { FairspecError, Resource } from "@fairspec/metadata"
import { createReport, inspectJson, resolveDataSchema } from "@fairspec/metadata"
import { loadData } from "./load.ts"

export async function validateData(resource: Resource) {
  const errors: FairspecError[] = []

  const dataSchema = await resolveDataSchema(resource.dataSchema)

  if (dataSchema) {
    const data = await loadData(resource)

    if (data) {
      const dataErrors = await inspectJson(data, { jsonSchema: dataSchema })
      errors.push(
        ...dataErrors.map(error => ({
          type: "data" as const,
          ...error,
        })),
      )
    }

    if (!data) {
      errors.push({
        type: "resource/type",
        expectedResourceType: "data",
      })
    }
  }

  return createReport(errors)
}
