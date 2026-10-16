import { z } from "zod"
import { BaseError } from "./base.ts"

export const TextualError = BaseError.extend({
  type: z.literal("file/textual").describe("Error type identifier"),
  actualEncoding: z.string().optional().describe("The actual encoding format found"),
})

export const IntegrityError = BaseError.extend({
  type: z.literal("file/integrity").describe("Error type identifier"),
  hashType: z.string().describe("The type of hash algorithm used"),
  expectedHash: z.string().describe("The expected hash value"),
  actualHash: z.string().describe("The actual hash value found"),
})

export const FileError = z.discriminatedUnion("type", [TextualError, IntegrityError])

export type TextualError = z.infer<typeof TextualError>
export type IntegrityError = z.infer<typeof IntegrityError>
export type FileError = z.infer<typeof FileError>
