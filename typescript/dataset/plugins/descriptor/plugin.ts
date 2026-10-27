import type { Dataset, Report } from "@fairspec/metadata"
import {
  convertDescriptorFromFrictionless,
  createReport,
  getFileExtension,
  getIsFrictionlessDescriptor,
  getIsRemotePath,
  loadDatasetDescriptor,
  loadDescriptor,
  mapFrictionlessErrors,
  resolveBasepath,
  saveDatasetDescriptor,
  validateDatasetDescriptor,
} from "@fairspec/metadata"
import type { DatasetPlugin } from "../../plugin.ts"

export class DescriptorPlugin implements DatasetPlugin {
  async loadDataset(source: string) {
    const isJson = getIsJson(source)
    if (!isJson) return undefined

    const dataset = await loadDatasetDescriptor(source)
    return dataset
  }

  async validateDataset(
    source: string,
    options: { validateDataset: (dataset: Dataset) => Promise<Report> },
  ) {
    if (!getIsJson(source)) return undefined

    const basepath = await resolveBasepath(source)
    const descriptor = await loadDescriptor(source)

    const report = await validateDatasetDescriptor(descriptor, { basepath })
    if (!report.dataset) return createReport(report.errors)

    const datasetReport = await options.validateDataset(report.dataset)
    if (!getIsFrictionlessDescriptor(descriptor, { kind: "dataset" }))
      return datasetReport

    const { mapping } = convertDescriptorFromFrictionless(descriptor, { kind: "dataset" })
    return createReport(mapFrictionlessErrors(datasetReport.errors, mapping))
  }

  async saveDataset(dataset: Dataset, options: { target: string; withRemote?: boolean }) {
    const isLocalJson = getIsLocalJson(options.target)
    if (!isLocalJson) return undefined

    await saveDatasetDescriptor(dataset, { path: options.target })

    return { path: options.target }
  }
}

function getIsLocalJson(path: string) {
  const isJson = getIsJson(path)
  const isRemote = getIsRemotePath(path)
  return isJson && !isRemote
}

function getIsJson(path: string) {
  const extension = getFileExtension(path)
  return extension === "json"
}
