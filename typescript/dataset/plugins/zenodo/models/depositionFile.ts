import { z } from "zod"

export const ZenodoDepositionFile = z
  .object({
    id: z.string().describe("File identifier"),
    filename: z.string().optional().describe("File name"),
    filesize: z.number().optional().describe("File size in bytes"),
    checksum: z.string().optional().describe("File MD5 checksum"),
  })
  .describe("File uploaded to a Zenodo deposition")

export type ZenodoDepositionFile = z.infer<typeof ZenodoDepositionFile>
