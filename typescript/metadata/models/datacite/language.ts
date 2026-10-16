import { z } from "zod"

export const Language = z
  .string()
  .describe(
    "The primary language of the resource. Allowed values are taken from IETF BCP 47, ISO 639-1 language code",
  )

export type Language = z.infer<typeof Language>
