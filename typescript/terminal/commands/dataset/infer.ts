import { inferDataset } from "@fairspec/library"
import { Command } from "commander"
import { helpConfiguration } from "../../helpers/help.ts"
import * as params from "../../params/index.ts"
import { Session } from "../../session.ts"

export const inferDatasetCommand = new Command()
  .name("infer")
  .description("Infer a dataset from local or remote file paths")
  .configureHelp(helpConfiguration)

  .addArgument(params.positionalFilePaths)
  .addOption(params.debug)
  .addOption(params.json)

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

  .action(async (paths, options) => {
    const session = new Session({
      debug: options.debug,
      json: options.json,
    })

    const dataset = await session.task("Infering dataset", async () => {
      const dataset = { resources: paths.map((data: string) => ({ data })) }
      return await inferDataset(dataset, options)
    })

    session.renderDataResult(dataset)
  })
