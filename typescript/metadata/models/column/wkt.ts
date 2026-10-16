import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const WktColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("wkt"),
})

export const WktColumn = BaseColumn.extend({
  type: z.literal("wkt"),
  property: WktColumnProperty,
})

export type WktColumn = z.infer<typeof WktColumn>
