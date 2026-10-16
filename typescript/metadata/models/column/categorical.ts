import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseIntegerColumnProperty } from "./integer.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const IntegerCategoricalColumnProperty = BaseIntegerColumnProperty.extend({
  format: z.literal("categorical"),

  categories: z
    .array(
      z.union([
        z.int(),
        z.object({
          value: z.int(),
          label: z.string(),
        }),
      ]),
    )
    .optional()
    .describe("An optional array of categorical values with optional labels"),

  withOrder: z
    .boolean()
    .optional()
    .describe("An optional boolean indicating whether the categories are ordered"),
})

export const StringCategoricalColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("categorical"),

  categories: z
    .array(
      z.union([
        z.string(),
        z.object({
          value: z.string(),
          label: z.string(),
        }),
      ]),
    )
    .optional()
    .describe("An optional array of categorical values with optional labels"),

  withOrder: z
    .boolean()
    .optional()
    .describe("An optional boolean indicating whether the categories are ordered"),
})

export const CategoricalColumn = BaseColumn.extend({
  type: z.literal("categorical"),
  property: z.union([StringCategoricalColumnProperty, IntegerCategoricalColumnProperty]),
})

export type CategoricalColumn = z.infer<typeof CategoricalColumn>
