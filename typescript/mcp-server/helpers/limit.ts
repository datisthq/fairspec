import type { Report } from "@fairspec/library"

export const DEFAULT_LIMIT = 100
export const MAX_LIMIT = 1000
export const MAX_RESPONSE_CHARS = 50_000

// room for the object a trimmed list is wrapped in
const MAX_ITEMS_CHARS = MAX_RESPONSE_CHARS - 1_000

/**
 * Cut a list to `limit` items, then to as many as fit the response budget.
 * Pass one item more than the limit to learn whether the list was truncated.
 */
export function limitItems<T>(items: T[], options: { limit: number }) {
  let chars = 0
  let count = 0

  for (const item of items.slice(0, options.limit)) {
    chars += JSON.stringify(item).length + 1
    if (chars > MAX_ITEMS_CHARS) break
    count++
  }

  return { items: items.slice(0, count), truncated: count < items.length }
}

/**
 * Cut a report's errors to `maxErrors`, then to the response budget.
 */
export function limitReport(report: Report, options: { maxErrors?: number }) {
  const { items: errors, truncated } = limitItems(report.errors, {
    limit: options.maxErrors ?? DEFAULT_LIMIT,
  })

  return { report: { valid: report.valid, errors }, truncated }
}
