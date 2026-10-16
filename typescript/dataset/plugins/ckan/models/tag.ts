import { z } from "zod"

export const CkanTag = z
  .object({
    id: z.string().describe("Tag identifier"),
    name: z.string().describe("Tag name"),
    display_name: z.string().describe("Tag display name"),
  })
  .describe("CKAN Tag interface")

export type CkanTag = z.infer<typeof CkanTag>
