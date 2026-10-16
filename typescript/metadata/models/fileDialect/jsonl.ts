import { z } from "zod"
import { BaseFileDialect } from "./base.ts"
import {
  ColumnNames,
  CommentPrefix,
  CommentRows,
  HeaderJoin,
  HeaderRows,
  RowType,
} from "./common.ts"

export const JsonlFileDialect = BaseFileDialect.extend({
  format: z.literal("jsonl"),
  rowType: RowType.optional(),
  headerRows: HeaderRows.optional(),
  headerJoin: HeaderJoin.optional(),
  commentRows: CommentRows.optional(),
  commentPrefix: CommentPrefix.optional(),
  columnNames: ColumnNames.optional(),
})

export type JsonlFileDialect = z.infer<typeof JsonlFileDialect>
