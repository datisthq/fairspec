import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const HexColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("hex"),
})

export const HexColumn = BaseColumn.extend({
  type: z.literal("hex"),
  property: HexColumnProperty,
})

export type HexColumn = z.infer<typeof HexColumn>
