import { z } from "zod"

export const Rights = z.object({
  rights: z.string().optional().describe("Any rights information for this resource"),
  rightsUri: z.string().optional().describe("The URI of the license"),
  rightsIdentifier: z
    .string()
    .optional()
    .describe("A short, standardized version of the license name"),
  rightsIdentifierScheme: z
    .string()
    .optional()
    .describe("The name of the scheme (e.g., SPDX)"),
  schemeUri: z.string().optional().describe("The URI of the rightsIdentifierScheme"),
  lang: z
    .string()
    .optional()
    .describe(
      "Language of the rights statement, specified using ISO 639-1 or ISO 639-3 codes",
    ),
})

export const RightsList = z
  .array(Rights)
  .describe("Any rights information for this resource")

export type Rights = z.infer<typeof Rights>
export type RightsList = z.infer<typeof RightsList>
