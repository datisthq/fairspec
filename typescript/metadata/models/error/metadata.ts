import { z } from "zod"
import { BaseError } from "./base.ts"

export const MetadataError = BaseError.extend({
  type: z.literal("metadata").describe("Error type identifier"),
  message: z.string().describe("The JSON parsing error message"),
  jsonPointer: z.string().describe("JSON Pointer to the location of the error"),
})

export type MetadataError = z.infer<typeof MetadataError>
