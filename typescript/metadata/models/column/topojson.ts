import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseObjectColumnProperty } from "./object.ts"

export const TopojsonColumnProperty = BaseObjectColumnProperty.extend({
  format: z.literal("topojson"),
})

export const TopojsonColumn = BaseColumn.extend({
  type: z.literal("topojson"),
  property: TopojsonColumnProperty,
})

export type TopojsonColumn = z.infer<typeof TopojsonColumn>
