import { dirname, join } from "node:path"
import FetchAdapter from "@pollyjs/adapter-fetch"
import { Polly } from "@pollyjs/core"
import FSPersister from "@pollyjs/persister-fs"
import { afterAll, beforeAll, beforeEach } from "vite-plus/test"

// @ts-expect-error CJS default-import types mismatch under NodeNext; runtime value is the class
Polly.register(FSPersister)
// @ts-expect-error CJS default-import types mismatch under NodeNext; runtime value is the class
Polly.register(FetchAdapter)

/**
 * Records the suite's fetch requests on first run and replays them afterwards.
 * Recordings land in `fixtures/generated` next to the test file unless `recordingPath` is set.
 * https://github.com/Netflix/pollyjs/issues/499
 */
export function useRecording(
  options: { recordingName?: string; recordingPath?: string } = {},
) {
  let polly: Polly

  beforeAll(({}, suite) => {
    polly = new Polly(options.recordingName ?? suite.name, {
      adapters: ["fetch"],
      mode: "replay",
      recordIfMissing: true,
      recordFailedRequests: true,
      persister: "fs",
      persisterOptions: {
        fs: {
          recordingsDir:
            options.recordingPath ??
            join(dirname(suite.file.filepath), "fixtures", "generated"),
        },
      },
    })
  })

  beforeEach(({ task }) => {
    polly.recordingName = options.recordingName ?? task.fullTestName ?? task.name
  })

  afterAll(async () => {
    await polly.stop()
  })
}
