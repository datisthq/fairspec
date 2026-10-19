import { join } from "node:path"
import { describe, expect, it } from "vite-plus/test"
import { useRecording } from "./recording.ts"

describe("useRecording", () => {
  useRecording()

  it("makes a fetch request with polly recording", async () => {
    const response = await fetch("https://httpbin.org/get")
    expect(response.ok).toBe(true)
    expect(await response.json()).toMatchObject({ url: "https://httpbin.org/get" })
  })

  it("makes another request to verify per-test recording names", async () => {
    const response = await fetch("https://httpbin.org/status/200")
    expect(response.ok).toBe(true)
  })
})

describe("useRecording with custom options", () => {
  useRecording({
    recordingName: "custom-recording",
    recordingPath: join(import.meta.dirname, "fixtures", "generated"),
  })

  it("uses custom recording name", async () => {
    const response = await fetch("https://httpbin.org/get")
    expect(response.ok).toBe(true)
  })
})
