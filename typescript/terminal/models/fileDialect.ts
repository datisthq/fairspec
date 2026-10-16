export type FileDialectOptions = {
  format?: string
  delimiter?: string
  lineTerminator?: string
  quoteChar?: string
  nullSequence?: string
  headerRows?: false | number[]
  headerJoin?: string
  commentRows?: number[]
  commentPrefix?: string
  columnNames?: string[]
  jsonPointer?: string
  rowType?: "array" | "object"
  sheetNumber?: number
  sheetName?: string
  tableName?: string
}
