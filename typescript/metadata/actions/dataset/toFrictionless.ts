import { convertResourceToFrictionless } from "../../actions/resource/toFrictionless.ts"
import { convertRightsToFrictionless } from "../../actions/rights/toFrictionless.ts"
import type { Contributor } from "../../models/datacite/contributor.ts"
import type { Creator } from "../../models/datacite/creator.ts"
import type { Dataset } from "../../models/dataset.ts"
import type { Descriptor } from "../../models/descriptor.ts"

const ROLES: Record<string, string> = {
  ContactPerson: "maintainer",
  DataCurator: "wrangler",
  Distributor: "publisher",
}

/**
 * Convert a denormalized Fairspec Dataset to a Frictionless Data Package descriptor
 * Its resources' `tableSchema` and `fileDialect` must already be objects, not paths
 */
export function convertDatasetToFrictionless(dataset: Dataset) {
  const frictionlessPackage: Descriptor = {
    $schema: "https://datapackage.org/profiles/2.0/datapackage.json",
  }

  if (dataset.doi) frictionlessPackage.id = `https://doi.org/${dataset.doi}`

  const title = dataset.titles?.[0]?.title
  if (title) frictionlessPackage.title = title

  const description = dataset.descriptions?.[0]?.description
  if (description) frictionlessPackage.description = description

  const homepage = dataset.relatedIdentifiers?.find(
    identifier => identifier.relationType === "IsDocumentedBy",
  )
  if (homepage) frictionlessPackage.homepage = homepage.relatedIdentifier

  if (dataset.version) frictionlessPackage.version = dataset.version

  const created = dataset.dates?.find(date => date.dateType === "Created")
  if (created) frictionlessPackage.created = created.date

  if (dataset.subjects && dataset.subjects.length > 0) {
    frictionlessPackage.keywords = dataset.subjects.map(subject => subject.subject)
  }

  if (dataset.rightsList && dataset.rightsList.length > 0) {
    frictionlessPackage.licenses = convertRightsToFrictionless(dataset.rightsList)
  }

  const contributors = [
    ...(dataset.creators ?? []).map(creator => convertPerson(creator, "author")),
    ...(dataset.contributors ?? []).map(contributor =>
      convertPerson(contributor, ROLES[contributor.contributorType] ?? "contributor"),
    ),
  ]
  if (contributors.length > 0) frictionlessPackage.contributors = contributors

  const sources = (dataset.relatedIdentifiers ?? [])
    .filter(identifier => identifier.relationType === "IsDerivedFrom")
    .map(identifier => ({
      title: identifier.relatedIdentifier,
      path: identifier.relatedIdentifier,
    }))
  if (sources.length > 0) frictionlessPackage.sources = sources

  frictionlessPackage.resources = (dataset.resources ?? []).map((resource, index) =>
    convertResourceToFrictionless(resource, { resourceNumber: index + 1 }),
  )

  return frictionlessPackage
}

function convertPerson(person: Creator | Contributor, role: string) {
  const contributor: Descriptor = { title: person.name }
  if (person.givenName) contributor.givenName = person.givenName
  if (person.familyName) contributor.familyName = person.familyName
  const organization = person.affiliation?.[0]?.name
  if (organization) contributor.organization = organization
  contributor.roles = [role]
  return contributor
}
