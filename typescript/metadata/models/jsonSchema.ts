import { z } from "zod"

export const JsonSchema = z.record(z.string(), z.unknown())

export type JsonSchema = z.infer<typeof JsonSchema>
