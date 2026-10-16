import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const DecimalColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("decimal"),

  // TODO: Extract to common (duplication in integer/number/decimal)

  minimum: z
    .number()
    .optional()
    .describe("An optional minimum value constraint (inclusive)"),

  maximum: z
    .number()
    .optional()
    .describe("An optional maximum value constraint (inclusive)"),

  exclusiveMinimum: z
    .number()
    .optional()
    .describe("An optional minimum value constraint (exclusive)"),

  exclusiveMaximum: z
    .number()
    .optional()
    .describe("An optional maximum value constraint (exclusive)"),

  multipleOf: z
    .number()
    .positive()
    .optional()
    .describe("An optional constraint that values must be a multiple of this number"),

  decimalChar: z
    .string()
    .length(1)
    .optional()
    .describe("An optional single character used as the decimal separator in the data"),

  groupChar: z
    .string()
    .length(1)
    .optional()
    .describe("An optional single character used as the thousands separator in the data"),

  withText: z
    .boolean()
    .optional()
    .describe(
      "An optional boolean indicating whether numeric values may include non-numeric text that should be stripped during parsing",
    ),
})

export const DecimalColumn = BaseColumn.extend({
  type: z.literal("decimal"),
  property: DecimalColumnProperty,
})

export type DecimalColumn = z.infer<typeof DecimalColumn>
