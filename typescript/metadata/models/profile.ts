import { z } from "zod"
import { JsonSchema } from "./jsonSchema.ts"

export const Profile = JsonSchema

export const ProfileType = z.enum([
  "catalog",
  "dataset",
  "file-dialect",
  "data-schema",
  "table-schema",
])

export const ProfileRegistry = z.array(
  z.object({
    type: ProfileType,
    path: z.string(),
    version: z.string(),
    profile: Profile,
  }),
)

export type Profile = z.infer<typeof Profile>
export type ProfileType = z.infer<typeof ProfileType>
export type ProfileRegistry = z.infer<typeof ProfileRegistry>
