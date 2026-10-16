import { z } from "zod"

export const ZenodoCreator = z
  .object({
    name: z.string().describe("Creator name (format: Family name, Given names)"),
    affiliation: z.string().optional().describe("Creator affiliation"),
    identifiers: z
      .array(
        z.object({
          identifier: z.string(),
          scheme: z.string(),
        }),
      )
      .optional()
      .describe("Creator identifiers (e.g., ORCID)"),
  })
  .describe("Zenodo Creator interface")

export type ZenodoCreator = z.infer<typeof ZenodoCreator>
