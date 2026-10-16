import { z } from "zod"

export const BasePropertyType = z.enum([
  "string",
  "number",
  "integer",
  "boolean",
  "array",
  "object",
])

export const BaseColumnProperty = z.object({
  title: z
    .string()
    .optional()
    .describe("An optional human-readable title for the column"),

  description: z
    .string()
    .optional()
    .describe("An optional detailed description of the column"),

  rdfType: z.string().optional().describe("An optional URI for semantic type (RDF)"),

  default: z.unknown().optional(),
})

export const BaseColumn = z.object({
  name: z.string(),
  type: z.string(),
  required: z.boolean().optional(),
  nullable: z.boolean().optional(),
  property: BaseColumnProperty,
})

export type BasePropertyType = z.infer<typeof BasePropertyType>
export type BaseColumnProperty = z.infer<typeof BaseColumnProperty>
export type BaseColumn = z.infer<typeof BaseColumn>
