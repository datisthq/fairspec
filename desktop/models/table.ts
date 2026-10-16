import { z } from "zod"

export const ValidateTableInput = z.object({
  table: z.union([z.instanceof(File), z.string()]),
  schema: z.union([z.instanceof(File), z.string()]),
  dialect: z.union([z.instanceof(File), z.string()]),
})

export const PreviewTableInput = z.object({
  table: z.union([z.instanceof(File), z.string()]),
  schema: z.union([z.instanceof(File), z.string()]),
  dialect: z.union([z.instanceof(File), z.string()]),
})
