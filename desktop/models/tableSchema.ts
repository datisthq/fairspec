import { z } from "zod"

export const InferTableSchemaInput = z.object({
  table: z.union([z.instanceof(File), z.string()]),
  dialect: z.union([z.instanceof(File), z.string()]),
})
