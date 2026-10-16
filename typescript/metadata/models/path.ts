import { z } from "zod"

export const InternalPath = z
  .string()
  .regex(/^(?![./~])(?!.*:\/\/)(?!.*\.\.)(?!.*\\)(?!.*:)[^/\\]+(?:\/[^/\\]+)*$/)

export const ExternalPath = z.string().regex(/^https?:\/\//)

export const Path = z.union([InternalPath, ExternalPath])

export type InternalPath = z.infer<typeof InternalPath>
export type ExternalPath = z.infer<typeof ExternalPath>
export type Path = z.infer<typeof Path>
