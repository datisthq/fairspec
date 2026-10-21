import type { Dataset } from "../generate/generate.ts"
import type { Check } from "../profiles/types.ts"

export type Tool = {
  name: string
  title: string
  /** Checks the tool cannot express; the correctness gate does not expect their faults. */
  unsupported: Check[]
  /** What the tool checks differently from Fairspec, shown in the report. */
  caveats: string[]
  /** Not a full validator: the report lists it after every validator and says so. */
  isPartial?: boolean
  getVersion: () => Promise<string>
  /** Returns the command validating the dataset, run from the dataset's directory. */
  createCommand: (dataset: Dataset) => Promise<Command>
  countErrors: (output: { stdout: string; dataset: Dataset }) => Promise<number>
}

export type Command = {
  file: string
  args: string[]
  /** Exit codes of a completed validation; `[0, 1]` (valid, invalid) by default. */
  okExitCodes?: number[]
  /** Files the command writes, removed after each run. */
  outputs?: string[]
}
