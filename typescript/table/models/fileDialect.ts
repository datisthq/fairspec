import type { FileDialect } from "@fairspec/metadata"

export type FileDialectWithHeaderAndCommentRows = Extract<
  FileDialect,
  {
    headerRows?: any
    headerJoin?: any
    commentRows?: any
    commentPrefix?: any
    columnNames?: any
  }
>
