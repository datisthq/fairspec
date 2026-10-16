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

export const OdsFileDialect = BaseFileDialect.extend({
  format: z.literal("ods"),
  sheetName: SheetName.optional(),
  sheetNumber: SheetNumber.optional(),
  headerRows: HeaderRows.optional(),
  headerJoin: HeaderJoin.optional(),
  commentRows: CommentRows.optional(),
  commentPrefix: CommentPrefix.optional(),
  columnNames: ColumnNames.optional(),
})

export type OdsFileDialect = z.infer<typeof OdsFileDialect>
