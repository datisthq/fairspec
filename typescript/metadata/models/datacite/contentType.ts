import { z } from "zod"
import { ContentTypeGeneral } from "./common.ts"

export const ContentTypes = z.object({
  resourceType: z
    .string()
    .optional()
    .describe("A description of the resource (free text)"),
  resourceTypeGeneral: ContentTypeGeneral.describe(
    "The general type of the resource (e.g., Dataset, Software, Collection, Image, etc.)",
  ),
})

export type ContentTypes = z.infer<typeof ContentTypes>
