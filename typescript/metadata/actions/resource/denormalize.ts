import { copyDescriptor } from "../../actions/descriptor/copy.ts"
import { denormalizePath } from "../../actions/path/denormalize.ts"
import type { Resource } from "../../models/resource.ts"

export function denormalizeResource(
  resource: Resource,
  options: {
    basepath?: string
  },
) {
  const { basepath } = options
  resource = copyDescriptor(resource)

  if (typeof resource.data === "string") {
    resource.data = denormalizePath(resource.data, { basepath })
  }

  if (Array.isArray(resource.data)) {
    for (const [index, path] of resource.data.entries()) {
      if (typeof path === "string") {
        resource.data[index] = denormalizePath(path, { basepath })
      }
    }
  }

  for (const name of ["fileDialect", "dataSchema", "tableSchema"] as const) {
    const property = resource[name]
    if (typeof property === "string") {
      resource[name] = denormalizePath(property, { basepath })
    }
  }

  return resource
}
