import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const LocalDateTimeColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("local-date-time"),

  temporalFormat: z
    .string()
    .optional()
    .describe(
      "An optional string specifying the datetime format pattern as per the Strftime specification",
    ),
})

export const LocalDateTimeColumn = BaseColumn.extend({
  type: z.literal("local-date-time"),
  property: LocalDateTimeColumnProperty,
})

export type LocalDateTimeColumn = z.infer<typeof LocalDateTimeColumn>
