import tab from "@bomb.sh/tab/commander"
import * as commander from "commander"
import { validateDataCommand } from "./commands/data/validate.ts"
import { inferDataSchemaCommand } from "./commands/dataSchema/infer.ts"
import { validateDataSchemaCommand } from "./commands/dataSchema/validate.ts"
import { copyDatasetCommand } from "./commands/dataset/copy.ts"
import { inferDatasetCommand } from "./commands/dataset/infer.ts"
import { listDatasetCommand } from "./commands/dataset/list.ts"
import { scriptDatasetCommand } from "./commands/dataset/script.ts"
import { validateDatasetCommand } from "./commands/dataset/validate.ts"
import { copyFileCommand } from "./commands/file/copy.ts"
import { describeFileCommand } from "./commands/file/describe.ts"
import { validateFileCommand } from "./commands/file/validate.ts"
import { inferFileDialectCommand } from "./commands/fileDialect/infer.ts"
import { mcpCommand } from "./commands/general/mcp.ts"
import { describeTableCommand } from "./commands/table/describe.ts"
import { previewTableCommand } from "./commands/table/preview.ts"
import { queryTableCommand } from "./commands/table/query.ts"
import { scriptTableCommand } from "./commands/table/script.ts"
import { validateTableCommand } from "./commands/table/validate.ts"
import { inferTableSchemaCommand } from "./commands/tableSchema/infer.ts"
import { renderTableSchemaCommand } from "./commands/tableSchema/render.ts"
import { validateTableSchemaCommand } from "./commands/tableSchema/validate.ts"
import { helpConfiguration } from "./helpers/help.ts"
import metadata from "./package.json" with { type: "json" }

const datasetCommand = new commander.Command()
  .configureHelp(helpConfiguration)
  .description("Dataset related commands")

  .addCommand(inferDatasetCommand.name("infer"))
  .addCommand(copyDatasetCommand.name("copy"))
  .addCommand(validateDatasetCommand.name("validate"))
  .addCommand(listDatasetCommand.name("list"))
  .addCommand(scriptDatasetCommand.name("script"))

const tableCommand = new commander.Command()
  .configureHelp(helpConfiguration)
  .description("Table related commands")

  .addCommand(describeTableCommand.name("describe"))
  .addCommand(previewTableCommand.name("preview"))
  .addCommand(queryTableCommand.name("query"))
  .addCommand(scriptTableCommand.name("script"))
  .addCommand(validateTableCommand.name("validate"))
  .addCommand(inferTableSchemaCommand.name("infer-schema"))
  .addCommand(renderTableSchemaCommand.name("render-schema"))
  .addCommand(validateTableSchemaCommand.name("validate-schema"))
  .addCommand(inferFileDialectCommand.name("infer-dialect"))

const dataCommand = new commander.Command()
  .configureHelp(helpConfiguration)
  .description("Data related commands")

  .addCommand(validateDataCommand.name("validate"))
  .addCommand(inferDataSchemaCommand.name("infer-schema"))
  .addCommand(validateDataSchemaCommand.name("validate-schema"))
  .addCommand(inferFileDialectCommand.name("infer-dialect"))

const fileCommand = new commander.Command()
  .configureHelp(helpConfiguration)
  .description("File related commands")

  .addCommand(copyFileCommand.name("copy"))
  .addCommand(describeFileCommand.name("describe"))
  .addCommand(validateFileCommand.name("validate"))
  .addCommand(inferFileDialectCommand.name("infer-dialect"))

export const program = commander.program
  .name("fairspec")
  .description(metadata.description)

  .version(metadata.version, "-v, --version")
  .configureHelp(helpConfiguration)

  .addCommand(datasetCommand.name("dataset"))
  .addCommand(tableCommand.name("table"))
  .addCommand(dataCommand.name("data"))
  .addCommand(fileCommand.name("file"))
  .addCommand(mcpCommand.name("mcp"))

tab(program)
