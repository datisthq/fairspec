import { z } from "zod"
import { ZenodoCreator } from "./creator.ts"

export const ZenodoMetadata = z
  .object({
    title: z.string().describe("Title of the deposit"),
    description: z.string().describe("Description of the deposit"),
    upload_type: z.string().describe('Upload type, e.g., "dataset"'),
    publication_date: z
      .string()
      .optional()
      .describe("Publication date in ISO format (YYYY-MM-DD)"),
    creators: z.array(ZenodoCreator).describe("Creators of the deposit"),
    access_right: z
      .string()
      .optional()
      .describe('Access right, e.g., "open", "embargoed", "restricted", "closed"'),
    license: z.string().optional().describe("License identifier"),
    doi: z.string().optional().describe("DOI of the deposit"),
    keywords: z.array(z.string()).optional().describe("Keywords/tags"),
    related_identifiers: z
      .array(
        z.object({
          identifier: z.string(),
          relation: z.string(),
          scheme: z.string(),
        }),
      )
      .optional()
      .describe("Related identifiers (e.g., DOIs of related works)"),
    communities: z
      .array(
        z.object({
          identifier: z.string(),
        }),
      )
      .optional()
      .describe("Communities the deposit belongs to"),
    version: z.string().optional().describe("Version of the deposit"),
  })
  .describe("Deposit metadata")

export type ZenodoMetadata = z.infer<typeof ZenodoMetadata>
