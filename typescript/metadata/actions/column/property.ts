import { z } from "zod"
import type { BasePropertyType } from "../../models/column/base.ts"
import type { Column, ColumnProperty } from "../../models/column/column.ts"

export function createNullablePropertyType<T extends BasePropertyType>(literal: T) {
  const members: string[] = [
    literal,
    "null",
    ...(literal === "number" ? ["integer"] : []),
  ]
  return z.union([
    z.literal(literal),
    z
      .array(z.union([z.literal(literal), z.literal("integer"), z.literal("null")]))
      .refine(isUniqueTypeArray(literal, members), "Type array must name one type"),
  ])
}

export function getBasePropertyType(type: ColumnProperty["type"]) {
  if (type === undefined) return undefined
  if (typeof type === "string") return type
  const names = type.filter(name => name !== "null")
  if (names.length === 1) return names[0]
  if (names.length === 2 && names.includes("integer") && names.includes("number")) {
    return "number"
  }
  return undefined
}

export function getIsNullablePropertyType(type: ColumnProperty["type"]) {
  if (type === undefined) return false
  if (typeof type === "string") return false
  return type.includes("null")
}

function isUniqueTypeArray(literal: string, members: string[]) {
  return (type: string[]) =>
    type.includes(literal) &&
    new Set(type).size === type.length &&
    type.every(name => members.includes(name))
}

export function getColumnProperties(columns: Column[]) {
  return Object.fromEntries(columns.map(column => [column.name, column.property]))
}
