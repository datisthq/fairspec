import { z } from "zod"

export const ZenodoLink = z
  .object({
    self: z.string(),
    html: z.string(),
    files: z.string(),
    bucket: z.string(),
    publish: z.string().optional(),
    discard: z.string().optional(),
    edit: z.string().optional(),
  })
  .describe("Deposit URL")

export type ZenodoLink = z.infer<typeof ZenodoLink>
