import { z } from "zod"
import { FrictionlessField, FrictionlessMissingValue } from "./field.ts"

const FieldNames = z.union([z.string(), z.array(z.string())])

export const FrictionlessForeignKey = z.looseObject({
  fields: FieldNames,
  reference: z.looseObject({
    resource: z.string().optional(),
    fields: FieldNames,
  }),
})

export const FrictionlessSchema = z
  .looseObject({
    fields: z.array(FrictionlessField),
    primaryKey: FieldNames.optional(),
    uniqueKeys: z.array(z.array(z.string())).optional(),
    foreignKeys: z.array(FrictionlessForeignKey).optional(),
    missingValues: z.array(FrictionlessMissingValue).optional(),
  })
  .describe("Frictionless Table Schema")

export type FrictionlessForeignKey = z.infer<typeof FrictionlessForeignKey>
export type FrictionlessSchema = z.infer<typeof FrictionlessSchema>
