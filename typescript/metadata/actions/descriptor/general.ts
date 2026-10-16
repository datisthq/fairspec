import type { Descriptor } from "../../models/descriptor.ts"

export function getIsDescriptor(value: unknown): value is Descriptor {
  return !!value && typeof value === "object" && !Array.isArray(value)
}
