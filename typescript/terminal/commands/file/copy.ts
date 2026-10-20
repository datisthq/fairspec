import { copyFile } from "@fairspec/library"
import { f } from "fireargs"
import pc from "picocolors"
import { z } from "zod"
import { selectFile } from "../../helpers/file.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import { TextResult } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const copyFileCommand = f
  .command({
    description: "Copy a local or remote file to a local path",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.filePath,
      toPath: params.toPathRequired,
      dataset: params.dataset,
      resource: params.resource,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(TextResult)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })
    const path = input.path ?? (await selectFile(session, input))

    await session.task("Copying file", async () => {
      await copyFile({ sourcePath: path, targetPath: input.toPath })
    })

    return session.renderTextResult(
      `Copied file from ${pc.bold(path)} to ${pc.bold(input.toPath)}`,
      { status: "success" },
    )
  })
