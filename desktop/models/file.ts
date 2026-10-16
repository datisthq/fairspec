import { z } from "zod"

export const ValidateFileInput = z.object({
  file: z.union([z.instanceof(File), z.string()]),
  // TODO: Reuse from fairspec
  hashType: z.enum(["md5", "sha1", "sha256", "sha512"]),
  hashValue: z.string(),
})

export const FileType = z.enum(["file", "data", "table", "schema", "dialect", "dataset"])

export type FileType = z.infer<typeof FileType>
