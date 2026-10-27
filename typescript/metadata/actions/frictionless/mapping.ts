import type { FairspecError } from "../../models/error/error.ts"

/**
 * How a converted Frictionless descriptor maps back to the one the user wrote
 */
export type FrictionlessMapping = {
  pointers: { fairspec: string; frictionless: string; verbatim: boolean }[]
  resourceNames: Record<string, string>
}

/**
 * Record that a Fairspec pointer was written from a Frictionless one; a verbatim value
 * was copied as it is, so pointers below it map key for key
 */
export type FrictionlessRecorder = (
  fairspec: string,
  frictionless: string,
  options?: { verbatim?: boolean },
) => void

export function createFrictionlessMapping(): FrictionlessMapping {
  return { pointers: [], resourceNames: {} }
}

export function createFrictionlessRecorder(
  mapping: FrictionlessMapping,
  prefix?: { fairspec: string; frictionless: string },
): FrictionlessRecorder {
  return (fairspec, frictionless, options) => {
    mapping.pointers.push({
      fairspec: (prefix?.fairspec ?? "") + fairspec,
      frictionless: (prefix?.frictionless ?? "") + frictionless,
      verbatim: options?.verbatim ?? false,
    })
  }
}

export function nestFrictionlessRecorder(
  record: FrictionlessRecorder,
  fairspec: string,
  frictionless: string,
): FrictionlessRecorder {
  return (innerFairspec, innerFrictionless, options) =>
    record(fairspec + innerFairspec, frictionless + innerFrictionless, options)
}

/**
 * Rewrite errors about a converted descriptor to point at the Frictionless one
 */
export function mapFrictionlessErrors(
  errors: FairspecError[],
  mapping: FrictionlessMapping,
) {
  const names = mapping.resourceNames

  return errors.map(error => {
    let mapped = error

    if ("jsonPointer" in mapped) {
      mapped = {
        ...mapped,
        jsonPointer: mapFrictionlessPointer(mapped.jsonPointer, mapping),
      }
    }

    if (mapped.resourceName && names[mapped.resourceName]) {
      mapped = { ...mapped, resourceName: names[mapped.resourceName] }
    }

    if (mapped.type === "foreignKey") {
      const resource = mapped.foreignKey.reference.resource
      if (resource && names[resource]) {
        const reference = { ...mapped.foreignKey.reference, resource: names[resource] }
        mapped = { ...mapped, foreignKey: { ...mapped.foreignKey, reference } }
      }
    }

    return mapped
  })
}

function mapFrictionlessPointer(pointer: string, mapping: FrictionlessMapping) {
  let best: FrictionlessMapping["pointers"][number] | undefined
  for (const entry of mapping.pointers) {
    const isPrefix =
      pointer === entry.fairspec || pointer.startsWith(`${entry.fairspec}/`)
    if (isPrefix && (!best || entry.fairspec.length > best.fairspec.length)) best = entry
  }

  if (!best) return pointer
  if (!best.verbatim) return best.frictionless
  return best.frictionless + pointer.slice(best.fairspec.length)
}
