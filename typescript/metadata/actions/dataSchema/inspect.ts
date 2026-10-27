import type { Descriptor } from "../../models/descriptor.ts"

const META_SCHEMA_URL = "https://json-schema.org/draft/2020-12/schema"
const PROFILE_REGEX = /^https:\/\/fairspec\.org\/profiles\/[^/]+\/data-schema\.json$/

const SCHEMA_MAP_KEYWORDS = [
  "$defs",
  "definitions",
  "properties",
  "patternProperties",
  "dependentSchemas",
]
const SCHEMA_KEYWORDS = [
  "additionalProperties",
  "unevaluatedProperties",
  "items",
  "additionalItems",
  "unevaluatedItems",
  "contains",
  "propertyNames",
  "not",
  "if",
  "then",
  "else",
  "contentSchema",
]
const SCHEMA_LIST_KEYWORDS = ["allOf", "anyOf", "oneOf", "prefixItems"]

/**
 * Tells the `$schema` a Data Schema may not name, and every `$ref` that does not
 * resolve within the document, which is never fetched
 */
export function inspectDataSchema(
  descriptor: Descriptor,
  options?: { rootJsonPointer?: string },
) {
  const rootPath = options?.rootJsonPointer ?? ""
  const errors: { message: string; jsonPointer: string }[] = []

  const $schema = descriptor.$schema
  if (
    typeof $schema === "string" &&
    $schema !== META_SCHEMA_URL &&
    !PROFILE_REGEX.test($schema)
  ) {
    errors.push({
      message:
        "must be a Fairspec Data Schema profile or the JSON Schema 2020-12 dialect",
      jsonPointer: `${rootPath}/$schema`,
    })
    return errors
  }

  const schemas = listSubschemas(descriptor, "")
  for (const [schema, pointer] of schemas) {
    for (const keyword of ["$ref", "$dynamicRef"]) {
      const $ref = schema[keyword]
      if (typeof $ref === "string" && !getIsLocalRef($ref, descriptor, schemas)) {
        errors.push({
          message: "must resolve within the data schema",
          jsonPointer: `${rootPath}${pointer}/${keyword}`,
        })
      }
    }
  }

  return errors
}

function listSubschemas(schema: unknown, pointer: string): [Descriptor, string][] {
  if (!isObject(schema)) return []

  const result: [Descriptor, string][] = [[schema, pointer]]
  for (const keyword of SCHEMA_MAP_KEYWORDS) {
    const map = schema[keyword]
    if (!isObject(map)) continue
    for (const [key, value] of Object.entries(map)) {
      result.push(...listSubschemas(value, `${pointer}/${keyword}/${escapeToken(key)}`))
    }
  }
  for (const keyword of [...SCHEMA_KEYWORDS, ...SCHEMA_LIST_KEYWORDS]) {
    const value = schema[keyword]
    if (Array.isArray(value)) {
      value.forEach((item, index) => {
        result.push(...listSubschemas(item, `${pointer}/${keyword}/${index}`))
      })
    } else {
      result.push(...listSubschemas(value, `${pointer}/${keyword}`))
    }
  }
  return result
}

function getIsLocalRef($ref: string, root: Descriptor, schemas: [Descriptor, string][]) {
  const hashIndex = $ref.indexOf("#")
  const base = hashIndex === -1 ? $ref : $ref.slice(0, hashIndex)
  const fragment = hashIndex === -1 ? "" : $ref.slice(hashIndex + 1)

  let target: Descriptor | undefined = root
  if (base) {
    target = schemas.find(([schema]) => schema.$id === base)?.[0]
    if (!target) return false
  }

  if (!fragment) return true
  if (!fragment.startsWith("/")) {
    return schemas.some(
      ([schema]) => schema.$anchor === fragment || schema.$dynamicAnchor === fragment,
    )
  }

  let decoded: string
  try {
    decoded = decodeURIComponent(fragment)
  } catch {
    return false
  }

  let value: unknown = target
  for (const token of decoded.slice(1).split("/")) {
    const key = token.replaceAll("~1", "/").replaceAll("~0", "~")
    if (Array.isArray(value)) {
      if (!/^(?:0|[1-9][0-9]*)$/.test(key)) return false
      value = value[Number(key)]
    } else if (isObject(value) && Object.hasOwn(value, key)) {
      value = value[key]
    } else {
      return false
    }
    if (value === undefined) return false
  }
  return true
}

function escapeToken(token: string) {
  return token.replaceAll("~", "~0").replaceAll("/", "~1")
}

function isObject(value: unknown): value is Descriptor {
  return typeof value === "object" && value !== null && !Array.isArray(value)
}
