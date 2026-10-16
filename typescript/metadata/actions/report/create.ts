import type { FairspecError } from "../../models/error/error.ts"
import type { Report } from "../../models/report.ts"

export function createReport(
  errors?: FairspecError[],
  options?: { maxErrors?: number },
): Report {
  errors = (errors ?? []).slice(0, options?.maxErrors)
  const valid = errors.length === 0

  return { valid, errors }
}
