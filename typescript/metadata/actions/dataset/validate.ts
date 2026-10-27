import { validateDataSchema } from "../../actions/dataSchema/validate.ts"
import { prepareFrictionlessDescriptor } from "../../actions/frictionless/convert.ts"
import { mapFrictionlessErrors } from "../../actions/frictionless/mapping.ts"
import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { inspectModel } from "../../actions/descriptor/inspect.ts"
import { validateDescriptor } from "../../actions/descriptor/validate.ts"
import { validateFileDialect } from "../../actions/fileDialect/validate.ts"
import { loadProfile } from "../../actions/profile/load.ts"
import { getDataPaths } from "../../actions/resource/data.ts"
import {
  inspectTableSchema,
  inspectTableSchemaNulls,
} from "../../actions/tableSchema/inspect.ts"
import { validateTableSchema } from "../../actions/tableSchema/validate.ts"
import { Dataset } from "../../models/dataset.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { MetadataError } from "../../models/error/metadata.ts"
import type { Resource } from "../../models/resource.ts"
import type { TableSchema } from "../../models/tableSchema.ts"
import { normalizeDataset } from "./normalize.ts"

export async function validateDatasetDescriptor(
  source: Dataset | Descriptor | string,
  options?: {
    basepath?: string
  },
) {
  const loaded =
    typeof source === "string" ? await loadDescriptor(source) : (source as Descriptor)

  const prepared = prepareFrictionlessDescriptor(loaded, { kind: "dataset" })
  if (!prepared.descriptor) {
    return { valid: false, errors: prepared.errors, dataset: undefined }
  }
  const descriptor = prepared.descriptor

  const $schema =
    typeof descriptor.$schema === "string"
      ? descriptor.$schema
      : `https://fairspec.org/profiles/latest/dataset.json`

  const profile = await loadProfile($schema, {
    profileType: "dataset",
  })

  const report = await validateDescriptor(descriptor, {
    profile,
  })

  let dataset: Dataset | undefined
  if (report.valid) {
    const parsed = inspectModel(descriptor, {
      model: Dataset,
    })
    dataset = parsed.data
    report.errors.push(...parsed.errors)
    report.valid = report.errors.length === 0
  }

  if (dataset) {
    dataset = normalizeDataset(dataset, {
      basepath: options?.basepath,
    })
  }

  if (dataset) {
    const names = new Set<string>()
    const resources = dataset.resources ?? []
    for (const [index, resource] of resources.entries()) {
      const rootJsonPointer = `/resources/${index}`

      if (resource.name !== undefined) {
        if (names.has(resource.name)) {
          report.errors.push({
            type: "metadata",
            message: `resource name "${resource.name}" is not unique`,
            jsonPointer: `${rootJsonPointer}/name`,
          })
        }
        names.add(resource.name)
      }

      if (resource.dataSchema && getDataPaths(resource).length > 1) {
        report.errors.push({
          type: "metadata",
          message: "a data schema needs single-part data",
          jsonPointer: `${rootJsonPointer}/dataSchema`,
        })
      }

      if (typeof resource.fileDialect === "string") {
        const fileDialectReport = await validateFileDialect(resource.fileDialect, {
          rootJsonPointer,
        })

        report.errors.push(...fileDialectReport.errors)
      }

      if (typeof resource.dataSchema === "string") {
        const dataSchemaReport = await validateDataSchema(resource.dataSchema, {
          rootJsonPointer,
        })

        report.errors.push(...dataSchemaReport.errors)
      }

      if (typeof resource.tableSchema === "string") {
        const tableSchemaReport = await validateTableSchema(resource.tableSchema, {
          rootJsonPointer,
        })

        report.errors.push(...tableSchemaReport.errors)
      } else if (resource.tableSchema) {
        report.errors.push(
          ...inspectTableSchema(resource.tableSchema, {
            rootJsonPointer: `${rootJsonPointer}/tableSchema`,
          }),
          ...inspectTableSchemaNulls(getTableSchemaDescriptor(descriptor, index), {
            rootJsonPointer: `${rootJsonPointer}/tableSchema`,
          }),
          ...inspectForeignKeyTargets(resource.tableSchema, resources, {
            rootJsonPointer: `${rootJsonPointer}/tableSchema`,
          }),
        )
      }
    }

    if (report.errors.length) {
      dataset = undefined
      report.valid = false
    }
  }

  if (prepared.mapping) {
    report.errors = mapFrictionlessErrors(report.errors, prepared.mapping)
  }

  return { ...report, dataset }
}

function inspectForeignKeyTargets(
  tableSchema: TableSchema,
  resources: Resource[],
  options: { rootJsonPointer: string },
) {
  const errors: MetadataError[] = []
  for (const [index, foreignKey] of (tableSchema.foreignKeys ?? []).entries()) {
    const name = foreignKey.reference.resource
    const target = resources.find(item => item.name === name)
    if (name !== undefined && target?.dataSchema) {
      errors.push({
        type: "metadata",
        message: `foreign key references resource "${name}", not a table`,
        jsonPointer: `${options.rootJsonPointer}/foreignKeys/${index}/reference/resource`,
      })
    }
  }
  return errors
}

function getTableSchemaDescriptor(descriptor: Descriptor, index: number) {
  const resources: unknown[] = Array.isArray(descriptor.resources)
    ? descriptor.resources
    : []
  const resource = resources[index]
  const tableSchema = isDescriptor(resource) ? resource.tableSchema : undefined
  return isDescriptor(tableSchema) ? tableSchema : {}
}

function isDescriptor(value: unknown): value is Descriptor {
  return typeof value === "object" && value !== null && !Array.isArray(value)
}
