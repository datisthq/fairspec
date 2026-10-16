import type { Descriptor } from "../../models/descriptor.ts"

export function stringifyDescriptor(descriptor: Descriptor) {
  return JSON.stringify(descriptor, null, 2)
}
