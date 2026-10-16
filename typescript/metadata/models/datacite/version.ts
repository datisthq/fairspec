import { z } from "zod"

export const Version = z
  .string()
  .describe(
    "The version number of the resource. Suggested practice: track major_version.minor_version",
  )

export type Version = z.infer<typeof Version>
