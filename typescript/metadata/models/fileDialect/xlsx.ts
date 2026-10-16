import { z } from "zod"
import { BaseFileDialect } from "./base.ts"
import {
  ColumnNames,
  CommentPrefix,
  CommentRows,
  HeaderJoin,
  HeaderRows,
  SheetName,
  SheetNumber,
} from "./common.ts"

export const XlsxFileDialect = BaseFileDialect.extend({
  format: z.literal("xlsx"),
  sheetName: SheetName.optional(),
  sheetNumber: SheetNumber.optional(),
  headerRows: HeaderRows.optional(),
  headerJoin: HeaderJoin.optional(),
  commentRows: CommentRows.optional(),
  commentPrefix: CommentPrefix.optional(),
  columnNames: ColumnNames.optional(),
})

export type XlsxFileDialect = z.infer<typeof XlsxFileDialect>
