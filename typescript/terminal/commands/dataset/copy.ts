import { loadDataset, saveDataset } from "@fairspec/library"
import { Command } from "commander"
import pc from "picocolors"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const copyDatasetCommand = new Command()
  .name("copy")
  .description("Copy a local or remote dataset to a local folder")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalDescriptorPath)
  .addOption(params.toPathRequired)
  .addOption(params.silent)
  .addOption(params.debug)
  .addOption(params.json)

  .action(async (path, options) => {
    const session = new Session({
      silent: options.silent,
      debug: options.debug,
      json: options.json,
    })

    await session.task(pc.bold("Copy dataset"), async () => {
      const dataset = await loadDataset(path)
      await saveDataset(dataset, { target: options.toPath })
    })

    session.renderTextResult(
      `Copied dataset from ${pc.bold(path)} to ${pc.bold(options.toPath)}`,
      { status: "success" },
    )
  })
