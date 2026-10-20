import { f } from "fireargs"
import { z } from "zod"

export const descriptorPath = f
  .argument()
  .string()
  .describe("local or remote path to the descriptor")

export const tablePath = f
  .argument()
  .schema(z.string().optional())
  .describe("local or remote path to the table")

export const filePath = f
  .argument()
  .schema(z.string().optional())
  .describe("local or remote path to the file")

export const requiredFilePath = f
  .argument()
  .string()
  .describe("local or remote path to the file")

export const filePaths = f
  .argument({ name: "path" })
  .array(z.string())
  .describe("local paths to files")

export const toPath = z.string().optional().describe("a local output path")

export const toPathRequired = z.string().describe("a local output path")
