import QuickLRU from "quick-lru"
import type { JsonSchema } from "../../models/jsonSchema.ts"

export const cache = new QuickLRU<string, JsonSchema>({
  maxSize: 100,
})
