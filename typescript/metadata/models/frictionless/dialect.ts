import { z } from "zod"

export const FrictionlessDialect = z
  .looseObject({
    header: z.boolean().optional(),
    headerRows: z.array(z.int().min(1)).optional(),
    headerJoin: z.string().optional(),
    commentRows: z.array(z.int().min(1)).optional(),
    commentChar: z.string().optional(),
    delimiter: z.string().optional(),
    lineTerminator: z.string().optional(),
    quoteChar: z.string().optional(),
    nullSequence: z.string().optional(),
    property: z.string().optional(),
    itemType: z.enum(["array", "object"]).optional(),
    itemKeys: z.array(z.string()).optional(),
    sheetNumber: z.int().optional(),
    sheetName: z.string().optional(),
    table: z.string().optional(),
  })
  .describe("Frictionless Table Dialect")

export type FrictionlessDialect = z.infer<typeof FrictionlessDialect>
