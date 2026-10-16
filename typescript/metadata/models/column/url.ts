import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const UrlColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("url"),
})

export const UrlColumn = BaseColumn.extend({
  type: z.literal("url"),
  property: UrlColumnProperty,
})

export type UrlColumn = z.infer<typeof UrlColumn>
