import type { Descriptor } from "@fairspec/metadata"
import type { FrictionlessDialect } from "../../models/dialect.ts"

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
 * Returns undefined for a format Fairspec does not read as a table
 */
export function convertFileDialectFromFrictionless(
  dialect: FrictionlessDialect | undefined,
  options: { format?: string },
) {
  const format = FORMATS[options.format?.toLowerCase() ?? ""]
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

  return fileDialect
}
