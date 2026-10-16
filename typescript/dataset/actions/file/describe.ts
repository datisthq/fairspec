import type { HashType } from "./infer.ts"
import { inferBytes, inferIntegrity, inferTextual } from "./infer.ts"
import { prefetchFile } from "./prefetch.ts"

export async function describeFile(path: string, options?: { hashType?: HashType }) {
  const localPath = await prefetchFile(path)

  const bytes = await inferBytes({ data: localPath })
  const textual = await inferTextual({ data: localPath })
  const integrity = await inferIntegrity({ data: localPath }, options)

  return { bytes, textual, integrity }
}
