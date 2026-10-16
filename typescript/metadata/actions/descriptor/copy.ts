import type { Descriptor } from "../../models/descriptor.ts"

export function copyDescriptor<T extends Descriptor>(descriptor: T) {
  return globalThis.structuredClone(descriptor)
}
