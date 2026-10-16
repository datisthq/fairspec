import { z } from "zod"

export const Descriptor = z.record(z.string(), z.unknown())

export type Descriptor = z.infer<typeof Descriptor>
