import type { TableSchemaOptions } from "@fairspec/library"

export function createTableSchemaOptions(
  options: Record<string, unknown>,
): TableSchemaOptions {
  const result: TableSchemaOptions = {}

  if (options.toColumnNames !== undefined)
    result.columnNames = options.toColumnNames as string[]
  if (options.toColumnTypes !== undefined)
    result.columnTypes = options.toColumnTypes as TableSchemaOptions["columnTypes"]
  if (options.toMissingValues !== undefined)
    result.missingValues = options.toMissingValues as string[]
  if (options.toDecimalChar !== undefined)
    result.decimalChar = options.toDecimalChar as string
  if (options.toGroupChar !== undefined) result.groupChar = options.toGroupChar as string
  if (options.toTrueValues !== undefined)
    result.trueValues = options.toTrueValues as string[]
  if (options.toFalseValues !== undefined)
    result.falseValues = options.toFalseValues as string[]
  if (options.toDatetimeFormat !== undefined)
    result.datetimeFormat = options.toDatetimeFormat as string
  if (options.toDateFormat !== undefined)
    result.dateFormat = options.toDateFormat as string
  if (options.toTimeFormat !== undefined)
    result.timeFormat = options.toTimeFormat as string
  if (options.toArrayType !== undefined)
    result.arrayType = options.toArrayType as "array" | "list"
  if (options.toListDelimiter !== undefined)
    result.listDelimiter = options.toListDelimiter as string
  if (options.toListItemType !== undefined)
    result.listItemType = options.toListItemType as TableSchemaOptions["listItemType"]

  return result
}
