import type { FrictionlessRecorder } from "../../actions/frictionless/mapping.ts"
import type { Descriptor } from "../../models/descriptor.ts"
import type { FrictionlessDialect } from "../../models/frictionless/dialect.ts"

const FORMATS: Record<string, string> = {
  csv: "csv",
  tsv: "tsv",
  json: "json",
  jsonl: "jsonl",
  ndjson: "jsonl",
  xlsx: "xlsx",
  ods: "ods",
  parquet: "parquet",
  arrow: "arrow",
  feather: "arrow",
  sqlite: "sqlite",
}

/**
 * Convert a Frictionless Table Dialect to a Fairspec File Dialect descriptor
 * Returns undefined for a format Fairspec does not read as a table; without a format, a
 * standalone dialect takes the one its keys belong to
 */
export function convertFileDialectFromFrictionless(
  dialect: FrictionlessDialect | undefined,
  options: { format?: string; record?: FrictionlessRecorder },
) {
  const format =
    options.format === undefined && dialect
      ? inferFormat(dialect)
      : FORMATS[options.format?.toLowerCase() ?? ""]
  if (!format) return undefined

  const fileDialect: Descriptor = { format }
  if (!dialect) return fileDialect

  switch (format) {
    case "csv":
      if (dialect.delimiter !== undefined) fileDialect.delimiter = dialect.delimiter
      if (dialect.lineTerminator !== undefined) {
        fileDialect.lineTerminator = dialect.lineTerminator
      }
      if (dialect.quoteChar !== undefined) fileDialect.quoteChar = dialect.quoteChar
      if (dialect.nullSequence !== undefined)
        fileDialect.nullSequence = dialect.nullSequence
      break
    case "tsv":
      if (dialect.lineTerminator !== undefined) {
        fileDialect.lineTerminator = dialect.lineTerminator
      }
      if (dialect.nullSequence !== undefined)
        fileDialect.nullSequence = dialect.nullSequence
      break
    case "json":
      if (dialect.property !== undefined) {
        fileDialect.jsonPointer = `/${dialect.property.replaceAll("~", "~0").replaceAll("/", "~1")}`
      }
      if (dialect.itemType) fileDialect.rowType = dialect.itemType
      break
    case "jsonl":
      if (dialect.itemType) fileDialect.rowType = dialect.itemType
      break
    case "xlsx":
    case "ods":
      if (dialect.sheetName !== undefined) fileDialect.sheetName = dialect.sheetName
      if (dialect.sheetNumber !== undefined) fileDialect.sheetNumber = dialect.sheetNumber
      break
    case "sqlite":
      if (dialect.table !== undefined) fileDialect.tableName = dialect.table
      recordKeys(fileDialect, options.record)
      return fileDialect
    default:
      return fileDialect
  }

  if (dialect.header === false) {
    fileDialect.headerRows = false
  } else if (dialect.headerRows) {
    fileDialect.headerRows = dialect.headerRows
  }
  if (dialect.headerJoin !== undefined) fileDialect.headerJoin = dialect.headerJoin
  if (dialect.commentRows) fileDialect.commentRows = dialect.commentRows
  if (dialect.commentChar !== undefined) fileDialect.commentPrefix = dialect.commentChar
  if (format.startsWith("json") && dialect.itemKeys) {
    fileDialect.columnNames = dialect.itemKeys
  }

  recordKeys(fileDialect, options.record)
  return fileDialect
}

const DIALECT_KEYS: Record<string, string> = {
  delimiter: "delimiter",
  lineTerminator: "lineTerminator",
  quoteChar: "quoteChar",
  nullSequence: "nullSequence",
  headerJoin: "headerJoin",
  commentRows: "commentRows",
  commentPrefix: "commentChar",
  jsonPointer: "property",
  rowType: "itemType",
  columnNames: "itemKeys",
  sheetName: "sheetName",
  sheetNumber: "sheetNumber",
  tableName: "table",
}

function recordKeys(fileDialect: Descriptor, record?: FrictionlessRecorder) {
  if (!record) return
  for (const key of Object.keys(fileDialect)) {
    if (key === "headerRows") {
      record("/headerRows", fileDialect.headerRows === false ? "/header" : "/headerRows")
    } else if (DIALECT_KEYS[key]) {
      record(`/${key}`, `/${DIALECT_KEYS[key]}`)
    }
  }
}

function inferFormat(dialect: FrictionlessDialect) {
  if (dialect.property !== undefined || dialect.itemType || dialect.itemKeys)
    return "json"
  if (dialect.sheetName !== undefined || dialect.sheetNumber !== undefined) return "xlsx"
  if (dialect.table !== undefined) return "sqlite"
  return "csv"
}
