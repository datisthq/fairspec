import os from "node:os"
import type { Dataset, FairspecError } from "@fairspec/metadata"
import {
  convertDescriptorFromFrictionless,
  createReport,
  FairspecException,
  getIsFrictionlessDescriptor,
  inferResourceName,
  mapFrictionlessErrors,
  validateDatasetDescriptor,
} from "@fairspec/metadata"
import pAll from "p-all"
import { validateResource } from "../../actions/resource/validate.ts"
import { validateDatasetForeignKeys } from "./foreignKey.ts"
import { system } from "../../system.ts"
import { loadDataset } from "./load.ts"

export async function validateDataset(source: string | Dataset) {
  let dataset: Dataset

  if (typeof source !== "string") {
    if (getIsFrictionlessDescriptor(source, { kind: "dataset" })) {
      return await validateFrictionlessDataset(source)
    }
    dataset = source
  } else {
    for (const plugin of system.plugins) {
      const report = await plugin.validateDataset?.(source, {
        validateDataset: validateLoadedDataset,
      })
      if (report) return report
    }

    try {
      dataset = await loadDataset(source)
    } catch (exception) {
      if (exception instanceof FairspecException) {
        if (exception.report) {
          return exception.report
        }
      }

      throw exception
    }
  }

  return await validateLoadedDataset(dataset)
}

async function validateFrictionlessDataset(source: Dataset) {
  const report = await validateDatasetDescriptor(source)
  if (!report.dataset) return createReport(report.errors)

  const datasetReport = await validateLoadedDataset(report.dataset)
  const { mapping } = convertDescriptorFromFrictionless(source, { kind: "dataset" })
  return createReport(mapFrictionlessErrors(datasetReport.errors, mapping))
}

async function validateLoadedDataset(dataset: Dataset) {
  const resourcesReport = await validateDatasetResources(dataset)
  const foreignKeyReport = await validateDatasetForeignKeys(dataset)

  const errors = [...resourcesReport.errors, ...foreignKeyReport.errors]
  return createReport(errors)
}

export async function validateDatasetResources(dataset: Dataset) {
  const concurrency = os.cpus().length

  const errors: FairspecError[] = (
    await pAll(
      (dataset.resources ?? []).map((resource, index) => async () => {
        const resourceName =
          resource.name ??
          inferResourceName(resource, {
            resourceNumber: index + 1,
          })

        try {
          const report = await validateResource(resource)
          return report.errors.map(error => ({
            ...error,
            resourceName,
          }))
        } catch (error) {
          const message = error instanceof Error ? error.message : String(error)
          throw new Error(`[${resource.name}] ${message}`)
        }
      }),
      { concurrency },
    )
  ).flat()

  return createReport(errors)
}
