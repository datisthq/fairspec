import repl from "node:repl"
import * as fairspec from "@fairspec/library"
import { loadDataset } from "@fairspec/library"
import { f } from "fireargs"
import pc from "picocolors"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { Empty } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const scriptDatasetCommand = f
  .command({
    description: "Script a dataset descriptor",
    configureHelp: helpConfiguration,
    jsonOption: false,
  })
  .input(z.object({ path: params.descriptorPath, debug: params.debug }))
  .output(Empty)
  .handler(async input => {
    const session = new Session(input)

    const dataset = await session.task("Loading dataset", async () => {
      return await loadDataset(input.path)
    })

    session.renderText(
      pc.dim("`fairspec` and `dataset` variables are available in the session"),
      { status: "warning" },
    )

    const replSession = repl.start({ prompt: "fairspec> " })
    replSession.context.fairspec = fairspec
    replSession.context.dataset = dataset

    return {}
  })
