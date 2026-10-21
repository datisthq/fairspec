import { existsSync } from "node:fs"
import { join } from "node:path"
import { describe, expect, it } from "vite-plus/test"
import { generateDataset } from "../generate/generate.ts"
import { measureTool } from "../harness/measure.ts"
import { runCommand } from "../harness/run.ts"
import { simpleProfile } from "../profiles/simple.ts"
import type { Tool } from "../tools/types.ts"
import { useTempDirectory } from "./helpers.ts"

const node = (script: string) => ({ file: process.execPath, args: ["-e", script] })
const hasSystemdRun = existsSync("/usr/bin/systemd-run")

describe.skipIf(process.platform !== "linux")("runCommand", () => {
  const directory = useTempDirectory()

  it("reports invalid data (exit 1) as a finished run with its peak memory", async () => {
    const script =
      "const kept = Buffer.alloc(64 * 1024 * 1024, 1); setTimeout(() => process.exit(1), 300)"
    const run = await runCommand(node(script), { cwd: directory(), timeoutMs: 10000 })
    expect(run).toMatchObject({ outcome: "ok", exit: "code 1" })
    expect(run.peakMemoryMb).toBeGreaterThan(64)
    expect(run.peakRssMb).toBeGreaterThan(64)
  })

  it("reports a tool killed by the kernel as out of memory", async () => {
    const run = await runCommand(node("process.kill(process.pid, 'SIGKILL')"), {
      cwd: directory(),
      timeoutMs: 10000,
    })
    expect(run).toMatchObject({ outcome: "oom", exit: "signal 9" })
  })

  it("kills a run past its timeout", async () => {
    const run = await runCommand(node("setTimeout(() => {}, 60000)"), {
      cwd: directory(),
      timeoutMs: 300,
    })
    expect(run.outcome).toBe("timeout")
  })

  it.skipIf(!hasSystemdRun)(
    "reports a run killed at its memory cap as out of memory",
    async () => {
      const script =
        "const kept = Buffer.alloc(400 * 1024 * 1024, 1); setTimeout(() => {}, 5000)"
      const run = await runCommand(node(script), {
        cwd: directory(),
        timeoutMs: 20000,
        memoryMax: "100M",
      })
      expect(run.outcome).toBe("oom")
    },
  )

  it("reports a command that cannot start as a crash instead of waiting", async () => {
    const run = await runCommand(node("process.exit(0)"), {
      cwd: join(directory(), "missing"),
      timeoutMs: 10000,
    })
    expect(run.outcome).toBe("crash")
    expect(run.stderr).toContain("ENOENT")
  })

  it("reports any other exit code as a crash", async () => {
    const run = await runCommand(node("process.exit(3)"), {
      cwd: directory(),
      timeoutMs: 10000,
    })
    expect(run).toMatchObject({ outcome: "crash", exit: "code 3" })
  })
})

describe.skipIf(process.platform !== "linux")("measureTool", () => {
  const directory = useTempDirectory()

  function createFakeTool(
    errorCount: number,
    unsupported: Tool["unsupported"] = [],
  ): Tool {
    return {
      name: "fake",
      title: "Fake",
      unsupported,
      caveats: [],
      getVersion: async () => "1.0.0",
      createCommand: async () => node(`console.log(${errorCount})`),
      countErrors: async ({ stdout }) => Number(stdout),
    }
  }

  async function measure(tool: Tool) {
    const dataset = await generateDataset({
      profile: simpleProfile,
      size: 100,
      directory: directory(),
      faults: 5,
    })
    return measureTool({
      caseName: "simple",
      tool,
      dataset,
      runs: 2,
      warmup: 1,
      timeoutMs: 10000,
    })
  }

  it("is correct when the tool finds every fault", async () => {
    const result = await measure(createFakeTool(5))
    expect(result).toMatchObject({
      correct: true,
      errorCount: 5,
      expectedErrors: 5,
      runs: 2,
    })
  })

  it("is wrong when the tool misses a fault", async () => {
    const result = await measure(createFakeTool(4))
    expect(result).toMatchObject({ correct: false, outcome: "ok", errorCount: 4 })
  })

  it("does not expect faults of checks the tool cannot express", async () => {
    const result = await measure(createFakeTool(4, ["type"]))
    expect(result).toMatchObject({ correct: true, expectedErrors: 4 })
  })

  it("times a run by the tool's own clock when it reports one", async () => {
    const result = await measure({ ...createFakeTool(5), readElapsedMs: () => 42 })
    expect(result.wallMs).toEqual({ median: 42, min: 42, max: 42 })
  })

  it("marks unparseable output as a crash", async () => {
    const tool = {
      ...createFakeTool(5),
      countErrors: async () => Promise.reject(new Error()),
    }
    const result = await measure(tool)
    expect(result).toMatchObject({ correct: false, outcome: "crash", runs: 0 })
  })
})
