import { z } from "zod"
import { CkanSchema } from "./schema.ts"

export const CkanResource = z
  .object({
    id: z.string().describe("Resource identifier"),
    url: z.string().describe("Resource URL"),
    name: z.string().nullish().describe("Resource name"),
    created: z.string().nullish().describe("Resource creation timestamp"),
    description: z.string().nullish().describe("Resource description"),
    format: z.string().nullish().describe("Resource format"),
    hash: z.string().nullish().describe("Resource hash"),
    last_modified: z.string().nullish().describe("Resource last modification timestamp"),
    metadata_modified: z
      .string()
      .nullish()
      .describe("Resource metadata modification timestamp"),
    mimetype: z.string().nullish().describe("Resource MIME type"),
    size: z.number().nullish().describe("Resource size in bytes"),
    schema: CkanSchema.optional().describe("Resource schema"),
  })
  .describe("CKAN Resource interface")

export const NewCkanResource = CkanResource.partial()
  .required({ url: true, name: true })
  .describe("New CKAN Resource with only url and name required")

export type CkanResource = z.infer<typeof CkanResource>
export type NewCkanResource = z.infer<typeof NewCkanResource>
