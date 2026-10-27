import { f } from "fireargs"
import { z } from "zod"

const option = f.option({ helpGroup: "Dialect:" })

export const dialectParams = {
  dialect: option
    .schema(z.string().optional())
    .describe("path to a Dialect descriptor, Fairspec or Frictionless"),
  format: option
    .schema(z.string().optional())
    .describe("format type (csv, json, xlsx, etc)"),
  delimiter: option
    .schema(z.string().optional())
    .describe("character used to separate fields in the data"),
  lineTerminator: option
    .schema(z.string().optional())
    .describe("character sequence used to terminate rows"),
  quoteChar: option
    .schema(z.string().optional())
    .describe("character used to quote fields"),
  nullSequence: option
    .schema(z.string().optional())
    .describe("character sequence representing null or missing values"),
  headerRows: f
    .option({
      helpGroup: "Dialect:",
      parse: value => (value === "false" ? false : value.split(",").map(Number)),
    })
    .schema(z.union([z.literal(false), z.array(z.number().int())]).optional())
    .describe(
      "comma-separated row numbers (1-indexed) for headers, or 'false' to disable",
    ),
  headerJoin: option
    .schema(z.string().optional())
    .describe("character used to join multi-line headers"),
  commentRows: f
    .option({ helpGroup: "Dialect:", parse: value => value.split(",").map(Number) })
    .schema(z.array(z.number().int()).optional())
    .describe("comma-separated row numbers (1-indexed) to exclude from data"),
  commentPrefix: option
    .schema(z.string().optional())
    .describe("character sequence denoting the start of a comment line"),
  columnNames: f
    .option({ helpGroup: "Dialect:", parse: value => value.split(",") })
    .schema(z.array(z.string()).optional())
    .describe("comma-separated list of column names"),
  jsonPointer: option
    .schema(z.string().optional())
    .describe("JSON pointer to the data array within the JSON document"),
  rowType: option
    .schema(z.enum(["array", "object"]).optional())
    .describe("the type of each row in the data"),
  sheetNumber: option
    .schema(z.number().int().optional())
    .describe("for spreadsheet data, the sheet number to read (0-indexed)"),
  sheetName: option
    .schema(z.string().optional())
    .describe("for spreadsheet data, the sheet name to read"),
  tableName: option
    .schema(z.string().optional())
    .describe("for database sources, the table name to read"),
}
