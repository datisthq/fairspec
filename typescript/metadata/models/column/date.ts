import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const DateColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("date"),

  temporalFormat: z
    .string()
    .optional()
    .describe(
      "An optional string specifying the datetime format pattern as per the Strftime specification",
    ),
})

export const DateColumn = BaseColumn.extend({
  type: z.literal("date"),
  property: DateColumnProperty,
})

export type DateColumn = z.infer<typeof DateColumn>
