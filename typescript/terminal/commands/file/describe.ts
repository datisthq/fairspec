import { describeFile } from "@fairspec/library"
import { Command } from "commander"
import { selectFile } from "../../helpers/file.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const describeFileCommand = new Command()
  .name("describe")
  .description("Show stats for a local or remote file")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalFilePath)
  .addOption(params.fromDataset)
  .addOption(params.fromResource)
  .addOption(params.hashType)
  .addOption(params.silent)
  .addOption(params.debug)
  .addOption(params.json)

  .action(async (path, options) => {
    const session = new Session({
      silent: options.silent,
      debug: options.debug,
      json: options.json,
    })

    if (!path) {
      path = await selectFile(session, options)
    }

    const stats = await session.task("Describing file", async () => {
      return await describeFile(path, { hashType: options.hashType })
    })

    session.renderDataResult(stats)
  })
