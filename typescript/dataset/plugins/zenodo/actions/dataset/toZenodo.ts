import type { Dataset } from "@fairspec/metadata"
import type { ZenodoCreator } from "../../models/creator.ts"
import type { ZenodoMetadata } from "../../models/metadata.ts"
import type { ZenodoRecord } from "../../models/record.ts"

export function convertDatasetToZenodo(dataset: Dataset): Partial<ZenodoRecord> {
  const metadata: Partial<ZenodoMetadata> = {
    upload_type: "dataset",
  }

  if (dataset.titles?.[0]) {
    metadata.title = dataset.titles[0].title
  }

  if (dataset.descriptions?.[0]) {
    metadata.description = dataset.descriptions[0].description
  } else if (dataset.titles?.[0]) {
    metadata.description = dataset.titles[0].title
  } else {
    metadata.description = "Dataset created with fairspec"
  }

  if (dataset.version) {
    metadata.version = dataset.version
  }

  if (dataset.rightsList && dataset.rightsList.length > 0) {
    const rights = dataset.rightsList[0]
    if (rights?.rights) {
      metadata.license = rights.rights
    }
  }

  if (dataset.creators && dataset.creators.length > 0) {
    metadata.creators = dataset.creators.map(creator => {
      const zenodoCreator: ZenodoCreator = {
        name: creator.name,
      }

      if (creator.affiliation?.[0]) {
        zenodoCreator.affiliation = creator.affiliation[0].name
      }

      return zenodoCreator
    })
  } else {
    metadata.creators = [
      {
        name: "Unknown Author",
        affiliation: "Unknown Affiliation",
      },
    ]
  }

  if (dataset.subjects && dataset.subjects.length > 0) {
    metadata.keywords = dataset.subjects.map(subject => subject.subject)
  }

  if (dataset.dates && dataset.dates.length > 0) {
    const issuedDate = dataset.dates.find(date => date.dateType === "Issued")
    if (issuedDate) {
      metadata.publication_date = issuedDate.date
    }
  }

  if (dataset.doi) {
    metadata.doi = dataset.doi
  }

  return {
    metadata: metadata as ZenodoMetadata,
  }
}
