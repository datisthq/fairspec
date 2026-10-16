import { z } from "zod"

export const AlternateIdentifier = z.object({
  alternateIdentifier: z
    .string()
    .describe(
      "An identifier or identifiers other than the primary Identifier applied to the resource being registered",
    ),
  alternateIdentifierType: z
    .string()
    .describe("The type of the AlternateIdentifier (e.g., URL, URN, ISBN, ISSN, etc.)"),
})

export const AlternateIdentifiers = z
  .array(AlternateIdentifier)
  .describe(
    "An identifier or identifiers other than the primary Identifier applied to the resource being registered",
  )

export type AlternateIdentifier = z.infer<typeof AlternateIdentifier>
export type AlternateIdentifiers = z.infer<typeof AlternateIdentifiers>
