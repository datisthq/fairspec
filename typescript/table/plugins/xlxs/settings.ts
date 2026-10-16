import type { Column } from "@fairspec/metadata"

export const NATIVE_TYPES = [
  "boolean",
  "integer",
  "number",
  "string",
] satisfies Column["type"][]
