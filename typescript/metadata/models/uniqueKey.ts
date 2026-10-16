import { z } from "zod"

export const UniqueKey = z
  .array(z.string())
  .min(1)
  .describe("An array of column names whose combined values must be unique")

export type UniqueKey = z.infer<typeof UniqueKey>
