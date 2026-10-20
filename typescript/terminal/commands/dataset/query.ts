import { queryDataset } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { Records } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const queryDatasetCommand = f
  .command({
    description: "Query a dataset from a local or remote path with SQL",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.descriptorPath,
      query: params.datasetQuery,
      debug: params.debug,
    }),
  )
  .output(Records)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })

    const table = await session.task("Executing query", async () => {
      return await queryDataset(input.path, input.query)
    })

    const frame = await session.task("Collecting data", async () => {
      return await table.collect()
    })

    return session.renderFrameResult(frame)
  })
