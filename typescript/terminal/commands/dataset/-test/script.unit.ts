import { basename } from "node:path"
import { writeTempFile } from "@fairspec/library"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"

let mockReplSession: { context: Record<string, unknown> }
const replStartMock = vi.fn(() => mockReplSession)

vi.mock("node:repl", () => ({
  default: {
    start: replStartMock,
  },
}))

const { scriptDatasetCommand } = await import("../script.ts")

// TODO: recover
// Skipped: script commands start interactive REPL sessions which are not suitable
// for automated testing. The tasuku library used by Session also conflicts with
// the test environment when trying to mock console and streams.
describe.skip("dataset script", () => {
  beforeEach(() => {
    vi.spyOn(process.stdout, "write").mockImplementation(() => true)
    vi.spyOn(process.stderr, "write").mockImplementation(() => true)

    mockReplSession = { context: {} }
    replStartMock.mockClear()
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  it("should start a REPL session with dataset context", async () => {
    const csvPath = await writeTempFile("id,name\n1,alice\n2,bob", {
      format: "csv",
    })
    const descriptor = {
      resources: [{ data: basename(csvPath) }],
    }
    const descriptorPath = await writeTempFile(JSON.stringify(descriptor), {
      format: "json",
    })

    try {
      await scriptDatasetCommand.parseAsync([descriptorPath], { from: "user" })
    } catch {}

    expect(replStartMock).toHaveBeenCalledWith({ prompt: "fairspec> " })
    expect(mockReplSession.context).toHaveProperty("fairspec")
    expect(mockReplSession.context).toHaveProperty("dataset")
  })

  it("should load dataset with single resource into REPL context", async () => {
    const csvPath = await writeTempFile("id,value\n1,100\n2,200", {
      format: "csv",
    })
    const descriptor = {
      resources: [{ name: "data", data: basename(csvPath) }],
    }
    const descriptorPath = await writeTempFile(JSON.stringify(descriptor), {
      format: "json",
    })

    try {
      await scriptDatasetCommand.parseAsync([descriptorPath], { from: "user" })
    } catch {}

    expect(replStartMock).toHaveBeenCalled()
    expect(mockReplSession.context.dataset).toBeDefined()
    expect(mockReplSession.context.fairspec).toBeDefined()
  })

  it("should load dataset with multiple resources into REPL context", async () => {
    const csv1Path = await writeTempFile("id,name\n1,alice", {
      format: "csv",
    })
    const csv2Path = await writeTempFile("id,age\n1,25", { format: "csv" })
    const descriptor = {
      resources: [
        { name: "users", data: basename(csv1Path) },
        { name: "ages", data: basename(csv2Path) },
      ],
    }
    const descriptorPath = await writeTempFile(JSON.stringify(descriptor), {
      format: "json",
    })

    try {
      await scriptDatasetCommand.parseAsync([descriptorPath], { from: "user" })
    } catch {}

    expect(replStartMock).toHaveBeenCalledWith({ prompt: "fairspec> " })
    expect(mockReplSession.context).toHaveProperty("dataset")
    expect(mockReplSession.context).toHaveProperty("fairspec")
  })
})
