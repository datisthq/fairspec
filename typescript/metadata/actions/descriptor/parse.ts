import type { Descriptor } from "../../models/descriptor.ts"

export function parseDescriptor(text: string) {
  const value = JSON.parse(text)

  if (typeof value !== "object" || Array.isArray(value)) {
    throw new Error(`Invalid descriptor: ${text}`)
  }

  return value as Descriptor
}
