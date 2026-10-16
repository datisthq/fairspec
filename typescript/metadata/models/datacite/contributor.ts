import { z } from "zod"
import { ContributorType } from "./common.ts"
import { Creator } from "./creator.ts"

export const Contributor = Creator.extend({
  contributorType: ContributorType.describe(
    "The type of contributor (e.g., ContactPerson, DataCollector, Editor, etc.)",
  ),
})

export const Contributors = z
  .array(Contributor)
  .describe(
    "The institution or person responsible for collecting, managing, distributing, or otherwise contributing to the development of the resource",
  )

export type Contributor = z.infer<typeof Contributor>
export type Contributors = z.infer<typeof Contributors>
