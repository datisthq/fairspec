import type { Dataset } from "@fairspec/metadata"
import type { SetRequired } from "type-fest"
import { convertResourceToCkan } from "../../actions/resource/toCkan.ts"
import type { CkanDataset } from "../../models/dataset.ts"
import type { CkanResource } from "../../models/resource.ts"
import type { CkanTag } from "../../models/tag.ts"

export function convertDatasetToCkan(dataset: Dataset) {
  const ckanDataset: SetRequired<Partial<CkanDataset>, "resources" | "tags"> = {
    resources: [],
    tags: [],
  }

  if (dataset.titles?.[0]?.title) {
    ckanDataset.title = dataset.titles[0].title
  }

  if (dataset.descriptions?.[0]?.description) {
    ckanDataset.notes = dataset.descriptions[0].description
  }

  if (dataset.version) {
    ckanDataset.version = dataset.version
  }

  if (dataset.rightsList && dataset.rightsList.length > 0) {
    const rights = dataset.rightsList[0]

    if (rights?.rightsIdentifier) {
      ckanDataset.license_id = rights.rightsIdentifier
    }
    if (rights?.rights) {
      ckanDataset.license_title = rights.rights
    }
    if (rights?.rightsUri) {
      ckanDataset.license_url = rights.rightsUri
    }
  }

  if (dataset.creators && dataset.creators.length > 0) {
    const creator = dataset.creators[0]
    if (creator?.name) {
      ckanDataset.author = creator.name
    }
  }

  if (dataset.contributors) {
    const maintainer = dataset.contributors.find(
      c => c.contributorType === "ContactPerson",
    )
    if (maintainer?.name) {
      ckanDataset.maintainer = maintainer.name
    }
  }

  if (dataset.resources && dataset.resources.length > 0) {
    ckanDataset.resources = dataset.resources
      .map(resource => convertResourceToCkan(resource))
      .filter((resource): resource is CkanResource => resource !== undefined)
  }

  if (dataset.subjects && dataset.subjects.length > 0) {
    ckanDataset.tags = dataset.subjects.map(subject => ({
      name: subject.subject,
      display_name: subject.subject,
    })) as CkanTag[]
  }

  const createdDate = dataset.dates?.find(d => d.dateType === "Created")
  if (createdDate?.date) {
    ckanDataset.metadata_created = createdDate.date
  }

  const updatedDate = dataset.dates?.find(d => d.dateType === "Updated")
  if (updatedDate?.date) {
    ckanDataset.metadata_modified = updatedDate.date
  }

  return ckanDataset
}
