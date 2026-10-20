import { loadDataset, saveDataset } from "@fairspec/library"
import { f } from "fireargs"
import pc from "picocolors"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { TextResult } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const copyDatasetCommand = f
  .command({
    description: "Copy a local or remote dataset to a local folder",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.descriptorPath,
      toPath: params.toPathRequired,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(TextResult)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })

    await session.task(pc.bold("Copy dataset"), async () => {
      const dataset = await loadDataset(input.path)
      await saveDataset(dataset, { target: input.toPath })
    })

    return session.renderTextResult(
      `Copied dataset from ${pc.bold(input.path)} to ${pc.bold(input.toPath)}`,
      { status: "success" },
    )
  })
