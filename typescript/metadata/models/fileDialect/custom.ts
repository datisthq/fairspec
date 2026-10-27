import { z } from "zod"
import { BaseFileDialect } from "./base.ts"
import { BUILTIN_FILE_DIALECT_FORMATS } from "./builtin.ts"

export const CustomFileDialect = BaseFileDialect.extend({
  format: z.string().refine(format => !BUILTIN_FILE_DIALECT_FORMATS.includes(format), {
    message: "Expected a format that is not built in",
  }),
})

export type CustomFileDialect = z.infer<typeof CustomFileDialect>
