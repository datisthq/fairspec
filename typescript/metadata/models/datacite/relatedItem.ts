import { z } from "zod"
import { ContentTypeGeneral, NumberType, RelatedIdentifierType } from "./common.ts"
import { Contributors } from "./contributor.ts"
import { Creators } from "./creator.ts"
import { PublicationYear } from "./publicationYear.ts"
import { RelatedObject } from "./relatedIdentifier.ts"
import { Titles } from "./title.ts"

export const RelatedItemIdentifier = z.object({
  relatedItemIdentifier: z.string().describe("Identifier for the related item"),
  relatedItemIdentifierType: RelatedIdentifierType.describe(
    "The type of the RelatedItemIdentifier",
  ),
})

export const RelatedItem = RelatedObject.extend({
  relatedItemIdentifier: RelatedItemIdentifier.optional().describe(
    "Identifiers of related items",
  ),
  relatedItemType: ContentTypeGeneral.describe("The general type of the related item"),
  creators: Creators.optional(),
  contributors: Contributors.optional(),
  titles: Titles.describe("The title(s) of the related item"),
  publicationYear: PublicationYear.optional(),
  volume: z.string().optional().describe("Volume of the related item"),
  issue: z.string().optional().describe("Issue of the related item"),
  firstPage: z.string().optional().describe("First page of the related item"),
  lastPage: z.string().optional().describe("Last page of the related item"),
  edition: z.string().optional().describe("Edition of the related item"),
  publisher: z.string().optional().describe("Publisher of the related item"),
  number: z
    .string()
    .optional()
    .describe("Number of the related item (e.g., report number, article number)"),
  numberType: NumberType.optional().describe("The type of the number"),
}).refine(
  data => {
    const hasMetadataRelation = ["HasMetadata", "IsMetadataFor"].includes(
      data.relationType,
    )
    if (hasMetadataRelation) return true
    return !data.relatedMetadataScheme && !data.schemeUri && !data.schemeType
  },
  {
    message:
      "relatedMetadataScheme, schemeUri, and schemeType are only allowed for HasMetadata/IsMetadataFor relations",
  },
)

export const RelatedItems = z
  .array(RelatedItem)
  .describe("Information about a resource related to the one being registered")

export type RelatedItemIdentifier = z.infer<typeof RelatedItemIdentifier>
export type RelatedItem = z.infer<typeof RelatedItem>
export type RelatedItems = z.infer<typeof RelatedItems>
