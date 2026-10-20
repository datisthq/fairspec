import { z } from "zod"

export const debug = z
  .boolean()
  .optional()
  .describe("Enable debug mode to print exception details to stderr")

export const silent = z.boolean().optional().describe("suppress all output except errors")
