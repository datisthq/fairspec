import { z } from "zod"

export const Sizes = z
  .array(z.string())
  .describe("Unstructured size information about the resource (e.g., '15 pages', '6 MB')")

export type Sizes = z.infer<typeof Sizes>
