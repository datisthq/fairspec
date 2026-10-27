import type { MetadataError } from "@fairspec/metadata"
import { createReport, FairspecException } from "@fairspec/metadata"
import type { z } from "zod"

/**
 * Parse a Frictionless descriptor, raising its mismatches as a metadata report
 */
export function assertFrictionlessModel<T extends z.ZodType>(
  descriptor: unknown,
  options: { model: T; jsonPointer?: string },
): z.infer<T> {
  const result = options.model.safeParse(descriptor)
  if (result.success) return result.data

  const errors: MetadataError[] = result.error.issues.map(issue => ({
    type: "metadata",
    message: issue.message,
    jsonPointer:
      (options.jsonPointer ?? "") +
      issue.path
        .map(key => `/${String(key).replaceAll("~", "~0").replaceAll("/", "~1")}`)
        .join(""),
  }))

  throw new FairspecException("Invalid Frictionless descriptor", {
    report: createReport(errors),
  })
}
