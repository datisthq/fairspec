import { z } from "zod"
import { BaseFileDialect } from "./base.ts"

export const UnknownFileDialect = BaseFileDialect.extend({
  format: z.string().optional(),
})

export type UnknownFileDialect = z.infer<typeof UnknownFileDialect>
