import { z } from "zod"

export const ForeignKey = z.object({
  columns: z.array(z.string()).describe("Source column(s) in this table"),

  reference: z
    .object({
      resource: z
        .string()
        .optional()
        .describe("Target resource name (optional, omit for self-reference)"),

      columns: z
        .array(z.string())
        .describe("Target column(s) in the referenced resource"),
    })
    .describe("Reference to columns in another resource"),
})

export type ForeignKey = z.infer<typeof ForeignKey>
