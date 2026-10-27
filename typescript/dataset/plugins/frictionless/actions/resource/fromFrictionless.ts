import type { Descriptor } from "@fairspec/metadata"
import { getFileExtension } from "@fairspec/metadata"
import { convertNameFromFrictionless } from "../../helpers/name.ts"
import type { ResolvedFrictionlessResource } from "../../models/resource.ts"
import { convertFileDialectFromFrictionless } from "../fileDialect/fromFrictionless.ts"
import { convertTableSchemaFromFrictionless } from "../tableSchema/fromFrictionless.ts"
import { convertRightsFromFrictionless } from "../rights/fromFrictionless.ts"

const HASH_TYPES = ["md5", "sha1", "sha256", "sha512"]
const MEDIA_TYPES: Record<string, string> = {
  "text/csv": "csv",
  "text/tab-separated-values": "tsv",
  "application/json": "json",
}

/**
 * Convert a Frictionless Data Resource to a Fairspec Resource descriptor
 * Its `schema` and `dialect` must already be objects, not paths
 */
export function convertResourceFromFrictionless(
  frictionlessResource: ResolvedFrictionlessResource,
) {
  const resource: Descriptor = {}
  const { path, data } = frictionlessResource

  if (path !== undefined) {
    resource.data = path
  } else if (data !== undefined) {
    resource.data = convertInlineData(data)
  }

  if (frictionlessResource.name) {
    resource.name = convertNameFromFrictionless(frictionlessResource.name)
  }

  if (frictionlessResource.title) {
    resource.titles = [{ title: frictionlessResource.title }]
  }

  if (frictionlessResource.description) {
    resource.descriptions = [
      { description: frictionlessResource.description, descriptionType: "Abstract" },
    ]
  }

  if (frictionlessResource.licenses && frictionlessResource.licenses.length > 0) {
    resource.rightsList = convertRightsFromFrictionless(frictionlessResource.licenses)
  }

  if (frictionlessResource.bytes !== undefined) {
    resource.sizes = [`${frictionlessResource.bytes} bytes`]
  }

  if (frictionlessResource.hash) {
    const integrity = convertHash(frictionlessResource.hash)
    if (integrity) resource.integrity = integrity
  }

  if (path !== undefined) {
    const firstPath = typeof path === "string" ? path : path[0]
    const format =
      frictionlessResource.format ??
      MEDIA_TYPES[frictionlessResource.mediatype ?? ""] ??
      (firstPath ? getFileExtension(firstPath) : undefined)
    const fileDialect = convertFileDialectFromFrictionless(frictionlessResource.dialect, {
      format,
    })
    if (fileDialect) resource.fileDialect = fileDialect
  }

  if (frictionlessResource.schema) {
    resource.tableSchema = convertTableSchemaFromFrictionless(frictionlessResource.schema)
  }

  return resource
}

function convertInlineData(data: unknown) {
  if (!Array.isArray(data) || !data.every(row => Array.isArray(row))) return data

  const [header = [], ...rows] = data
  return rows.map(row =>
    Object.fromEntries(header.map((name, index) => [String(name), row[index] ?? null])),
  )
}

function convertHash(hash: string) {
  const [type, value] = hash.includes(":") ? hash.split(":", 2) : ["md5", hash]
  if (!type || !value || !HASH_TYPES.includes(type)) return undefined
  return { type, hash: value }
}
