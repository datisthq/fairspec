import { writeFile } from "node:fs/promises"
import { renderTableSchemaAs, resolveTableSchema } from "@fairspec/library"
import { f } from "fireargs"
import pc from "picocolors"
import { z } from "zod"
import { helpConfiguration } from "../../helpers/help.ts"
import { TextResult } from "../../models/output.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const renderTableSchemaCommand = f
  .command({
    description: "Render a Table Schema as HTML or Markdown",
    configureHelp: helpConfiguration,
  })
  .input(
    z.object({
      path: params.requiredFilePath,
      toFormat: z.enum(["markdown", "html"]).describe("target schema format"),
      toPath: params.toPath,
      silent: params.silent,
      debug: params.debug,
    }),
  )
  .output(TextResult)
  .handler(async (input, ctx) => {
    const session = new Session({ ...input, json: ctx.json })
    const { toPath } = input

    const tableSchema = await session.task("Loading table schema", async () => {
      return await resolveTableSchema(input.path)
    })

    if (!tableSchema) {
      throw new Error("Could not load table schema")
    }

    const rendered = await session.task("Rendering table schema", async () => {
      return await renderTableSchemaAs(tableSchema, { format: input.toFormat })
    })

    if (!toPath) {
      return session.renderTextResult(rendered)
    }

    await session.task("Saving rendered schema", async () => {
      await writeFile(toPath, rendered, "utf-8")
    })

    return session.renderTextResult(`Saved rendered schema to ${pc.bold(toPath)}`, {
      status: "success",
    })
  })
