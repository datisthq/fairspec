import { z } from "zod"
import { BaseError } from "./base.ts"

export const RowPrimaryKeyError = BaseError.extend({
  type: z.literal("row/primaryKey").describe("Error type identifier"),
  rowNumber: z.number().describe("The row number where the error occurred"),

  columnNames: z
    .array(z.string())
    .describe("Column names involved in the primary key constraint violation"),
})

export const RowUniqueKeyError = BaseError.extend({
  type: z.literal("row/uniqueKey").describe("Error type identifier"),
  rowNumber: z.number().describe("The row number where the error occurred"),

  columnNames: z
    .array(z.string())
    .describe("Column names involved in the unique key constraint violation"),
})

export const RowError = z.discriminatedUnion("type", [
  RowPrimaryKeyError,
  RowUniqueKeyError,
])

export type RowPrimaryKeyError = z.infer<typeof RowPrimaryKeyError>
export type RowUniqueKeyError = z.infer<typeof RowUniqueKeyError>
export type RowError = z.infer<typeof RowError>
