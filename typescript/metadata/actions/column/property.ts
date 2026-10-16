import { z } from "zod"
import type { BasePropertyType } from "../../models/column/base.ts"
import type { Column, ColumnProperty } from "../../models/column/column.ts"

export function createNullablePropertyType<T extends BasePropertyType>(literal: T) {
  return z.union([
    z.literal(literal),
    z.tuple([z.literal(literal), z.literal("null")]),
    z.tuple([z.literal("null"), z.literal(literal)]),
  ])
}

export function getBasePropertyType(type: ColumnProperty["type"]) {
  if (type === undefined) return undefined
  if (typeof type === "string") return type
  return type.find(t => t !== "null") ?? "null"
}

export function getIsNullablePropertyType(type: ColumnProperty["type"]) {
  if (type === undefined) return false
  if (typeof type === "string") return false
  return type.includes("null")
}

export function getColumnProperties(columns: Column[]) {
  return Object.fromEntries(columns.map(column => [column.name, column.property]))
}
