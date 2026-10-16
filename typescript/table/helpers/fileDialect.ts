import type { FileDialectWithHeaderAndCommentRows } from "../models/fileDialect.ts"

export function getHeaderRows(fileDialect?: FileDialectWithHeaderAndCommentRows) {
  return fileDialect?.headerRows !== false ? (fileDialect?.headerRows ?? [1]) : []
}
