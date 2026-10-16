export type LineTerminator = "LF" | "CRLF" | "CR"

export type Quote = { type: "None" } | { type: "Some"; char: number }

export interface Header {
  hasHeaderRow: boolean
  numPreambleRows: number
}

export interface Dialect {
  delimiter: number
  header: Header
  quote: Quote
  flexible: boolean
  isUtf8: boolean
  lineTerminator: LineTerminator
}

export interface Metadata {
  dialect: Dialect
  avgRecordLen: number
  numFields: number
  fields: string[]
}
