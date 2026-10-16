import { z } from "zod"

export const BaseFileDialect = z.object({
  $schema: z.httpUrl().optional().describe("Fairspec Dialect profile url."),

  title: z.string().optional().describe("An optional human-readable title of the format"),

  description: z
    .string()
    .optional()
    .describe("An optional detailed description of the format"),
})

export type BaseFileDialect = z.infer<typeof BaseFileDialect>
