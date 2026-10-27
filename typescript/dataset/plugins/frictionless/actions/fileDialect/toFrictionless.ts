import type { Descriptor, FileDialect } from "@fairspec/metadata"

const BUILTIN_FORMATS = [
  "csv",
  "tsv",
  "json",
  "jsonl",
  "xlsx",
  "ods",
  "parquet",
  "arrow",
  "sqlite",
]

/**
 * Convert a Fairspec File Dialect to a Frictionless resource format and Table Dialect
 */
export function convertFileDialectToFrictionless(fileDialect: FileDialect) {
  if (!BUILTIN_FORMATS.includes(fileDialect.format)) return undefined

  const dialect: Descriptor = {}

  for (const [key, value] of Object.entries(fileDialect)) {
    if (value === undefined) continue
    switch (key) {
      case "delimiter":
      case "lineTerminator":
      case "quoteChar":
      case "nullSequence":
      case "headerJoin":
      case "commentRows":
      case "sheetName":
      case "sheetNumber":
        dialect[key] = value
        break
      case "headerRows":
        if (value === false) dialect.header = false
        else dialect.headerRows = value
        break
      case "commentPrefix":
        dialect.commentChar = value
        break
      case "jsonPointer":
        if (typeof value === "string" && value !== "") {
          dialect.property = value.slice(1).replaceAll("~1", "/").replaceAll("~0", "~")
        }
        break
      case "rowType":
        dialect.itemType = value
        break
      case "columnNames":
        dialect.itemKeys = value
        break
      case "tableName":
        dialect.table = value
        break
    }
  }

  return { format: fileDialect.format, dialect }
}
