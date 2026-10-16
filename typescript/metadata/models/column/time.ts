import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const TimeColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("time"),

  temporalFormat: z
    .string()
    .optional()
    .describe(
      "An optional string specifying the datetime format pattern as per the Strftime specification",
    ),
})

export const TimeColumn = BaseColumn.extend({
  type: z.literal("time"),
  property: TimeColumnProperty,
})

export type TimeColumn = z.infer<typeof TimeColumn>
