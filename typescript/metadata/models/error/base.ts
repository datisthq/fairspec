import { z } from "zod"

export const BaseError = z.object({
  type: z.string().describe("Error type identifier"),
  resourceName: z.string().optional().describe("Name of the resource if available"),
})

export type BaseError = z.infer<typeof BaseError>
