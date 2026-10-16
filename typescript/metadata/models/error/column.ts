import { z } from "zod"
import { ColumnType } from "../column/column.ts"
import { BaseError } from "./base.ts"

export const ColumnMissingError = BaseError.extend({
  type: z.literal("column/missing").describe("Error type identifier"),
  columnName: z.string().describe("Names of missing column"),
})

export const ColumnTypeError = BaseError.extend({
  type: z.literal("column/type").describe("Error type identifier"),
  columnName: z.string().describe("The name of the column"),
  expectedColumnType: ColumnType.describe("The column type that was expected"),
  actualColumnType: ColumnType.describe("The actual column type found"),
})

export const ColumnError = z.discriminatedUnion("type", [
  ColumnMissingError,
  ColumnTypeError,
])

export type ColumnMissingError = z.infer<typeof ColumnMissingError>
export type ColumnTypeError = z.infer<typeof ColumnTypeError>
export type ColumnError = z.infer<typeof ColumnError>
