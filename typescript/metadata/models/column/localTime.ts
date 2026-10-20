import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const LocalTimeColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("local-time"),

  temporalFormat: z
    .string()
    .optional()
    .describe(
      "An optional string specifying the datetime format pattern as per the Strftime specification",
    ),
})

export const LocalTimeColumn = BaseColumn.extend({
  type: z.literal("local-time"),
  property: LocalTimeColumnProperty,
})

export type LocalTimeColumn = z.infer<typeof LocalTimeColumn>
