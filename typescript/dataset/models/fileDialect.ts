import { z } from "zod"

export const InferFileDialectOptions = z.object({
  sampleBytes: z.number().optional(),
})

export type InferFileDialectOptions = z.infer<typeof InferFileDialectOptions>
