import { z } from "zod"

export const SaveDatasetOptions = z.object({
  target: z.string(),
  withRemote: z.boolean().optional(),
})

export type SaveDatasetOptions = z.infer<typeof SaveDatasetOptions>
