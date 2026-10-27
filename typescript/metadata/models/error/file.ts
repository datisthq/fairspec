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

export const HeaderError = BaseError.extend({
  type: z.literal("file/header").describe("Error type identifier"),
  partNumber: z
    .number()
    .describe("The 1-based position of the part among the paths of the resource"),
  columnNames: z
    .array(z.string())
    .describe("The column names the part's header gives, empty when it has none"),
})

export const FileError = z.discriminatedUnion("type", [
  TextualError,
  IntegrityError,
  HeaderError,
])

export type TextualError = z.infer<typeof TextualError>
export type IntegrityError = z.infer<typeof IntegrityError>
export type HeaderError = z.infer<typeof HeaderError>
export type FileError = z.infer<typeof FileError>
