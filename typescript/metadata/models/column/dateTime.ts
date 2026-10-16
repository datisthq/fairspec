import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const DateTimeColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("date-time"),

  temporalFormat: z
    .string()
    .optional()
    .describe(
      "An optional string specifying the datetime format pattern as per the Strftime specification",
    ),
})

export const DateTimeColumn = BaseColumn.extend({
  type: z.literal("date-time"),
  property: DateTimeColumnProperty,
})

export type DateTimeColumn = z.infer<typeof DateTimeColumn>
