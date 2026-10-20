import { z } from "zod"
import { ZenodoCreator } from "./creator.ts"
import { ZenodoFile } from "./file.ts"

export const ZenodoRecord = z
  .object({
    id: z.number().describe("Record identifier"),
    metadata: z
      .object({
        title: z.string().describe("Title of the record"),
        description: z.string().optional().describe("Description of the record"),
        creators: z.array(ZenodoCreator).describe("Creators of the record"),
        publication_date: z
          .string()
          .optional()
          .describe("Publication date in ISO format (YYYY-MM-DD)"),
        license: z
          .object({ id: z.string().describe("License identifier, e.g. cc-by-4.0") })
          .optional()
          .describe("License of the record"),
        doi: z.string().optional().describe("DOI of the record"),
        keywords: z.array(z.string()).optional().describe("Keywords/tags"),
        version: z.string().optional().describe("Version of the record"),
      })
      .describe("Record metadata"),
    files: z.array(ZenodoFile).optional().describe("Files associated with the record"),
  })
  .describe("Published Zenodo record, as returned by /records/{id}")

export type ZenodoRecord = z.infer<typeof ZenodoRecord>
