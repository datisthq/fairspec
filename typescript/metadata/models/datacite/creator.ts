import { z } from "zod"
import { CreatorNameType } from "./common.ts"

export const CreatorNameIdentifier = z.object({
  nameIdentifier: z
    .string()
    .describe(
      "Uniquely identifies an individual or legal entity, according to various schemas",
    ),
  nameIdentifierScheme: z
    .string()
    .describe("The name of the name identifier scheme (e.g., ORCID, ISNI, ROR)"),
  schemeUri: z.string().optional().describe("The URI of the name identifier scheme"),
})

export const CreatorNameIdentifiers = z
  .array(CreatorNameIdentifier)
  .describe(
    "Uniquely identifies an individual or legal entity, according to various schemas",
  )

export const CreatorAffiliation = z.object({
  name: z
    .string()
    .describe("The organizational or institutional affiliation of the creator"),
  affiliationIdentifier: z
    .string()
    .optional()
    .describe("Uniquely identifies the organizational affiliation of the creator"),
  affiliationIdentifierScheme: z
    .string()
    .optional()
    .describe("The name of the affiliation identifier scheme"),
  schemeUri: z
    .string()
    .optional()
    .describe("The URI of the affiliation identifier scheme"),
})

export const CreatorAffiliations = z
  .array(CreatorAffiliation)
  .describe("The organizational or institutional affiliation of the creator")

export const Creator = z.object({
  name: z
    .string()
    .describe(
      "The main researchers involved in producing the data, or the authors of the publication in priority order",
    ),
  nameType: CreatorNameType.optional().describe(
    "The type of name (Organizational or Personal)",
  ),
  givenName: z.string().optional().describe("The personal or first name of the creator"),
  familyName: z.string().optional().describe("The surname or last name of the creator"),
  nameIdentifiers: CreatorNameIdentifiers.optional(),
  affiliation: CreatorAffiliations.optional(),
  lang: z
    .string()
    .optional()
    .describe("Language of the name, specified using ISO 639-1 or ISO 639-3 codes"),
})

export const Creators = z
  .array(Creator)
  .min(1)
  .describe(
    "The main researchers involved in producing the data, or the authors of the publication in priority order",
  )

export type CreatorNameIdentifier = z.infer<typeof CreatorNameIdentifier>
export type CreatorNameIdentifiers = z.infer<typeof CreatorNameIdentifiers>
export type CreatorAffiliation = z.infer<typeof CreatorAffiliation>
export type CreatorAffiliations = z.infer<typeof CreatorAffiliations>
export type Creator = z.infer<typeof Creator>
export type Creators = z.infer<typeof Creators>
