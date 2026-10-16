import { writeFile } from "node:fs/promises"
import { renderTableSchemaAs, resolveTableSchema } from "@fairspec/library"
import { Command, Option } from "commander"
import pc from "picocolors"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

const toFormat = new Option("--to-format <format>", "target schema format")
  .choices(["markdown", "html"])
  .makeOptionMandatory()

export const renderTableSchemaCommand = new Command()
  .name("render")
  .description("Render a Table Schema as HTML or Markdown")
  .configureHelp(helpConfiguration)

  .addArgument(params.requiredPositionalFilePath)
  .addOption(toFormat)
  .addOption(params.toPath)
  .addOption(params.silent)
  .addOption(params.debug)
  .addOption(params.json)

  .action(async (path, options) => {
    const session = new Session({
      silent: options.silent,
      debug: options.debug,
      json: options.json,
    })

    if (!options.toFormat) {
      throw new Error("--to-format must be specified")
    }

    const tableSchema = await session.task("Loading table schema", async () => {
      return await resolveTableSchema(path)
    })

    if (!tableSchema) {
      throw new Error("Could not load table schema")
    }

    const rendered = await session.task("Rendering table schema", async () => {
      return await renderTableSchemaAs(tableSchema, {
        format: options.toFormat,
      })
    })

    const isSaved = await session.task("Saving rendered schema", async () => {
      if (!options.toPath) return false
      await writeFile(options.toPath, rendered, "utf-8")
      return true
    })

    if (!isSaved) {
      session.renderTextResult(rendered)
      return
    }

    session.renderTextResult(`Saved rendered schema to ${pc.bold(options.toPath)}`, {
      status: "success",
    })
  })
