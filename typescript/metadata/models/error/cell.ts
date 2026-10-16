import { z } from "zod"
import { ColumnType } from "../column/column.ts"
import { BaseError } from "./base.ts"

const BaseCellError = BaseError.extend({
  columnName: z.string().describe("The name of the column"),
  rowNumber: z.number().describe("The row number where the error occurred"),
  cell: z.string().describe("The cell value that caused the error"),
})

export const CellTypeError = BaseCellError.extend({
  type: z.literal("cell/type").describe("Error type identifier"),
  columnType: ColumnType.describe("The expected column type"),
})

export const CellMissingError = BaseCellError.extend({
  type: z.literal("cell/missing").describe("Error type identifier"),
})

export const CellMinimumError = BaseCellError.extend({
  type: z.literal("cell/minimum").describe("Error type identifier"),
  minimum: z.string().describe("The minimum value allowed"),
})

export const CellMaximumError = BaseCellError.extend({
  type: z.literal("cell/maximum").describe("Error type identifier"),
  maximum: z.string().describe("The maximum value allowed"),
})

export const CellExclusiveMinimumError = BaseCellError.extend({
  type: z.literal("cell/exclusiveMinimum").describe("Error type identifier"),
  minimum: z.string().describe("The exclusive minimum value"),
})

export const CellMultipleOfError = BaseCellError.extend({
  type: z.literal("cell/multipleOf").describe("Error type identifier"),
  multipleOf: z.number().describe("The multiple of constraint"),
})

export const CellExclusiveMaximumError = BaseCellError.extend({
  type: z.literal("cell/exclusiveMaximum").describe("Error type identifier"),
  maximum: z.string().describe("The exclusive maximum value"),
})

export const CellMinLengthError = BaseCellError.extend({
  type: z.literal("cell/minLength").describe("Error type identifier"),
  minLength: z.number().describe("The minimum length required"),
})

export const CellMaxLengthError = BaseCellError.extend({
  type: z.literal("cell/maxLength").describe("Error type identifier"),
  maxLength: z.number().describe("The maximum length allowed"),
})

export const CellPatternError = BaseCellError.extend({
  type: z.literal("cell/pattern").describe("Error type identifier"),
  pattern: z.string().describe("The pattern that must be matched"),
})

export const CellUniqueError = BaseCellError.extend({
  type: z.literal("cell/unique").describe("Error type identifier"),
})

export const CellConstError = BaseCellError.extend({
  type: z.literal("cell/const").describe("Error type identifier"),
  const: z.string().describe("The allowed value"),
})

export const CellEnumError = BaseCellError.extend({
  type: z.literal("cell/enum").describe("Error type identifier"),
  enum: z.array(z.string()).describe("The allowed enumeration values"),
})

export const CellJsonError = BaseCellError.extend({
  type: z.literal("cell/json").describe("Error type identifier"),
  message: z.string().describe("The JSON schema validation error message"),
  jsonPointer: z.string().describe("JSON Pointer to the validation error location"),
})

export const CellMinItemsError = BaseCellError.extend({
  type: z.literal("cell/minItems").describe("Error type identifier"),
  minItems: z.number().describe("The minimum number of items required"),
})

export const CellMaxItemsError = BaseCellError.extend({
  type: z.literal("cell/maxItems").describe("Error type identifier"),
  maxItems: z.number().describe("The maximum number of items allowed"),
})

export const CellError = z.discriminatedUnion("type", [
  CellTypeError,
  CellMissingError,
  CellMinimumError,
  CellMaximumError,
  CellExclusiveMinimumError,
  CellExclusiveMaximumError,
  CellMultipleOfError,
  CellMinLengthError,
  CellMaxLengthError,
  CellMinItemsError,
  CellMaxItemsError,
  CellPatternError,
  CellUniqueError,
  CellConstError,
  CellEnumError,
  CellJsonError,
])

export type CellTypeError = z.infer<typeof CellTypeError>
export type CellMissingError = z.infer<typeof CellMissingError>
export type CellMinimumError = z.infer<typeof CellMinimumError>
export type CellMaximumError = z.infer<typeof CellMaximumError>
export type CellExclusiveMinimumError = z.infer<typeof CellExclusiveMinimumError>
export type CellExclusiveMaximumError = z.infer<typeof CellExclusiveMaximumError>
export type CellMultipleOfError = z.infer<typeof CellMultipleOfError>
export type CellMinLengthError = z.infer<typeof CellMinLengthError>
export type CellMaxLengthError = z.infer<typeof CellMaxLengthError>
export type CellMinItemsError = z.infer<typeof CellMinItemsError>
export type CellMaxItemsError = z.infer<typeof CellMaxItemsError>
export type CellPatternError = z.infer<typeof CellPatternError>
export type CellUniqueError = z.infer<typeof CellUniqueError>
export type CellConstError = z.infer<typeof CellConstError>
export type CellEnumError = z.infer<typeof CellEnumError>
export type CellJsonError = z.infer<typeof CellJsonError>
export type CellError = z.infer<typeof CellError>
