import { z } from "zod"

export const CatalogDataset = z.object({
  loc: z.url().describe("The location (URI) of the dataset"),
  upd: z.iso
    .datetime({ offset: true })
    .describe("The last updated date-time of the dataset"),
})

export const Catalog = z
  .array(CatalogDataset)
  .describe(
    "A catalog is an array of dataset references with their locations and update times",
  )

export type CatalogDataset = z.infer<typeof CatalogDataset>
export type Catalog = z.infer<typeof Catalog>
