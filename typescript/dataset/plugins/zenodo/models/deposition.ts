import { z } from "zod"
import { ZenodoLink } from "./link.ts"

export const ZenodoDeposition = z
  .object({
    id: z.number().describe("Deposition identifier"),
    links: ZenodoLink,
    state: z.string().optional().describe("State of the deposition, e.g. unsubmitted"),
    submitted: z.boolean().optional().describe("Submitted flag"),
  })
  .describe("Zenodo deposition, as returned by /deposit/depositions")

export type ZenodoDeposition = z.infer<typeof ZenodoDeposition>
