import { z } from "zod"

export const CkanOrganization = z
  .object({
    id: z.string().describe("Organization identifier"),
    name: z.string().describe("Organization name"),
    title: z.string().describe("Organization title"),
    description: z.string().describe("Organization description"),
  })
  .describe("CKAN Organization interface")

export type CkanOrganization = z.infer<typeof CkanOrganization>
