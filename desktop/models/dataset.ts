import { z } from "zod"

export const InferDatasetInput = z.object({
  table: z.union([z.instanceof(File), z.string()]),
})

export const ValidateDatasetInput = z.object({
  dataset: z.union([z.instanceof(File), z.string()]),
})
