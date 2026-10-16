import { z } from "zod"
import { ZenodoFile } from "./file.ts"
import { ZenodoLink } from "./link.ts"
import { ZenodoMetadata } from "./metadata.ts"

export const ZenodoRecord = z
  .object({
    id: z.number().describe("Record identifier"),
    links: ZenodoLink,
    metadata: ZenodoMetadata,
    files: z.array(ZenodoFile).describe("Files associated with the record"),
    state: z.enum(["unsubmitted", "inprogress", "done"]).describe("State of the record"),
    submitted: z.boolean().describe("Submitted flag"),
  })
  .describe("Zenodo record interface")

export type ZenodoRecord = z.infer<typeof ZenodoRecord>
