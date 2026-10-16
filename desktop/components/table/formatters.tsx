import type { Column, TableSchema } from "@fairspec/metadata"
import type { ReactNode } from "react"

function isMissing(
  value: any,
  column: Column,
  globalMissing?: NonNullable<TableSchema["missingValues"]>,
): boolean {
  const fieldMissing = (column.property.missingValues ?? []) as NonNullable<
    TableSchema["missingValues"]
  >
  const missing = [...(globalMissing ?? []), ...fieldMissing]

  return missing.some(missingItem => {
    const missingValue = typeof missingItem === "object" ? missingItem.value : missingItem
    return missingValue === value || String(missingValue) === String(value)
  })
}

function formatBoolean(value: any, column: Column & { type: "boolean" }): ReactNode {
  const { trueValues = ["true"], falseValues = ["false"] } = column.property
  if (value === true) return trueValues[0]
  if (value === false) return falseValues[0]
  return null
}

function formatNumber(value: any): ReactNode {
  if (typeof value !== "number") return value
  return value.toLocaleString()
}

function formatDate(value: any): ReactNode {
  try {
    return new Date(value).toLocaleDateString()
  } catch {
    return value
  }
}

function formatDateTime(value: any): ReactNode {
  try {
    return new Date(value).toLocaleString()
  } catch {
    return value
  }
}

function formatList(value: any, _column: Column & { type: "list" }): ReactNode {
  if (Array.isArray(value)) {
    return value.join(", ")
  }
  return value
}

export function formatCell(
  value: any,
  column: Column,
  globalMissing?: NonNullable<TableSchema["missingValues"]>,
): ReactNode {
  if (isMissing(value, column, globalMissing)) {
    return <span className="text-gray-400 italic">—</span>
  }

  switch (column.type) {
    case "boolean":
      return formatBoolean(value, column)
    case "integer":
    case "number":
    case "decimal":
      return formatNumber(value)
    case "email":
      return (
        <a href={`mailto:${value}`} className="text-blue-600 hover:underline">
          {value}
        </a>
      )
    case "url":
      return (
        <a href={value} target="_blank" className="text-blue-600 hover:underline">
          {value}
        </a>
      )
    case "date":
      return formatDate(value)
    case "date-time":
      return formatDateTime(value)
    case "time":
      return value
    case "list":
      return formatList(value, column)
    case "categorical":
      return (
        <span className="px-2 py-1 rounded bg-gray-100 dark:bg-gray-800 text-xs">
          {value}
        </span>
      )
    case "array":
    case "object":
    case "geojson":
    case "topojson":
      return (
        <code className="text-xs bg-gray-100 dark:bg-gray-800 px-1 rounded">
          {JSON.stringify(value)}
        </code>
      )
    default:
      return String(value ?? "")
  }
}
