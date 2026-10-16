import { z } from "zod"

export const Integrity = z.object({
  type: z.enum(["md5", "sha1", "sha256", "sha512"]),
  hash: z.string(),
})

export type Integrity = z.infer<typeof Integrity>
