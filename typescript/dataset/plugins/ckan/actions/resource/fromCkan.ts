import type { Resource } from "@fairspec/metadata"
import { getFileName } from "@fairspec/metadata"
import { convertTableSchemaFromCkan } from "../../actions/tableSchema/fromCkan.ts"
import type { CkanResource } from "../../models/resource.ts"

export function convertResourceFromCkan(ckanResource: CkanResource): Resource {
  const resource: Resource = {
    data: ckanResource.url,
    unstable_customMetadata: {
      ckanKey: getFileName(ckanResource.url),
      ckanUrl: ckanResource.url,
      ckanId: ckanResource.id,
    },
  }

  if (ckanResource.name) {
    resource.name = convertName(ckanResource.name)
  }

  if (ckanResource.description) {
    resource.descriptions = [
      {
        description: ckanResource.description,
        descriptionType: "Abstract",
      },
    ]
  }

  if (ckanResource.size) {
    resource.sizes = [`${ckanResource.size} bytes`]
  }

  if (ckanResource.hash) {
    resource.integrity = {
      type: "md5",
      hash: ckanResource.hash,
    }
  }

  if (ckanResource.created) {
    resource.dates = [
      {
        date: ckanResource.created,
        dateType: "Created",
      },
    ]
  }

  if (ckanResource.last_modified) {
    resource.dates = [
      ...(resource.dates || []),
      {
        date: ckanResource.last_modified,
        dateType: "Updated",
      },
    ]
  }

  if (ckanResource.schema) {
    resource.tableSchema = convertTableSchemaFromCkan(ckanResource.schema)
  }

  return resource
}

function convertName(name: string): string {
  return name
    .replace(/[\s.()/\\,]+/g, "_")
    .toLowerCase()
    .replace(/[^a-z0-9_-]/g, "")
    .replace(/^(\d)/, "_$1")
    .slice(0, 100)
}
