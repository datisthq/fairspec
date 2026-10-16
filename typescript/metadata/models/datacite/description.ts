import { z } from "zod"
import { DescriptionType } from "./common.ts"

export const Description = z.object({
  description: z
    .string()
    .describe(
      "All additional information that does not fit in any of the other categories",
    ),
  descriptionType: DescriptionType.describe(
    "The type of the Description (e.g., Abstract, Methods, TechnicalInfo, etc.)",
  ),
  lang: z
    .string()
    .optional()
    .describe(
      "Language of the description, specified using ISO 639-1 or ISO 639-3 codes",
    ),
})

export const Descriptions = z
  .array(Description)
  .describe("All additional information that does not fit in any of the other categories")

export type Description = z.infer<typeof Description>
export type Descriptions = z.infer<typeof Descriptions>
