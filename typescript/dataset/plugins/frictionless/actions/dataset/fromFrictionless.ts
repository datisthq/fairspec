import type { Descriptor } from "@fairspec/metadata"
import type {
  FrictionlessContributor,
  ResolvedFrictionlessPackage,
} from "../../models/package.ts"
import { convertResourceFromFrictionless } from "../resource/fromFrictionless.ts"
import { convertRightsFromFrictionless } from "../rights/fromFrictionless.ts"

const CONTRIBUTOR_TYPES: Record<string, string> = {
  maintainer: "ContactPerson",
  wrangler: "DataCurator",
  publisher: "Distributor",
}

/**
 * Convert a Frictionless Data Package to a Fairspec Dataset descriptor
 * Its resources' `schema` and `dialect` must already be objects, not paths
 */
export function convertDatasetFromFrictionless(
  frictionlessPackage: ResolvedFrictionlessPackage,
) {
  const dataset: Descriptor = {}

  const doi = convertDoi(frictionlessPackage.id)
  if (doi) dataset.doi = doi

  const creators: Descriptor[] = []
  const contributors: Descriptor[] = []
  for (const contributor of frictionlessPackage.contributors ?? []) {
    const person = convertPerson(contributor)
    if (!person) continue

    const roles = contributor.roles ?? (contributor.role ? [contributor.role] : [])
    if (roles.includes("author") || roles.includes("creator")) {
      creators.push(person)
    } else {
      const role = roles.find(role => CONTRIBUTOR_TYPES[role])
      contributors.push({
        ...person,
        contributorType: role ? CONTRIBUTOR_TYPES[role] : "Other",
      })
    }
  }

  if (creators.length > 0) dataset.creators = creators

  if (frictionlessPackage.title) {
    dataset.titles = [{ title: frictionlessPackage.title }]
  }

  if (frictionlessPackage.keywords && frictionlessPackage.keywords.length > 0) {
    dataset.subjects = frictionlessPackage.keywords.map(subject => ({ subject }))
  }

  if (contributors.length > 0) dataset.contributors = contributors

  if (frictionlessPackage.created) {
    dataset.dates = [{ date: frictionlessPackage.created, dateType: "Created" }]
  }

  const relatedIdentifiers: Descriptor[] = []
  if (frictionlessPackage.homepage) {
    relatedIdentifiers.push({
      relatedIdentifier: frictionlessPackage.homepage,
      relatedIdentifierType: "URL",
      relationType: "IsDocumentedBy",
    })
  }
  for (const source of frictionlessPackage.sources ?? []) {
    if (!source.path?.match(/^https?:\/\//)) continue
    relatedIdentifiers.push({
      relatedIdentifier: source.path,
      relatedIdentifierType: "URL",
      relationType: "IsDerivedFrom",
    })
  }
  if (relatedIdentifiers.length > 0) dataset.relatedIdentifiers = relatedIdentifiers

  if (frictionlessPackage.version) dataset.version = frictionlessPackage.version

  if (frictionlessPackage.licenses && frictionlessPackage.licenses.length > 0) {
    dataset.rightsList = convertRightsFromFrictionless(frictionlessPackage.licenses)
  }

  if (frictionlessPackage.description) {
    dataset.descriptions = [
      { description: frictionlessPackage.description, descriptionType: "Abstract" },
    ]
  }

  dataset.resources = frictionlessPackage.resources.map(resource =>
    convertResourceFromFrictionless(resource),
  )

  return dataset
}

function convertPerson(contributor: FrictionlessContributor) {
  const name =
    contributor.title ??
    ([contributor.givenName, contributor.familyName].filter(Boolean).join(" ") ||
      undefined)
  if (!name) return undefined

  const person: Descriptor = { name }
  if (contributor.givenName || contributor.familyName) person.nameType = "Personal"
  if (contributor.givenName) person.givenName = contributor.givenName
  if (contributor.familyName) person.familyName = contributor.familyName
  if (contributor.organization) person.affiliation = [{ name: contributor.organization }]
  return person
}

function convertDoi(id?: string) {
  const doi = id?.replace(/^https?:\/\/(dx\.)?doi\.org\//, "")
  return doi?.match(/^10[.][0-9]{4,9}[/][^\s]+$/) ? doi : undefined
}
