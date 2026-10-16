import { validateDataset } from "@fairspec/library"
import { Command } from "commander"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateDatasetCommand = new Command()
  .name("validate")
  .description("Validate a dataset from a local or remote path")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalDescriptorPath)
  .addOption(params.json)
  .addOption(params.debug)

  .action(async (path, options) => {
    const session = new Session({
      debug: options.debug,
      json: options.json,
    })

    const report = await session.task("Validating dataset", async () => {
      return await validateDataset(path)
    })

    session.renderReportResult(report)
  })
