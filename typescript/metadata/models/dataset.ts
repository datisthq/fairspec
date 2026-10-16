import { z } from "zod"
import { Datacite } from "./datacite/datacite.ts"
import { Resource } from "./resource.ts"

export const Dataset = Datacite.extend({
  $schema: z.httpUrl().optional().describe("Fairspec Dataset profile url."),

  resources: z
    .array(Resource)
    .optional()
    .describe(
      "A list of resources. Each item must be a Resource object describing data files or inline data.",
    ),
})

export const RenderDatasetOptions = z.object({
  format: z.string(),
})

export const ConvertDatasetToOptions = z.object({
  format: z.string(),
})

export const ConvertDatasetFromOptions = z.object({
  format: z.string(),
})

export type Dataset = z.infer<typeof Dataset>
export type RenderDatasetOptions = z.infer<typeof RenderDatasetOptions>
export type ConvertDatasetToOptions = z.infer<typeof ConvertDatasetToOptions>
export type ConvertDatasetFromOptions = z.infer<typeof ConvertDatasetFromOptions>
