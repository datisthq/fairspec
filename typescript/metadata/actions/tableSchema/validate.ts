import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { validateDescriptor } from "../../actions/descriptor/validate.ts"
import { loadProfile } from "../../actions/profile/load.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { TableSchema } from "../../models/tableSchema.ts"

export async function validateTableSchema(
  source: TableSchema | Descriptor | string,
  options?: {
    rootJsonPointer?: string
  },
) {
  const descriptor =
    typeof source === "string" ? await loadDescriptor(source) : (source as Descriptor)

  const $schema =
    typeof descriptor.$schema === "string"
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
    // Valid -> we can cast it
    tableSchema = descriptor as TableSchema
  }

  return { ...report, tableSchema }
}
