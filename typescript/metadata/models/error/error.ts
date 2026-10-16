import { z } from "zod"
import { DataError } from "./data.ts"
import { FileError } from "./file.ts"
import { MetadataError } from "./metadata.ts"
import { ResourceError } from "./resource.ts"
import { TableError } from "./table.ts"

export const FairspecError = z.union([
  MetadataError,
  ResourceError,
  TableError,
  DataError,
  FileError,
])

export type FairspecError = z.infer<typeof FairspecError>
