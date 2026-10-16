import { z } from "zod"
import { BaseFileDialect } from "./base.ts"

export const ArrowFileDialect = BaseFileDialect.extend({
  format: z.literal("arrow"),
})

export type ArrowFileDialect = z.infer<typeof ArrowFileDialect>
