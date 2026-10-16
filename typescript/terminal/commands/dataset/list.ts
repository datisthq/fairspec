import { inferResourceName, loadDataset } from "@fairspec/library"
import { Command } from "commander"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const listDatasetCommand = new Command()
  .name("list")
  .description("List Dataset resources")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalDescriptorPath)
  .addOption(params.debug)
  .addOption(params.json)

  .action(async (path, options) => {
    const session = new Session({
      debug: options.debug,
      json: options.json,
    })

    const dataset = await session.task("Loading dataset", async () => {
      const dataset = await loadDataset(path)

      if (!dataset) {
        throw new Error("Could not load dataset")
      }

      return dataset
    })

    const resourceNames = (dataset.resources ?? []).map(
      resource => resource.name ?? inferResourceName(resource),
    )

    session.renderDataResult(resourceNames)
  })
