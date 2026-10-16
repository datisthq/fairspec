import { z } from "zod"
import { BaseColumn } from "./base.ts"
import { BaseObjectColumnProperty } from "./object.ts"

export const GeojsonColumnProperty = BaseObjectColumnProperty.extend({
  format: z.literal("geojson"),
})

export const GeojsonColumn = BaseColumn.extend({
  type: z.literal("geojson"),
  property: GeojsonColumnProperty,
})

export type GeojsonColumn = z.infer<typeof GeojsonColumn>
