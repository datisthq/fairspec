import { Option } from "commander"

export const dialect = new Option("--dialect <dialect>", "path to a Dialect descriptor")

export const format = new Option(
  "--format <format>",
  "format type (csv, json, xlsx, etc)",
)

export const delimiter = new Option(
  "--delimiter <char>",
  "character used to separate fields in the data",
)

export const lineTerminator = new Option(
  "--line-terminator <char>",
  "character sequence used to terminate rows",
)

export const quoteChar = new Option(
  "--quote-char <char>",
  "character used to quote fields",
)

export const nullSequence = new Option(
  "--null-sequence <seq>",
  "character sequence representing null or missing values",
)

export const headerRows = new Option(
  "--header-rows <rows>",
  "comma-separated row numbers (1-indexed) for headers, or 'false' to disable",
).argParser((value: string) => {
  if (value === "false") return false
  return value.split(",").map(Number)
})

export const headerJoin = new Option(
  "--header-join <char>",
  "character used to join multi-line headers",
)

export const commentRows = new Option(
  "--comment-rows <rows>",
  "comma-separated row numbers (1-indexed) to exclude from data",
).argParser((value: string) => value.split(",").map(Number))

export const commentPrefix = new Option(
  "--comment-prefix <prefix>",
  "character sequence denoting the start of a comment line",
)

export const columnNames = new Option(
  "--column-names <names>",
  "comma-separated list of column names",
).argParser((value: string) => value.split(","))

export const jsonPointer = new Option(
  "--json-pointer <pointer>",
  "JSON pointer to the data array within the JSON document",
)

export const rowType = new Option(
  "--row-type <type>",
  "the type of each row in the data",
).choices(["array", "object"])

export const sheetNumber = new Option(
  "--sheet-number <num>",
  "for spreadsheet data, the sheet number to read (0-indexed)",
).argParser(Number.parseInt)

export const sheetName = new Option(
  "--sheet-name <name>",
  "for spreadsheet data, the sheet name to read",
)

export const tableName = new Option(
  "--table-name <name>",
  "for database sources, the table name to read",
)
