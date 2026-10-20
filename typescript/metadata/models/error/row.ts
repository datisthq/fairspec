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

export const RowCheckError = BaseError.extend({
  type: z.literal("row/check").describe("Error type identifier"),
  rowNumber: z.number().describe("The row number where the error occurred"),
  checkName: z.string().describe("The name of the check that failed"),
  message: z.string().optional().describe("The message of the check that failed"),
})

export const RowError = z.discriminatedUnion("type", [
  RowPrimaryKeyError,
  RowUniqueKeyError,
  RowCheckError,
])

export type RowPrimaryKeyError = z.infer<typeof RowPrimaryKeyError>
export type RowUniqueKeyError = z.infer<typeof RowUniqueKeyError>
export type RowCheckError = z.infer<typeof RowCheckError>
export type RowError = z.infer<typeof RowError>
