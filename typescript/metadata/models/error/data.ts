import { z } from "zod"
import { BaseError } from "./base.ts"

export const DataError = BaseError.extend({
  type: z.literal("data").describe("Error type identifier"),
  message: z.string().describe("The JSON parsing error message"),
  jsonPointer: z.string().describe("JSON Pointer to the location of the error"),
})

export type DataError = z.infer<typeof DataError>
