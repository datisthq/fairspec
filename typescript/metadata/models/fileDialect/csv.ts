import { z } from "zod"
import { BaseFileDialect } from "./base.ts"
import {
  ColumnNames,
  CommentPrefix,
  CommentRows,
  Delimiter,
  HeaderJoin,
  HeaderRows,
  LineTerminator,
  NullSequence,
  QuoteChar,
} from "./common.ts"

export const CsvFileDialect = BaseFileDialect.extend({
  format: z.literal("csv"),
  delimiter: Delimiter.optional(),
  lineTerminator: LineTerminator.optional(),
  quoteChar: QuoteChar.optional(),
  nullSequence: NullSequence.optional(),
  headerRows: HeaderRows.optional(),
  headerJoin: HeaderJoin.optional(),
  commentRows: CommentRows.optional(),
  commentPrefix: CommentPrefix.optional(),
  columnNames: ColumnNames.optional(),
})

export type CsvFileDialect = z.infer<typeof CsvFileDialect>
