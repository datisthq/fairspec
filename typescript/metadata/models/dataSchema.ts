import { z } from "zod"
import { JsonSchema } from "./jsonSchema.ts"

export const DataSchema = JsonSchema

export const RenderDataSchemaOptions = z.looseObject({
  format: z.string(),
})

export type DataSchema = z.infer<typeof DataSchema>
export type RenderDataSchemaOptions = z.infer<typeof RenderDataSchemaOptions>
