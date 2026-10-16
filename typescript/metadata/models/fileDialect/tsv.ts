import { z } from "zod"
import { BaseFileDialect } from "./base.ts"
import {
  ColumnNames,
  CommentPrefix,
  CommentRows,
  HeaderJoin,
  HeaderRows,
  LineTerminator,
  NullSequence,
} from "./common.ts"

export const TsvFileDialect = BaseFileDialect.extend({
  format: z.literal("tsv"),
  lineTerminator: LineTerminator.optional(),
  nullSequence: NullSequence.optional(),
  headerRows: HeaderRows.optional(),
  headerJoin: HeaderJoin.optional(),
  commentRows: CommentRows.optional(),
  commentPrefix: CommentPrefix.optional(),
  columnNames: ColumnNames.optional(),
})

export type TsvFileDialect = z.infer<typeof TsvFileDialect>
