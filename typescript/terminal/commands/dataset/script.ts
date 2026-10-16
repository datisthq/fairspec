import repl from "node:repl"
import * as fairspec from "@fairspec/library"
import { loadDataset } from "@fairspec/library"
import { Command } from "commander"
import pc from "picocolors"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const scriptDatasetCommand = new Command()
  .name("script")
  .description("Script a dataset descriptor")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalDescriptorPath)
  .addOption(params.debug)

  .action(async (path, options) => {
    const session = new Session({
      debug: options.debug,
    })

    const dataset = await session.task("Loading dataset", async () => {
      return await loadDataset(path)
    })

    session.renderText(
      pc.dim("`fairspec` and `dataset` variables are available in the session"),
      { status: "warning" },
    )

    const replSession = repl.start({ prompt: "fairspec> " })
    replSession.context.fairspec = fairspec
    replSession.context.dataset = dataset
  })
