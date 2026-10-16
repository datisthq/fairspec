import type { Resource } from "@fairspec/library"
import { loadTable } from "@fairspec/library"
import { Command } from "commander"
import { createFileDialectFromPathAndOptions } from "../../helpers/fileDialect.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import { selectResource } from "../../helpers/resource.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const previewTableCommand = new Command()
  .name("preview")
  .description("Preview a table from a local or remote path")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalTablePath)
  .addOption(params.fromDataset)
  .addOption(params.fromResource)
  .addOption(params.debug)
  .addOption(params.json)

  .optionsGroup("Dialect")
  .addOption(params.dialect)
  .addOption(params.format)
  .addOption(params.delimiter)
  .addOption(params.lineTerminator)
  .addOption(params.quoteChar)
  .addOption(params.nullSequence)
  .addOption(params.headerRows)
  .addOption(params.headerJoin)
  .addOption(params.commentRows)
  .addOption(params.commentPrefix)
  .addOption(params.columnNames)
  .addOption(params.jsonPointer)
  .addOption(params.rowType)
  .addOption(params.sheetNumber)
  .addOption(params.sheetName)
  .addOption(params.tableName)

  .optionsGroup("Table Schema")
  .addOption(params.tableSchema)
  .addOption(params.columnTypes)
  .addOption(params.missingValues)
  .addOption(params.decimalChar)
  .addOption(params.groupChar)
  .addOption(params.trueValues)
  .addOption(params.falseValues)
  .addOption(params.datetimeFormat)
  .addOption(params.dateFormat)
  .addOption(params.timeFormat)
  .addOption(params.arrayType)
  .addOption(params.listDelimiter)
  .addOption(params.listItemType)
  .addOption(params.sampleRows)
  .addOption(params.confidence)
  .addOption(params.commaDecimal)
  .addOption(params.monthFirst)
  .addOption(params.keepStrings)

  .action(async (path, options) => {
    const session = new Session({
      debug: options.debug,
      json: options.json,
    })

    const fileDialect = path
      ? (options.dialect ?? createFileDialectFromPathAndOptions(path, options))
      : undefined

    const resource: Resource = path
      ? { data: path, fileDialect, tableSchema: options.schema }
      : await selectResource(session, options)

    const table = await session.task("Loading table preview", async () => {
      const table = await loadTable(resource, { previewBytes: 10_000 })
      if (!table) throw new Error("Could not load table")
      return table
    })

    const frame = await session.task("Collecting data", async () => {
      return await table.limit(100).collect()
    })

    session.renderFrameResult(frame)
  })
