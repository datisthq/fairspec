import { Dataset, inferDataset } from "@fairspec/library"
import { z } from "zod"
import { createTool } from "../../helpers/tool.ts"

export const inferDatasetTool = createTool({
  name: "inferDatasetTool",
  description:
    "Infer metadata for all resources in a dataset. Analyzes each resource to determine schemas and dialects.",
  inputSchema: z.object({
    dataset: Dataset.describe("The dataset to infer metadata for"),
  }),
  outputSchema: z.object({
    dataset: Dataset,
  }),
  execute: async input => {
    const dataset = await inferDataset(input.dataset)
    return { dataset }
  },
})
