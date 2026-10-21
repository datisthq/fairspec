import { execFile } from "node:child_process"
import { resolve } from "node:path"
import { promisify } from "node:util"
import { describe, expect, it } from "vite-plus/test"
import metadata from "../../../terminal/package.json" with { type: "json" }

describe("run", () => {
  it("should run the terminal bundle", async () => {
    const { stdout } = await promisify(execFile)(process.execPath, [
      resolve(import.meta.dirname, "../../build/entrypoints/run.js"),
      "--version",
    ])

    expect(stdout.trim()).toBe(metadata.version)
  })
})
