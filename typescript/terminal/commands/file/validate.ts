import { validateFile } from "@fairspec/library"
import { Command } from "commander"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const validateFileCommand = new Command()
  .name("validate")
  .description("Validate a file from a local or remote path")
  .configureHelp(helpConfiguration)

  .addArgument(params.requiredPositionalFilePath)
  .addOption(params.hash)
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

    const report = await session.task("Validating file", async () => {
      return await validateFile({
        data: path,
        integrity: options.hash
          ? { hash: options.hash, type: options.hashType ?? "md5" }
          : undefined,
      })
    })

    session.renderReportResult(report)
  })
