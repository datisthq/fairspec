import { z } from "zod"

export const InternalPath = z
  .string()
  .regex(
    /^(?:[^\/\\.~A-Za-z][^\/\\]*|[A-Za-z][^\/\\:]*|\.[^\/\\.][^\/\\]*|\.\.[^\/\\]+|\.)(?:\/(?:[^\/\\.][^\/\\]*|\.[^\/\\.][^\/\\]*|\.\.[^\/\\]+|\.))*$/,
  )

export const ExternalPath = z.string().regex(/^[Hh][Tt][Tt][Pp][Ss]?:\/\//)

export const Path = z.union([InternalPath, ExternalPath])

export type InternalPath = z.infer<typeof InternalPath>
export type ExternalPath = z.infer<typeof ExternalPath>
export type Path = z.infer<typeof Path>
