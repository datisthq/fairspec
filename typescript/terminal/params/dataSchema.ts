import { z } from "zod"

export const dataSchema = z
  .string()
  .describe("path to a data schema descriptor (JSON Schema)")
