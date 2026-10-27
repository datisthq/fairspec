import {
  assertDataset,
  getIsDescriptor,
  loadDescriptor,
  normalizePath,
  resolveBasepath,
} from "@fairspec/metadata"
import { assertFrictionlessModel } from "../../helpers/inspect.ts"
import { FrictionlessDialect } from "../../models/dialect.ts"
import { FrictionlessPackage } from "../../models/package.ts"
import { FrictionlessSchema } from "../../models/schema.ts"
import { convertDatasetFromFrictionless } from "./fromFrictionless.ts"

/**
 * Load a Frictionless Data Package (v1 or v2) as a Dataset
 * A `datapackage.json` holding a Fairspec Dataset loads as it is
 */
export async function loadDatasetFromFrictionless(path: string) {
  const basepath = await resolveBasepath(path)
  const descriptor = await loadDescriptor(path)

  const $schema = descriptor.$schema
  if (typeof $schema === "string" && $schema.startsWith("https://fairspec.org/")) {
    return await assertDataset(descriptor, { basepath })
  }

  const frictionlessPackage = assertFrictionlessModel(descriptor, {
    model: FrictionlessPackage,
  })

  const resources = await Promise.all(
    frictionlessPackage.resources.map(async (resource, index) => {
      const jsonPointer = `/resources/${index}`
      const schema =
        typeof resource.schema === "string"
          ? assertFrictionlessModel(await loadReference(resource.schema, basepath), {
              model: FrictionlessSchema,
              jsonPointer: `${jsonPointer}/schema`,
            })
          : resource.schema
      const dialect =
        typeof resource.dialect === "string"
          ? assertFrictionlessModel(await loadReference(resource.dialect, basepath), {
              model: FrictionlessDialect,
              jsonPointer: `${jsonPointer}/dialect`,
            })
          : resource.dialect
      return { ...resource, schema, dialect }
    }),
  )

  const dataset = convertDatasetFromFrictionless({ ...frictionlessPackage, resources })
  return await assertDataset(dataset, { basepath })
}

async function loadReference(path: string, basepath: string) {
  const descriptor = await loadDescriptor(normalizePath(path, { basepath }))
  if (!getIsDescriptor(descriptor)) {
    throw new Error(`Referenced descriptor is not a JSON object: ${path}`)
  }
  return descriptor
}
