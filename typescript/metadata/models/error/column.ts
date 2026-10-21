import { z } from "zod"
import { ColumnType } from "../column/column.ts"
import { BaseError } from "./base.ts"

export const ColumnMissingError = BaseError.extend({
  type: z.literal("column/missing").describe("Error type identifier"),
  columnName: z.string().describe("Names of missing column"),
})

export const ColumnExtraError = BaseError.extend({
  type: z.literal("column/extra").describe("Error type identifier"),
  columnName: z.string().describe("Name of the column the schema does not list"),
})

export const ColumnTypeError = BaseError.extend({
  type: z.literal("column/type").describe("Error type identifier"),
  columnName: z.string().describe("The name of the column"),
  expectedColumnType: ColumnType.describe("The column type that was expected"),
  actualColumnType: ColumnType.describe("The actual column type found"),
})

export const ColumnError = z.discriminatedUnion("type", [
  ColumnMissingError,
  ColumnExtraError,
  ColumnTypeError,
])

export type ColumnMissingError = z.infer<typeof ColumnMissingError>
export type ColumnExtraError = z.infer<typeof ColumnExtraError>
export type ColumnTypeError = z.infer<typeof ColumnTypeError>
export type ColumnError = z.infer<typeof ColumnError>
