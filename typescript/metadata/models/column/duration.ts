import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const DurationColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("duration"),
})

export const DurationColumn = BaseColumn.extend({
  type: z.literal("duration"),
  property: DurationColumnProperty,
})

export type DurationColumn = z.infer<typeof DurationColumn>
