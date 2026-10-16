import type { Dataset } from "@fairspec/metadata"
import { convertResourceFromZenodo } from "../../actions/resource/fromZenodo.ts"
import type { ZenodoRecord } from "../../models/record.ts"

export function convertDatasetFromZenodo(zenodoRecord: ZenodoRecord): Dataset {
  const dataset: Dataset = {
    resources: [],
  }

  const metadata = zenodoRecord.metadata

  if (metadata.title) {
    dataset.titles = [{ title: metadata.title }]
  }

  if (metadata.description) {
    dataset.descriptions = [
      { description: metadata.description, descriptionType: "Abstract" },
    ]
  }

  if (metadata.creators && metadata.creators.length > 0) {
    dataset.creators = metadata.creators.map(creator => ({
      name: creator.name,
      nameType: "Personal",
      affiliation: creator.affiliation ? [{ name: creator.affiliation }] : undefined,
    }))
  }

  if (metadata.keywords && metadata.keywords.length > 0) {
    dataset.subjects = metadata.keywords.map(keyword => ({ subject: keyword }))
  }

  if (metadata.publication_date) {
    dataset.dates = [{ date: metadata.publication_date, dateType: "Issued" }]
  }

  if (metadata.license) {
    dataset.rightsList = [{ rights: metadata.license }]
  }

  if (metadata.doi) {
    dataset.doi = metadata.doi
  }

  if (metadata.version) {
    dataset.version = metadata.version
  }

  if (zenodoRecord.files && zenodoRecord.files.length > 0) {
    dataset.resources = zenodoRecord.files.map(zenodoResource =>
      convertResourceFromZenodo(zenodoResource),
    )
  }

  return dataset
}
