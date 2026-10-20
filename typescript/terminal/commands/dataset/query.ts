import { queryDataset } from "@fairspec/library"
import { Command } from "commander"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const queryDatasetCommand = new Command()
  .name("query")
  .description("Query a dataset from a local or remote path with SQL")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalDescriptorPath)
  .addArgument(params.datasetQuery)
  .addOption(params.json)
  .addOption(params.debug)

  .action(async (path, query, options) => {
    const session = new Session({
      debug: options.debug,
      json: options.json,
    })

    const table = await session.task("Executing query", async () => {
      return await queryDataset(path, query)
    })

    const frame = await session.task("Collecting data", async () => {
      return await table.collect()
    })

    session.renderFrameResult(frame)
  })
