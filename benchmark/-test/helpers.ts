import { mkdtemp, rm } from "node:fs/promises"
import { tmpdir } from "node:os"
import { join } from "node:path"
import { afterEach, beforeEach } from "vite-plus/test"

export function useTempDirectory() {
  let directory = ""
  beforeEach(async () => {
    directory = await mkdtemp(join(tmpdir(), "fairspec-benchmark-test-"))
  })
  afterEach(async () => {
    await rm(directory, { recursive: true, force: true })
  })
  return () => directory
}
