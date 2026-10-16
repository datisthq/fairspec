import { z } from "zod"

export const Formats = z
  .array(z.string())
  .describe(
    "Technical format of the resource (e.g., file format, physical medium, or dimensions of the resource)",
  )

export type Formats = z.infer<typeof Formats>
