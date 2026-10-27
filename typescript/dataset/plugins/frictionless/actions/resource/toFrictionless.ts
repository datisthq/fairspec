import type { Descriptor, Resource } from "@fairspec/metadata"
import { inferResourceName } from "@fairspec/metadata"
import { convertRightsToFrictionless } from "../rights/toFrictionless.ts"
import { convertFileDialectToFrictionless } from "../fileDialect/toFrictionless.ts"
import { convertTableSchemaToFrictionless } from "../tableSchema/toFrictionless.ts"

/**
 * Convert a denormalized Fairspec Resource to a Frictionless Data Resource descriptor
 * Its `tableSchema` and `fileDialect` must already be objects, not paths
 */
export function convertResourceToFrictionless(
  resource: Resource,
  options: { resourceNumber: number },
) {
  const frictionlessResource: Descriptor = {
    name: resource.name ?? inferResourceName(resource, options),
  }

  if (typeof resource.data === "string" || isPathList(resource.data)) {
    frictionlessResource.path = resource.data
  } else if (resource.data !== undefined) {
    frictionlessResource.data = resource.data
  }

  if (resource.tableSchema && typeof resource.tableSchema !== "string") {
    frictionlessResource.type = "table"
  }

  const title = resource.titles?.[0]?.title
  if (title) frictionlessResource.title = title

  const description = resource.descriptions?.[0]?.description
  if (description) frictionlessResource.description = description

  if (resource.fileDialect && typeof resource.fileDialect !== "string") {
    const converted = convertFileDialectToFrictionless(resource.fileDialect)
    if (converted) {
      frictionlessResource.format = converted.format
      if (Object.keys(converted.dialect).length > 0) {
        frictionlessResource.dialect = converted.dialect
      }
    }
  }

  if (resource.integrity) {
    frictionlessResource.hash = `${resource.integrity.type}:${resource.integrity.hash}`
  }

  const bytes = resource.sizes
    ?.map(size => size.match(/^(\d+) bytes$/)?.[1])
    .find(Boolean)
  if (bytes) frictionlessResource.bytes = Number(bytes)

  if (resource.rightsList && resource.rightsList.length > 0) {
    frictionlessResource.licenses = convertRightsToFrictionless(resource.rightsList)
  }

  if (resource.tableSchema && typeof resource.tableSchema !== "string") {
    frictionlessResource.schema = convertTableSchemaToFrictionless(resource.tableSchema)
  }

  return frictionlessResource
}

function isPathList(data: unknown): data is string[] {
  return (
    Array.isArray(data) && data.length > 0 && data.every(item => typeof item === "string")
  )
}
