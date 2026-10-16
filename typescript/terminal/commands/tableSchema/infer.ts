import type { Resource } from "@fairspec/library"
import { inferTableSchema } from "@fairspec/library"
import { Command } from "commander"
import { createFileDialectFromPathAndOptions } from "../../helpers/fileDialect.ts"
import { helpConfiguration } from "../../helpers/help.ts"
import { selectResource } from "../../helpers/resource.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const inferTableSchemaCommand = new Command()
  .name("infer")
  .description("Infer a table schema from a table")
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
      ? { data: path, fileDialect }
      : await selectResource(session, options)

    const tableSchema = await session.task("Inferring schema", async () => {
      const tableSchema = await inferTableSchema(resource, options)

      if (!tableSchema) {
        throw new Error("Could not infer table schema")
      }

      return tableSchema
    })

    session.renderDataResult(tableSchema)
  })
