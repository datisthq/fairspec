import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseStringColumnProperty } from "./string.ts"

export const EmailColumnProperty = BaseStringColumnProperty.extend({
  format: z.literal("email"),
})

export const EmailColumn = BaseColumn.extend({
  type: z.literal("email"),
  property: EmailColumnProperty,
})

export type EmailColumn = z.infer<typeof EmailColumn>
