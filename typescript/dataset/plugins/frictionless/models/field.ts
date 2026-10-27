import { z } from "zod"

export const FrictionlessConstraints = z
  .looseObject({
    required: z.boolean().optional(),
    unique: z.boolean().optional(),
    minimum: z.unknown().optional(),
    maximum: z.unknown().optional(),
    exclusiveMinimum: z.unknown().optional(),
    exclusiveMaximum: z.unknown().optional(),
    minLength: z.int().optional(),
    maxLength: z.int().optional(),
    pattern: z.string().optional(),
    enum: z.array(z.unknown()).optional(),
    jsonSchema: z.record(z.string(), z.unknown()).optional(),
  })
  .describe("Frictionless field constraints")

export const FrictionlessMissingValue = z.union([
  z.string(),
  z.looseObject({ value: z.string(), label: z.string() }),
])

export const FrictionlessField = z
  .looseObject({
    name: z.string(),
    type: z.string().optional(),
    format: z.string().optional(),
    title: z.string().optional(),
    description: z.string().optional(),
    example: z.unknown().optional(),
    rdfType: z.string().optional(),
    constraints: FrictionlessConstraints.optional(),
    missingValues: z.array(FrictionlessMissingValue).optional(),
    trueValues: z.array(z.string()).optional(),
    falseValues: z.array(z.string()).optional(),
    decimalChar: z.string().optional(),
    groupChar: z.string().optional(),
    bareNumber: z.boolean().optional(),
    categories: z.array(z.unknown()).optional(),
    categoriesOrdered: z.boolean().optional(),
    delimiter: z.string().optional(),
    itemType: z.string().optional(),
  })
  .describe("Frictionless Table Schema field")

export type FrictionlessConstraints = z.infer<typeof FrictionlessConstraints>
export type FrictionlessMissingValue = z.infer<typeof FrictionlessMissingValue>
export type FrictionlessField = z.infer<typeof FrictionlessField>
