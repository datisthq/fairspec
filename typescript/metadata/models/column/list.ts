import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const ListColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("list"),

  itemType: z
    .enum(["string", "integer", "number", "boolean", "date-time", "date", "time"])
    .optional()
    .describe("An optional type for items in a list column"),

  delimiter: z
    .string()
    .length(1)
    .optional()
    .describe("An optional single character used to delimit items in a list column"),

  minItems: z
    .number()
    .int()
    .min(0)
    .optional()
    .describe("An optional minimum length constraint for list values"),

  maxItems: z
    .number()
    .int()
    .min(0)
    .optional()
    .describe("An optional maximum length constraint for list values"),
})

export const ListColumn = BaseColumn.extend({
  type: z.literal("list"),
  property: ListColumnProperty,
})

export type ListColumn = z.infer<typeof ListColumn>
