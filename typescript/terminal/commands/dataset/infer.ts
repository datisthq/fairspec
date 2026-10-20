import { Dataset, inferDataset } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const inferDatasetCommand = f
  .command({
    description: "Infer a dataset from local or remote file paths",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      paths: params.filePaths,
      debug: params.debug,
      ...params.tableSchemaParams,
    }),
  )
  .output(Dataset)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })

    const dataset = await session.task("Infering dataset", async () => {
      const dataset = { resources: input.paths.map(data => ({ data })) }
      return await inferDataset(dataset, input)
    })

    return session.renderDataResult(dataset)
  })
