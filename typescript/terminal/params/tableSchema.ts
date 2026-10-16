import type { Column, ListColumn } from "@fairspec/library"
import { Option } from "commander"

export const tableSchema = new Option(
  "--schema <schema>",
  "path to a Table Schema descriptor",
)

export const sampleRows = new Option(
  "--sample-rows <sampleRows>",
  "number of rows to sample for schema inference",
).argParser(Number.parseInt)

export const confidence = new Option(
  "--confidence <confidence>",
  "confidence threshold for schema inference",
).argParser(Number.parseFloat)

export const commaDecimal = new Option(
  "--comma-decimal",
  "use comma as decimal separator in schema inference",
)

export const monthFirst = new Option(
  "--month-first",
  "interpret dates as month-first in schema inference",
)

export const keepStrings = new Option(
  "--keep-strings",
  "keep fields as strings instead of inferring types",
)

export const columnTypes = new Option(
  "--column-types <columnTypes>",
  "a list of comma-separated column name:type pairs to use for the schema",
).argParser((value: string) => {
  const result: Record<string, Column["type"]> = {}
  value.split(",").forEach(pair => {
    const [key, val] = pair.split(":")
    if (key && val) result[key] = val as Column["type"]
  })
  return result
})

export const missingValues = new Option(
  "--missing-values <missingValues>",
  "comma-separated values to treat as missing",
).argParser((value: string) => value.split(","))

export const decimalChar = new Option(
  "--decimal-char <decimalChar>",
  "character to use as decimal separator",
)

export const groupChar = new Option(
  "--group-char <groupChar>",
  "character to use for digit grouping",
)

export const trueValues = new Option(
  "--true-values <trueValues>",
  "values to treat as true",
).argParser((value: string) => value.split(","))

export const falseValues = new Option(
  "--false-values <falseValues>",
  "values to treat as false",
).argParser((value: string) => value.split(","))

export const datetimeFormat = new Option(
  "--datetime-format <datetimeFormat>",
  "datetime format pattern",
)

export const dateFormat = new Option("--date-format <dateFormat>", "date format pattern")

export const timeFormat = new Option("--time-format <timeFormat>", "time format pattern")

export const arrayType = new Option(
  "--array-type <arrayType>",
  "array type (array or list)",
).choices(["array", "list"])

export const listDelimiter = new Option(
  "--list-delimiter <listDelimiter>",
  "delimiter for list values",
)

export const listItemType = new Option(
  "--list-item-type <listItemType>",
  "type of items in lists",
).choices([
  "string",
  "integer",
  "number",
  "boolean",
  "date-time",
  "date",
  "time",
] satisfies ListColumn["property"]["itemType"][])
