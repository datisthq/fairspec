import { previewFile } from "@fairspec/library"
import { Command } from "commander"
import { selectFile } from "../../helpers/file.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const previewFileCommand = new Command()
  .name("preview")
  .description("Show the raw source of a local or remote file")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalFilePath)
  .addOption(params.fromDataset)
  .addOption(params.fromResource)
  .addOption(params.lines)
  .addOption(params.sampleBytes)
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

    const preview = await session.task("Loading file preview", async () => {
      return await previewFile(path, {
        maxLines: options.lines,
        maxBytes: options.sampleBytes,
      })
    })

    if (options.json) {
      session.renderDataResult(preview)
      return
    }

    session.renderTextResult(preview.text.replace(/\r?\n$/, ""))
  })
