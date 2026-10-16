import { z } from "zod"
import { ContentTypeGeneral, RelatedIdentifierType, RelationType } from "./common.ts"

export const RelatedObject = z.object({
  relationType: RelationType.describe(
    "Description of the relationship of the resource being registered and the related resource",
  ),
  relatedMetadataScheme: z
    .string()
    .optional()
    .describe("The name of the scheme (only for HasMetadata/IsMetadataFor relations)"),
  schemeUri: z
    .string()
    .optional()
    .describe(
      "The URI of the relatedMetadataScheme (only for HasMetadata/IsMetadataFor relations)",
    ),
  schemeType: z
    .string()
    .optional()
    .describe(
      "The type of the relatedMetadataScheme (only for HasMetadata/IsMetadataFor relations)",
    ),
  resourceTypeGeneral: ContentTypeGeneral.optional().describe(
    "The general type of the related resource",
  ),
})

export const RelatedIdentifier = RelatedObject.extend({
  relatedIdentifier: z.string().describe("Identifiers of related resources"),
  relatedIdentifierType: RelatedIdentifierType.describe(
    "The type of the RelatedIdentifier (e.g., DOI, Handle, URL, etc.)",
  ),
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

export const RelatedIdentifiers = z
  .array(RelatedIdentifier)
  .describe("Identifiers of related resources")

export type RelatedObject = z.infer<typeof RelatedObject>
export type RelatedIdentifier = z.infer<typeof RelatedIdentifier>
export type RelatedIdentifiers = z.infer<typeof RelatedIdentifiers>
