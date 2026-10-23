import type { Descriptor } from "../../models/descriptor.ts"

export function parseDescriptor(text: string) {
  const value = JSON.parse(text)

  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error(
      `Invalid descriptor: expected a JSON object, got ${getJsonType(value)}`,
    )
  }

  return value as Descriptor
}

function getJsonType(value: unknown) {
  if (value === null) return "null"
  if (Array.isArray(value)) return "array"
  return typeof value
}
