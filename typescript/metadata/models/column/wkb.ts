import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const WkbColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("wkb"),
})

export const WkbColumn = BaseColumn.extend({
  type: z.literal("wkb"),
  property: WkbColumnProperty,
})

export type WkbColumn = z.infer<typeof WkbColumn>
