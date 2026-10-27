import { convertDatasetFromFrictionless } from "../../actions/dataset/fromFrictionless.ts"
import { inspectModel } from "../../actions/descriptor/inspect.ts"
import { convertFileDialectFromFrictionless } from "../../actions/fileDialect/fromFrictionless.ts"
import { convertTableSchemaFromFrictionless } from "../../actions/tableSchema/fromFrictionless.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { MetadataError } from "../../models/error/metadata.ts"
import { FrictionlessDialect } from "../../models/frictionless/dialect.ts"
import { FrictionlessPackage } from "../../models/frictionless/package.ts"
import { FrictionlessSchema } from "../../models/frictionless/schema.ts"
import type { FrictionlessMapping } from "./mapping.ts"
import { createFrictionlessMapping, createFrictionlessRecorder } from "./mapping.ts"

export type FrictionlessKind = "dataset" | "tableSchema" | "fileDialect"

const FRICTIONLESS_RESOURCE_KEYS = ["path", "schema", "dialect"]
const FRICTIONLESS_DIALECT_KEYS = [
  "header",
  "headerRows",
  "headerJoin",
  "commentRows",
  "commentChar",
  "delimiter",
  "lineTerminator",
  "quoteChar",
  "doubleQuote",
  "escapeChar",
  "nullSequence",
  "skipInitialSpace",
  "property",
  "itemType",
  "itemKeys",
  "sheetNumber",
  "sheetName",
  "table",
]

/**
 * Whether a descriptor is the Frictionless counterpart of a Dataset, Table Schema or
 * File Dialect, judged by its content; a Fairspec `$schema` always wins
 */
export function getIsFrictionlessDescriptor(
  descriptor: Descriptor,
  options: { kind: FrictionlessKind },
) {
  const $schema = typeof descriptor.$schema === "string" ? descriptor.$schema : ""
  if ($schema.startsWith("https://fairspec.org/")) return false

  switch (options.kind) {
    case "tableSchema":
      return Array.isArray(descriptor.fields) && !("properties" in descriptor)
    case "fileDialect":
      return (
        !("format" in descriptor) &&
        FRICTIONLESS_DIALECT_KEYS.some(key => key in descriptor)
      )
    case "dataset": {
      if (typeof descriptor.profile === "string") return true
      if (/datapackage\.org|frictionlessdata\.io/.test($schema)) return true
      const resources = Array.isArray(descriptor.resources) ? descriptor.resources : []
      return resources.some(
        resource =>
          typeof resource === "object" &&
          resource !== null &&
          FRICTIONLESS_RESOURCE_KEYS.some(key => key in resource),
      )
    }
  }
}

/**
 * Convert a Frictionless descriptor to its Fairspec counterpart, with the mapping that
 * points errors about the result back at the original
 * A descriptor the Frictionless model cannot read comes back as metadata errors instead
 */
export function convertDescriptorFromFrictionless(
  descriptor: Descriptor,
  options: { kind: FrictionlessKind; rootJsonPointer?: string },
): { descriptor?: Descriptor; mapping: FrictionlessMapping; errors: MetadataError[] } {
  const mapping = createFrictionlessMapping()
  const rootJsonPointer = options.rootJsonPointer ?? ""

  switch (options.kind) {
    case "dataset": {
      const parsed = inspectModel(descriptor, {
        model: FrictionlessPackage,
        rootJsonPointer,
      })
      if (!parsed.data) return { mapping, errors: parsed.errors }
      return {
        descriptor: convertDatasetFromFrictionless(parsed.data, { mapping }),
        mapping,
        errors: [],
      }
    }
    case "tableSchema": {
      const parsed = inspectModel(descriptor, {
        model: FrictionlessSchema,
        rootJsonPointer,
      })
      if (!parsed.data) return { mapping, errors: parsed.errors }
      const record = createFrictionlessRecorder(mapping, {
        fairspec: rootJsonPointer,
        frictionless: rootJsonPointer,
      })
      return {
        descriptor: convertTableSchemaFromFrictionless(parsed.data, { record }),
        mapping,
        errors: [],
      }
    }
    case "fileDialect": {
      const parsed = inspectModel(descriptor, {
        model: FrictionlessDialect,
        rootJsonPointer,
      })
      if (!parsed.data) return { mapping, errors: parsed.errors }
      const record = createFrictionlessRecorder(mapping, {
        fairspec: rootJsonPointer,
        frictionless: rootJsonPointer,
      })
      return {
        descriptor: convertFileDialectFromFrictionless(parsed.data, { record }),
        mapping,
        errors: [],
      }
    }
  }
}

/**
 * Convert a descriptor when it is Frictionless, else pass it through unchanged
 */
export function prepareFrictionlessDescriptor(
  descriptor: Descriptor,
  options: { kind: FrictionlessKind; rootJsonPointer?: string },
): { descriptor?: Descriptor; mapping?: FrictionlessMapping; errors: MetadataError[] } {
  if (!getIsFrictionlessDescriptor(descriptor, options)) return { descriptor, errors: [] }
  return convertDescriptorFromFrictionless(descriptor, options)
}
