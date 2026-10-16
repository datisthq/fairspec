import { validateDataSchema } from "@fairspec/library"
import { Command } from "commander"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateDataSchemaCommand = new Command()
  .name("validate")
  .description("Validate a Data Schema")
  .configureHelp(helpConfiguration)

  .addArgument(params.requiredPositionalFilePath)
  .addOption(params.silent)
  .addOption(params.debug)
  .addOption(params.json)

  .action(async (path, options) => {
    const session = new Session({
      silent: options.silent,
      debug: options.debug,
      json: options.json,
    })

    const report = await session.task("Validating data", async () => {
      const { valid, errors } = await validateDataSchema(path)
      return { valid, errors }
    })

    session.renderReportResult(report)
  })
