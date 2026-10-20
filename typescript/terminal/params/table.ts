import type { TableCheck } from "@fairspec/library"
import { Argument, InvalidArgumentError, Option } from "commander"

export const query = new Argument(
  "[query]",
  "a SQL query to execute against a table (use `self` to refer to the table)",
)

export const check = new Option(
  "--check <check>",
  "a named SQL condition every row must satisfy, as name=condition (repeatable)",
).argParser((value: string, previous: TableCheck[] = []) => {
  const index = value.indexOf("=")
  if (index < 1) throw new InvalidArgumentError('Expected "name=condition"')
  return [...previous, { name: value.slice(0, index), condition: value.slice(index + 1) }]
})

export const overwrite = new Option(
  "--overwrite",
  "whether to overwrite a file if it already exists",
)
