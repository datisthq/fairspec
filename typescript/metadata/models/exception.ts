import type { Report } from "./report.ts"

export class FairspecException extends Error {
  report?: Report

  constructor(message: string, options?: { report?: Report }) {
    super(message)
    this.report = options?.report
  }
}
