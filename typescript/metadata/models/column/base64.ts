import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const Base64ColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("base64"),
})

export const Base64Column = BaseColumn.extend({
  type: z.literal("base64"),
  property: Base64ColumnProperty,
})

export type Base64Column = z.infer<typeof Base64Column>
