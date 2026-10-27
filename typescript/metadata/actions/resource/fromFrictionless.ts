import { convertFileDialectFromFrictionless } from "../../actions/fileDialect/fromFrictionless.ts"
import type { FrictionlessRecorder } from "../../actions/frictionless/mapping.ts"
import { nestFrictionlessRecorder } from "../../actions/frictionless/mapping.ts"
import { convertNameFromFrictionless } from "../../actions/frictionless/name.ts"
import { getFileExtension } from "../../actions/path/general.ts"
import { convertRightsFromFrictionless } from "../../actions/rights/fromFrictionless.ts"
import { convertTableSchemaFromFrictionless } from "../../actions/tableSchema/fromFrictionless.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { FrictionlessResource } from "../../models/frictionless/resource.ts"

const HASH_TYPES = ["md5", "sha1", "sha256", "sha512"]
const MEDIA_TYPES: Record<string, string> = {
  "text/csv": "csv",
  "text/tab-separated-values": "tsv",
  "application/json": "json",
}

/**
 * Convert a Frictionless Data Resource to a Fairspec Resource descriptor
 * A `schema` or `dialect` path stays a path; loading it converts the file it names
 */
export function convertResourceFromFrictionless(
  frictionlessResource: FrictionlessResource,
  options?: { record?: FrictionlessRecorder },
) {
  const record: FrictionlessRecorder = options?.record ?? (() => {})
  const resource: Descriptor = {}
  const { path, data } = frictionlessResource

  if (path !== undefined) {
    resource.data = path
    record("/data", "/path", { verbatim: true })
  } else if (data !== undefined) {
    resource.data = convertInlineData(data)
    record("/data", "/data")
  }

  if (frictionlessResource.name) {
    resource.name = convertNameFromFrictionless(frictionlessResource.name)
    record("/name", "/name")
  }

  if (frictionlessResource.title) {
    resource.titles = [{ title: frictionlessResource.title }]
    record("/titles", "/title")
  }

  if (frictionlessResource.description) {
    resource.descriptions = [
      { description: frictionlessResource.description, descriptionType: "Abstract" },
    ]
    record("/descriptions", "/description")
  }

  if (frictionlessResource.licenses && frictionlessResource.licenses.length > 0) {
    resource.rightsList = convertRightsFromFrictionless(frictionlessResource.licenses, {
      record: nestFrictionlessRecorder(record, "/rightsList", "/licenses"),
    })
  }

  if (frictionlessResource.bytes !== undefined) {
    resource.sizes = [`${frictionlessResource.bytes} bytes`]
    record("/sizes", "/bytes")
  }

  if (frictionlessResource.hash) {
    const integrity = convertHash(frictionlessResource.hash)
    if (integrity) {
      resource.integrity = integrity
      record("/integrity", "/hash")
    }
  }

  const { dialect, schema } = frictionlessResource
  if (typeof dialect === "string") {
    resource.fileDialect = dialect
    record("/fileDialect", "/dialect")
  } else if (path !== undefined) {
    const firstPath = typeof path === "string" ? path : path[0]
    const format =
      frictionlessResource.format ??
      MEDIA_TYPES[frictionlessResource.mediatype ?? ""] ??
      (firstPath ? getFileExtension(firstPath) : undefined) ??
      ""
    const fileDialect = convertFileDialectFromFrictionless(dialect, {
      format,
      record: nestFrictionlessRecorder(record, "/fileDialect", "/dialect"),
    })
    if (fileDialect) {
      resource.fileDialect = fileDialect
      record("/fileDialect", dialect ? "/dialect" : "/format")
      record("/fileDialect/format", "/format")
    }
  }

  if (typeof schema === "string") {
    resource.tableSchema = schema
    record("/tableSchema", "/schema")
  } else if (schema) {
    resource.tableSchema = convertTableSchemaFromFrictionless(schema, {
      record: nestFrictionlessRecorder(record, "/tableSchema", "/schema"),
    })
    record("/tableSchema", "/schema")
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
