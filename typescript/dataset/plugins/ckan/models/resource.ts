import { z } from "zod"
import { CkanSchema } from "./schema.ts"

export const CkanResource = z
  .object({
    id: z.string().describe("Resource identifier"),
    url: z.string().describe("Resource URL"),
    name: z.string().describe("Resource name"),
    created: z.string().describe("Resource creation timestamp"),
    description: z.string().describe("Resource description"),
    format: z.string().describe("Resource format"),
    hash: z.string().describe("Resource hash"),
    last_modified: z.string().describe("Resource last modification timestamp"),
    metadata_modified: z.string().describe("Resource metadata modification timestamp"),
    mimetype: z.string().describe("Resource MIME type"),
    size: z.number().describe("Resource size in bytes"),
    schema: CkanSchema.optional().describe("Resource schema"),
  })
  .describe("CKAN Resource interface")

export const NewCkanResource = CkanResource.partial()
  .required({ url: true, name: true })
  .describe("New CKAN Resource with only url and name required")

export type CkanResource = z.infer<typeof CkanResource>
export type NewCkanResource = z.infer<typeof NewCkanResource>
