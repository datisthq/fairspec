import { z } from "zod"
import { BuiltinFileDialect } from "./builtin.ts"
import { CustomFileDialect } from "./custom.ts"

export const FileDialect = z.union([BuiltinFileDialect, CustomFileDialect])

export type FileDialect = z.infer<typeof FileDialect>
