import tab from "@bomb.sh/tab/commander"
import { f } from "fireargs"
import { validateDataCommand } from "./commands/data/validate.ts"
import { inferDataSchemaCommand } from "./commands/dataSchema/infer.ts"
import { validateDataSchemaCommand } from "./commands/dataSchema/validate.ts"
import { copyDatasetCommand } from "./commands/dataset/copy.ts"
import { inferDatasetCommand } from "./commands/dataset/infer.ts"
import { listDatasetCommand } from "./commands/dataset/list.ts"
import { queryDatasetCommand } from "./commands/dataset/query.ts"
import { scriptDatasetCommand } from "./commands/dataset/script.ts"
import { validateDatasetCommand } from "./commands/dataset/validate.ts"
import { copyFileCommand } from "./commands/file/copy.ts"
import { describeFileCommand } from "./commands/file/describe.ts"
import { previewFileCommand } from "./commands/file/preview.ts"
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

const datasetCommand = f
  .program({
    description: "Dataset related commands",
    configureHelp: helpConfiguration,
    helpCommand: { name: "help [command]" },
  })
  .router({
    infer: inferDatasetCommand,
    copy: copyDatasetCommand,
    validate: validateDatasetCommand,
    list: listDatasetCommand,
    query: queryDatasetCommand,
    script: scriptDatasetCommand,
  })

const tableCommand = f
  .program({
    description: "Table related commands",
    configureHelp: helpConfiguration,
    helpCommand: { name: "help [command]" },
  })
  .router({
    describe: describeTableCommand,
    preview: previewTableCommand,
    query: queryTableCommand,
    script: scriptTableCommand,
    validate: validateTableCommand,
    "infer-schema": inferTableSchemaCommand,
    "render-schema": renderTableSchemaCommand,
    "validate-schema": validateTableSchemaCommand,
    "infer-dialect": inferFileDialectCommand,
  })

const dataCommand = f
  .program({
    description: "Data related commands",
    configureHelp: helpConfiguration,
    helpCommand: { name: "help [command]" },
  })
  .router({
    validate: validateDataCommand,
    "infer-schema": inferDataSchemaCommand,
    "validate-schema": validateDataSchemaCommand,
    "infer-dialect": inferFileDialectCommand,
  })

const fileCommand = f
  .program({
    description: "File related commands",
    configureHelp: helpConfiguration,
    helpCommand: { name: "help [command]" },
  })
  .router({
    copy: copyFileCommand,
    describe: describeFileCommand,
    preview: previewFileCommand,
    validate: validateFileCommand,
    "infer-dialect": inferFileDialectCommand,
  })

export const program = f
  .program({
    name: "fairspec",
    description: metadata.description,
    version: metadata.version,
    versionOption: { flags: "-v, --version" },
    configureHelp: helpConfiguration,
    helpCommand: { name: "help [command]" },
  })
  .router({
    dataset: datasetCommand,
    table: tableCommand,
    data: dataCommand,
    file: fileCommand,
    mcp: mcpCommand,
  })

tab(program)
