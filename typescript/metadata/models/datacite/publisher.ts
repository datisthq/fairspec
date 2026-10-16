import { z } from "zod"

export const Publisher = z.object({
  name: z
    .string()
    .describe(
      "The name of the entity that holds, archives, publishes, prints, distributes, releases, issues, or produces the resource",
    ),
  publisherIdentifier: z
    .string()
    .optional()
    .describe(
      "Uniquely identifies the publisher, according to various identifier schemes",
    ),
  publisherIdentifierScheme: z
    .string()
    .optional()
    .describe(
      "The name of the publisher identifier scheme (e.g., ISNI, ROR, Crossref Funder ID)",
    ),
  schemeUri: z.string().optional().describe("The URI of the publisher identifier scheme"),
  lang: z
    .string()
    .optional()
    .describe(
      "Language of the publisher name, specified using ISO 639-1 or ISO 639-3 codes",
    ),
})

export type Publisher = z.infer<typeof Publisher>
