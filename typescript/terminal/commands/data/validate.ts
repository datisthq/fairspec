import { loadDataSchema, validateData } from "@fairspec/library"
import { Command } from "commander"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateDataCommand = new Command()
  .name("validate")
  .description("Validate a JSON Data from a local or remote path")
  .configureHelp(helpConfiguration)

  .addArgument(params.requiredPositionalFilePath)
  .addOption(params.dataSchema)
  .addOption(params.silent)
  .addOption(params.debug)
  .addOption(params.json)

  .action(async (path, options) => {
    const session = new Session({
      silent: options.silent,
      debug: options.debug,
      json: options.json,
    })

    const dataSchema = await session.task("Loading data schema", async () => {
      if (!options.schema) {
        throw new Error("No data schema provided")
      }

      return await loadDataSchema(options.schema)
    })

    const report = await session.task("Validating data", async () => {
      return await validateData({
        data: path,
        dataSchema,
      })
    })

    session.renderReportResult(report)
  })
