import type {
  CsvFileDialect,
  JsonFileDialect,
  JsonlFileDialect,
  OdsFileDialect,
  SqliteFileDialect,
  TsvFileDialect,
  XlsxFileDialect,
} from "@fairspec/library"
import { inferFileDialectFormat } from "@fairspec/library"
import type { FileDialectOptions } from "../models/fileDialect.ts"

export function createFileDialectFromPathAndOptions(
  path: string,
  options: FileDialectOptions,
) {
  const format = options.format ?? inferFileDialectFormat({ data: path })

  if (format === "csv" || format === "tsv") {
    const fileDialect: CsvFileDialect | TsvFileDialect = { format }

    if (options.lineTerminator) fileDialect.lineTerminator = options.lineTerminator
    if (options.nullSequence) fileDialect.nullSequence = options.nullSequence
    if (options.headerRows) fileDialect.headerRows = options.headerRows
    if (options.headerJoin) fileDialect.headerJoin = options.headerJoin
    if (options.commentRows) fileDialect.commentRows = options.commentRows
    if (options.commentPrefix) fileDialect.commentPrefix = options.commentPrefix
    if (options.columnNames) fileDialect.columnNames = options.columnNames

    if (fileDialect.format === "csv") {
      if (options.delimiter) fileDialect.delimiter = options.delimiter
      if (options.quoteChar) fileDialect.quoteChar = options.quoteChar
    }

    return fileDialect
  }

  if (format === "xlsx" || format === "ods") {
    const fileDialect: XlsxFileDialect | OdsFileDialect = { format }

    if (options.sheetNumber) fileDialect.sheetNumber = options.sheetNumber
    if (options.sheetName) fileDialect.sheetName = options.sheetName
    if (options.headerRows) fileDialect.headerRows = options.headerRows
    if (options.headerJoin) fileDialect.headerJoin = options.headerJoin
    if (options.commentRows) fileDialect.commentRows = options.commentRows
    if (options.commentPrefix) fileDialect.commentPrefix = options.commentPrefix

    return fileDialect
  }

  if (format === "json" || format === "jsonl") {
    const fileDialect: JsonFileDialect | JsonlFileDialect = { format }

    if (options.headerRows) fileDialect.headerRows = options.headerRows
    if (options.headerJoin) fileDialect.headerJoin = options.headerJoin
    if (options.commentRows) fileDialect.commentRows = options.commentRows
    if (options.commentPrefix) fileDialect.commentPrefix = options.commentPrefix
    if (options.rowType) fileDialect.rowType = options.rowType

    if (fileDialect?.format === "json") {
      if (options.jsonPointer) fileDialect.jsonPointer = options.jsonPointer
    }

    return fileDialect
  }

  if (format === "sqlite") {
    const fileDialect: SqliteFileDialect = { format }

    if (options.tableName) fileDialect.tableName = options.tableName

    return fileDialect
  }

  return undefined
}
