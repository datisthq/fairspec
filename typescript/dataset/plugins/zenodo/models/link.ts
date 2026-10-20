import { z } from "zod"

export const ZenodoLink = z
  .object({
    self: z.string().optional(),
    html: z.string(),
    files: z.string().optional(),
    bucket: z.string().optional(),
    publish: z.string().optional(),
    discard: z.string().optional(),
    edit: z.string().optional(),
  })
  .describe("Deposition URLs")

export type ZenodoLink = z.infer<typeof ZenodoLink>
