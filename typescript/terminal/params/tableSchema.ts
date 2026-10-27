import { InferTableSchemaOptions } from "@fairspec/library"
import { f } from "fireargs"
import { z } from "zod"

const options = InferTableSchemaOptions.shape
const option = f.option({ helpGroup: "Table Schema:" })

export const tableSchema = option
  .schema(z.string().optional())
  .describe("path to a Table Schema descriptor, Fairspec or Frictionless")

export const tableSchemaParams = {
  columnTypes: f
    .option({ helpGroup: "Table Schema:", parse: parseColumnTypes })
    .schema(options.columnTypes)
    .describe("a list of comma-separated column name:type pairs to use for the schema"),
  columnProperties: f
    .option({ helpGroup: "Table Schema:", parse: value => JSON.parse(value) })
    .schema(options.columnProperties)
    .describe("a JSON object of column name to column property to use for the schema"),
  missingValues: f
    .option({ helpGroup: "Table Schema:", parse: value => value.split(",") })
    .schema(options.missingValues)
    .describe("comma-separated values to treat as missing"),
  decimalChar: option
    .schema(options.decimalChar)
    .describe("character to use as decimal separator"),
  groupChar: option
    .schema(options.groupChar)
    .describe("character to use for digit grouping"),
  trueValues: f
    .option({ helpGroup: "Table Schema:", parse: value => value.split(",") })
    .schema(options.trueValues)
    .describe("values to treat as true"),
  falseValues: f
    .option({ helpGroup: "Table Schema:", parse: value => value.split(",") })
    .schema(options.falseValues)
    .describe("values to treat as false"),
  datetimeFormat: option
    .schema(options.datetimeFormat)
    .describe("datetime format pattern"),
  dateFormat: option.schema(options.dateFormat).describe("date format pattern"),
  timeFormat: option.schema(options.timeFormat).describe("time format pattern"),
  arrayType: option.schema(options.arrayType).describe("array type (array or list)"),
  listDelimiter: option
    .schema(options.listDelimiter)
    .describe("delimiter for list values"),
  listItemType: option.schema(options.listItemType).describe("type of items in lists"),
  sampleRows: option
    .schema(z.number().int().optional())
    .describe("number of rows to sample for schema inference"),
  confidence: option
    .schema(options.confidence)
    .describe("confidence threshold for schema inference"),
  commaDecimal: option
    .schema(options.commaDecimal)
    .describe("use comma as decimal separator in schema inference"),
  monthFirst: option
    .schema(options.monthFirst)
    .describe("interpret dates as month-first in schema inference"),
  keepStrings: option
    .schema(options.keepStrings)
    .describe("keep fields as strings instead of inferring types"),
}

function parseColumnTypes(value: string) {
  const result: Record<string, string> = {}
  for (const pair of value.split(",")) {
    const [key, type] = pair.split(":")
    if (key && type) result[key] = type
  }
  return result
}
