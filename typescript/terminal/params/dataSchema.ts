import { Option } from "commander"

export const dataSchema = new Option(
  "--schema <dataSchema>",
  "path to a data schema descriptor (JSON Schema)",
).makeOptionMandatory()
