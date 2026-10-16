import { copyFile } from "@fairspec/library"
import { Command } from "commander"
import pc from "picocolors"
import { selectFile } from "../../helpers/file.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const copyFileCommand = new Command()
  .name("copy")
  .description("Copy a local or remote file to a local path")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalFilePath)
  .addOption(params.toPathRequired)
  .addOption(params.fromDataset)
  .addOption(params.fromResource)
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

    await session.task("Copying file", async () => {
      await copyFile({
        sourcePath: path,
        targetPath: options.toPath,
      })
    })

    session.renderTextResult(
      `Copied file from ${pc.bold(path)} to ${pc.bold(options.toPath)}`,
      { status: "success" },
    )
  })
