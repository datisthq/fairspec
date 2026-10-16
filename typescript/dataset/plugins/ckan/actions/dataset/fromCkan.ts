import type { Dataset } from "@fairspec/metadata"
import { convertResourceFromCkan } from "../../actions/resource/fromCkan.ts"
import type { CkanDataset } from "../../models/dataset.ts"

export function convertDatasetFromCkan(ckanDataset: CkanDataset): Dataset {
  const dataset: Dataset = {
    resources: [],
  }

  if (ckanDataset.title) {
    dataset.titles = [{ title: ckanDataset.title }]
  }

  if (ckanDataset.notes) {
    dataset.descriptions = [
      {
        description: ckanDataset.notes,
        descriptionType: "Abstract",
      },
    ]
  }

  if (ckanDataset.version) {
    dataset.version = ckanDataset.version
  }

  if (ckanDataset.resources && ckanDataset.resources.length > 0) {
    dataset.resources = ckanDataset.resources.map(resource =>
      convertResourceFromCkan(resource),
    )
  }

  if (ckanDataset.license_id || ckanDataset.license_title) {
    dataset.rightsList = [
      {
        rights: ckanDataset.license_title || ckanDataset.license_id || "",
        rightsUri: ckanDataset.license_url,
        rightsIdentifier: ckanDataset.license_id,
      },
    ]
  }

  if (ckanDataset.author) {
    dataset.creators = [
      {
        name: ckanDataset.author,
        nameType: "Personal",
      },
    ]
  }

  const contributors = []

  if (ckanDataset.maintainer) {
    contributors.push({
      name: ckanDataset.maintainer,
      nameType: "Personal" as const,
      contributorType: "ContactPerson" as const,
    })
  }

  if (contributors.length > 0) {
    dataset.contributors = contributors
  }

  if (ckanDataset.tags && ckanDataset.tags.length > 0) {
    dataset.subjects = ckanDataset.tags.map(tag => ({ subject: tag.name }))
  }

  if (ckanDataset.metadata_created) {
    dataset.dates = [
      {
        date: ckanDataset.metadata_created,
        dateType: "Created",
      },
    ]
  }

  if (ckanDataset.metadata_modified) {
    dataset.dates = [
      ...(dataset.dates || []),
      {
        date: ckanDataset.metadata_modified,
        dateType: "Updated",
      },
    ]
  }

  return dataset
}
