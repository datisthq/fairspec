import { prepareFrictionlessDescriptor } from "../../actions/frictionless/convert.ts"
import { mapFrictionlessErrors } from "../../actions/frictionless/mapping.ts"
import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { inspectModel } from "../../actions/descriptor/inspect.ts"
import { validateDescriptor } from "../../actions/descriptor/validate.ts"
import { META_SCHEMA_URL } from "../../actions/jsonSchema/ajv.ts"
import { loadProfile } from "../../actions/profile/load.ts"
import { inspectTableSchema, inspectTableSchemaNulls } from "./inspect.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import { TableSchema } from "../../models/tableSchema.ts"

export async function validateTableSchema(
  source: TableSchema | Descriptor | string,
  options?: {
    rootJsonPointer?: string
  },
) {
  const loaded =
    typeof source === "string" ? await loadDescriptor(source) : (source as Descriptor)

  const prepared = prepareFrictionlessDescriptor(loaded, {
    kind: "tableSchema",
    rootJsonPointer: options?.rootJsonPointer,
  })
  if (!prepared.descriptor) {
    return { valid: false, errors: prepared.errors, tableSchema: undefined }
  }
  const descriptor = prepared.descriptor

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
    report.errors.push(...parsed.errors)
    if (parsed.data) {
      report.errors.push(
        ...inspectTableSchema(parsed.data, { rootJsonPointer: options?.rootJsonPointer }),
      )
    }
    report.errors.push(
      ...inspectTableSchemaNulls(descriptor, {
        rootJsonPointer: options?.rootJsonPointer,
      }),
    )
    report.valid = report.errors.length === 0
    tableSchema = report.valid ? parsed.data : undefined
  }

  if (prepared.mapping) {
    report.errors = mapFrictionlessErrors(report.errors, prepared.mapping)
  }

  return { ...report, tableSchema }
}
