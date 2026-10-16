import { z } from "zod"

export const InferDialectInput = z.object({
  file: z.union([z.instanceof(File), z.string()]),
})
