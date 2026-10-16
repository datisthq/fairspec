import { z } from "zod"

export const ZenodoFile = z
  .object({
    id: z.string().describe("File identifier"),
    key: z.string().describe("File key"),
    size: z.number().describe("File size in bytes"),
    checksum: z.string().describe("File checksum"),
    links: z
      .object({
        self: z.string(),
      })
      .describe("Links related to the file"),
  })
  .describe("Zenodo File interface")

export type ZenodoFile = z.infer<typeof ZenodoFile>
