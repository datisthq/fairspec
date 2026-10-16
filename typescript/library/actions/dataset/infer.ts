import os from "node:os"
import type { Dataset } from "@fairspec/metadata"
import pAll from "p-all"
import type { InferResourceOptions } from "../../actions/resource/infer.ts"
import { inferResource } from "../../actions/resource/infer.ts"

export async function inferDataset(dataset: Dataset, options?: InferResourceOptions) {
  const concurrency = os.cpus().length

  const resources = await pAll(
    (dataset.resources ?? []).map(res => () => inferResource(res, options)),
    { concurrency },
  )

  const result = {
    ...dataset,
    resources,
  }

  return result
}
