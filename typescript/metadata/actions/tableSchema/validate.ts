import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { inspectModel } from "../../actions/descriptor/inspect.ts"
import { validateDescriptor } from "../../actions/descriptor/validate.ts"
import { META_SCHEMA_URL } from "../../actions/jsonSchema/ajv.ts"
import { loadProfile } from "../../actions/profile/load.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import { TableSchema } from "../../models/tableSchema.ts"

export async function validateTableSchema(
  source: TableSchema | Descriptor | string,
  options?: {
    rootJsonPointer?: string
  },
) {
  const descriptor =
    typeof source === "string" ? await loadDescriptor(source) : (source as Descriptor)

  const $schema =
    typeof descriptor.$schema === "string" && descriptor.$schema !== META_SCHEMA_URL
      ? descriptor.$schema
      : `https://fairspec.org/profiles/latest/table-schema.json`

  const profile = await loadProfile($schema, {
    profileType: "table-schema",
  })

  const report = await validateDescriptor(descriptor, {
    profile,
    rootJsonPointer: options?.rootJsonPointer,
  })

  let tableSchema: TableSchema | undefined
  if (report.valid) {
    const parsed = inspectModel(descriptor, {
      model: TableSchema,
      rootJsonPointer: options?.rootJsonPointer,
    })
    tableSchema = parsed.data
    report.errors.push(...parsed.errors)
    report.valid = report.errors.length === 0
  }

  return { ...report, tableSchema }
}
