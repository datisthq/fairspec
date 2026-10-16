import { validateDataSchema } from "../../actions/dataSchema/validate.ts"
import { loadDescriptor } from "../../actions/descriptor/load.ts"
import { validateDescriptor } from "../../actions/descriptor/validate.ts"
import { validateFileDialect } from "../../actions/fileDialect/validate.ts"
import { loadProfile } from "../../actions/profile/load.ts"
import { validateTableSchema } from "../../actions/tableSchema/validate.ts"
import type { Dataset } from "../../models/dataset.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import { normalizeDataset } from "./normalize.ts"

export async function validateDatasetDescriptor(
  source: Dataset | Descriptor | string,
  options?: {
    basepath?: string
  },
) {
  const descriptor =
    typeof source === "string" ? await loadDescriptor(source) : (source as Descriptor)

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
    // Valid -> we can cast it
    dataset = descriptor as Dataset
  }

  if (dataset) {
    dataset = normalizeDataset(dataset, {
      basepath: options?.basepath,
    })
  }

  if (dataset) {
    for (const [index, resource] of (dataset?.resources ?? []).entries()) {
      const rootJsonPointer = `/resources/${index}`

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
      }
    }

    if (report.errors.length) {
      dataset = undefined
      report.valid = false
    }
  }

  return { ...report, dataset }
}
