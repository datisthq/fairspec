import { z } from "zod"
import { TitleType } from "./common.ts"

export const Title = z.object({
  title: z.string().describe("A name or title by which a resource is known"),
  titleType: TitleType.optional().describe(
    "The type of title (e.g., AlternativeTitle, Subtitle, TranslatedTitle, Other)",
  ),
  lang: z
    .string()
    .optional()
    .describe("Language of the title, specified using ISO 639-1 or ISO 639-3 codes"),
})

export const Titles = z
  .array(Title)
  .min(1)
  .describe("A name or title by which a resource is known")

export type Title = z.infer<typeof Title>
export type Titles = z.infer<typeof Titles>
