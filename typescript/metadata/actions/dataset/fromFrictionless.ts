import type { FrictionlessMapping } from "../../actions/frictionless/mapping.ts"
import {
  createFrictionlessRecorder,
  nestFrictionlessRecorder,
} from "../../actions/frictionless/mapping.ts"
import { convertResourceFromFrictionless } from "../../actions/resource/fromFrictionless.ts"
import { convertRightsFromFrictionless } from "../../actions/rights/fromFrictionless.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type {
  FrictionlessContributor,
  FrictionlessPackage,
} from "../../models/frictionless/package.ts"

const CONTRIBUTOR_TYPES: Record<string, string> = {
  maintainer: "ContactPerson",
  wrangler: "DataCurator",
  publisher: "Distributor",
}

/**
 * Convert a Frictionless Data Package to a Fairspec Dataset descriptor
 * A resource's `schema` or `dialect` path stays a path
 */
export function convertDatasetFromFrictionless(
  frictionlessPackage: FrictionlessPackage,
  options?: { mapping?: FrictionlessMapping },
) {
  const mapping = options?.mapping
  const record = mapping ? createFrictionlessRecorder(mapping) : () => {}
  const dataset: Descriptor = {}

  const doi = convertDoi(frictionlessPackage.id)
  if (doi) {
    dataset.doi = doi
    record("/doi", "/id")
  }

  const creators: Descriptor[] = []
  const contributors: Descriptor[] = []
  for (const [index, contributor] of (frictionlessPackage.contributors ?? []).entries()) {
    const person = convertPerson(contributor)
    if (!person) continue

    const roles = contributor.roles ?? (contributor.role ? [contributor.role] : [])
    if (roles.includes("author") || roles.includes("creator")) {
      record(`/creators/${creators.length}`, `/contributors/${index}`)
      creators.push(person)
    } else {
      const role = roles.find(role => CONTRIBUTOR_TYPES[role])
      record(`/contributors/${contributors.length}`, `/contributors/${index}`)
      contributors.push({
        ...person,
        contributorType: role ? CONTRIBUTOR_TYPES[role] : "Other",
      })
    }
  }

  if (creators.length > 0) dataset.creators = creators

  if (frictionlessPackage.title) {
    dataset.titles = [{ title: frictionlessPackage.title }]
    record("/titles", "/title")
  }

  if (frictionlessPackage.keywords && frictionlessPackage.keywords.length > 0) {
    dataset.subjects = frictionlessPackage.keywords.map(subject => ({ subject }))
    for (const index of frictionlessPackage.keywords.keys()) {
      record(`/subjects/${index}`, `/keywords/${index}`)
    }
  }

  if (contributors.length > 0) dataset.contributors = contributors

  if (frictionlessPackage.created) {
    dataset.dates = [{ date: frictionlessPackage.created, dateType: "Created" }]
    record("/dates", "/created")
  }

  const relatedIdentifiers: Descriptor[] = []
  if (frictionlessPackage.homepage) {
    record(`/relatedIdentifiers/${relatedIdentifiers.length}`, "/homepage")
    relatedIdentifiers.push({
      relatedIdentifier: frictionlessPackage.homepage,
      relatedIdentifierType: "URL",
      relationType: "IsDocumentedBy",
    })
  }
  for (const [index, source] of (frictionlessPackage.sources ?? []).entries()) {
    if (!source.path?.match(/^https?:\/\//)) continue
    record(`/relatedIdentifiers/${relatedIdentifiers.length}`, `/sources/${index}`)
    relatedIdentifiers.push({
      relatedIdentifier: source.path,
      relatedIdentifierType: "URL",
      relationType: "IsDerivedFrom",
    })
  }
  if (relatedIdentifiers.length > 0) dataset.relatedIdentifiers = relatedIdentifiers

  if (frictionlessPackage.version) {
    dataset.version = frictionlessPackage.version
    record("/version", "/version")
  }

  if (frictionlessPackage.licenses && frictionlessPackage.licenses.length > 0) {
    dataset.rightsList = convertRightsFromFrictionless(frictionlessPackage.licenses, {
      record: nestFrictionlessRecorder(record, "/rightsList", "/licenses"),
    })
  }

  if (frictionlessPackage.description) {
    dataset.descriptions = [
      { description: frictionlessPackage.description, descriptionType: "Abstract" },
    ]
    record("/descriptions", "/description")
  }

  dataset.resources = frictionlessPackage.resources.map((resource, index) => {
    const prefix = `/resources/${index}`
    record(prefix, prefix, { verbatim: true })
    const converted = convertResourceFromFrictionless(resource, {
      record: nestFrictionlessRecorder(record, prefix, prefix),
    })
    if (mapping && resource.name && typeof converted.name === "string") {
      mapping.resourceNames[converted.name] = resource.name
    }
    return converted
  })
  record("/resources", "/resources", { verbatim: true })

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
