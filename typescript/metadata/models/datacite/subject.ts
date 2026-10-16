import { z } from "zod"

export const Subject = z.object({
  subject: z
    .string()
    .describe(
      "Subject, keyword, classification code, or key phrase describing the resource",
    ),
  subjectScheme: z
    .string()
    .optional()
    .describe(
      "The name of the subject scheme or classification code or authority if one is used",
    ),
  schemeUri: z.string().optional().describe("The URI of the subject identifier scheme"),
  valueUri: z.string().optional().describe("The URI of the subject term"),
  classificationCode: z
    .string()
    .optional()
    .describe("The classification code used for the subject term in the subject scheme"),
  lang: z
    .string()
    .optional()
    .describe("Language of the subject, specified using ISO 639-1 or ISO 639-3 codes"),
})

export const Subjects = z
  .array(Subject)
  .describe(
    "Subject, keyword, classification code, or key phrase describing the resource",
  )

export type Subject = z.infer<typeof Subject>
export type Subjects = z.infer<typeof Subjects>
