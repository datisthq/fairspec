import { copyDescriptor } from "../../actions/descriptor/copy.ts"
import { normalizePath } from "../../actions/path/normalize.ts"
import type { Resource } from "../../models/resource.ts"

export function normalizeResource(
  resource: Resource,
  options: {
    basepath?: string
  },
) {
  const { basepath } = options
  resource = copyDescriptor(resource)

  if (typeof resource.data === "string") {
    resource.data = normalizePath(resource.data, { basepath })
  }

  if (Array.isArray(resource.data)) {
    for (const [index, path] of resource.data.entries()) {
      if (typeof path === "string") {
        resource.data[index] = normalizePath(path, { basepath })
      }
    }
  }

  for (const name of ["fileDialect", "dataSchema", "tableSchema"] as const) {
    const property = resource[name]
    if (typeof property === "string") {
      resource[name] = normalizePath(property, { basepath })
    }
  }

  return resource
}
