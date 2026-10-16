import { z } from "zod"

export const PublicationYear = z
  .string()
  .regex(/^[0-9]{4}$/)
  .describe(
    "The year when the data was or will be made publicly available in YYYY format",
  )

export type PublicationYear = z.infer<typeof PublicationYear>
