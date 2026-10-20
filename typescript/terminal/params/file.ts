import { z } from "zod"

export const hashType = z
  .enum(["md5", "sha1", "sha256", "sha512"])
  .default("sha256")
  .describe("hash type")

export const sampleBytes = z.number().int().optional().describe("sample size in bytes")

export const hash = z
  .string()
  .optional()
  .describe("expected file hash calculated with the specified hash type")

export const lines = z.number().int().optional().describe("maximum number of lines")
