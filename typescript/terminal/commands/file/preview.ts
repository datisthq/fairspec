import { previewFile } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"
import { selectFile } from "../../helpers/file.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const previewFileCommand = f
  .command({
    description: "Show the raw source of a local or remote file",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.filePath,
      dataset: params.dataset,
      resource: params.resource,
      lines: params.lines,
      sampleBytes: params.sampleBytes,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(z.object({ text: z.string(), truncated: z.boolean() }))
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })
    const path = input.path ?? (await selectFile(session, input))

    const preview = await session.task("Loading file preview", async () => {
      return await previewFile(path, {
        maxLines: input.lines,
        maxBytes: input.sampleBytes,
      })
    })

    session.renderText(preview.text.replace(/\r?\n$/, ""))
    return preview
  })
