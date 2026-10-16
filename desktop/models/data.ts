import { z } from "zod"

export const ValidateDataInput = z.object({
  data: z.union([z.instanceof(File), z.string()]),
  schema: z.union([z.instanceof(File), z.string()]),
})
