import { z } from "zod"

export const InferDataSchemaInput = z.object({
  data: z.union([z.instanceof(File), z.string()]),
})
