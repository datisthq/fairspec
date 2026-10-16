import { z } from "zod"

export const CkanFieldInfo = z
  .object({
    label: z.string().describe("Human-readable field label"),
    notes: z.string().describe("Additional notes about the field"),
    type_override: z.string().describe("Field type override"),
  })
  .describe("CKAN Field Info interface")

export const CkanField = z
  .object({
    id: z.string().describe("Field identifier"),
    type: z.string().describe("Field data type"),
    info: CkanFieldInfo.optional().describe("Additional field information"),
  })
  .describe("CKAN Field interface")

export type CkanFieldInfo = z.infer<typeof CkanFieldInfo>
export type CkanField = z.infer<typeof CkanField>
