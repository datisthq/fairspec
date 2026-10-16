import { inferDataSchema } from "@fairspec/library"
import { Command } from "commander"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const inferDataSchemaCommand = new Command()
  .name("infer")
  .description("Infer a Data Schema")
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

    const dataSchema = await session.task("Infering data schema", async () => {
      const dataSchema = await inferDataSchema({
        data: path,
      })

      if (!dataSchema) {
        throw new Error("Could not infer data schema")
      }

      return dataSchema
    })

    session.renderDataResult(dataSchema)
  })
