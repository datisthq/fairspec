import { z } from "zod"
import { BaseError } from "./base.ts"

export const ResourceMissingError = BaseError.extend({
  type: z.literal("resource/missing").describe("Error type identifier"),
  referencingResourceName: z.string().describe("The name of the referencing resource"),
})

export const ResourceTypeError = BaseError.extend({
  type: z.literal("resource/type").describe("Error type identifier"),
  expectedResourceType: z.enum(["data", "table"]).describe("The expected data type"),
  referencingResourceName: z
    .string()
    .optional()
    .describe("The name of the referencing resource"),
})

export const ResourceError = z.discriminatedUnion("type", [
  ResourceMissingError,
  ResourceTypeError,
])

export type ResourceMissingError = z.infer<typeof ResourceMissingError>
export type ResourceTypeError = z.infer<typeof ResourceTypeError>
export type ResourceError = z.infer<typeof ResourceError>
