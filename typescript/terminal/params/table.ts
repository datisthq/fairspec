import { Argument, Option } from "commander"

export const query = new Argument(
  "[query]",
  "a SQL query to execute against a table (use `self` to refer to the table)",
)

export const overwrite = new Option(
  "--overwrite",
  "whether to overwrite a file if it already exists",
)
