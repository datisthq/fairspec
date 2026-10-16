import { z } from "zod"
import { FunderIdentifierType } from "./common.ts"

export const FundingReference = z.object({
  funderName: z.string().describe("Name of the funding provider"),
  funderIdentifier: z
    .string()
    .optional()
    .describe(
      "Uniquely identifies a funding entity, according to various identifier schemes",
    ),
  funderIdentifierType: FunderIdentifierType.optional().describe(
    "The type of the funderIdentifier (e.g., ISNI, GRID, Crossref Funder ID, ROR, Other)",
  ),
  awardNumber: z
    .string()
    .optional()
    .describe("The code assigned by the funder to a sponsored award (grant)"),
  awardUri: z
    .string()
    .optional()
    .describe(
      "The URI leading to a page provided by the funder for more information about the award (grant)",
    ),
  awardTitle: z
    .string()
    .optional()
    .describe("The human readable title of the award (grant)"),
})

export const FundingReferences = z
  .array(FundingReference)
  .describe(
    "Information about financial support (funding) for the resource being registered",
  )

export type FundingReference = z.infer<typeof FundingReference>
export type FundingReferences = z.infer<typeof FundingReferences>
