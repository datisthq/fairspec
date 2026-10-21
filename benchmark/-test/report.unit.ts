import { describe, expect, it } from "vite-plus/test"
import type { Result } from "../harness/measure.ts"
import { renderReport } from "../report/render.ts"

const machine = { cpu: "Test CPU", cpus: 4, memory: "8GB", os: "linux test" }

function createResult(overrides: Partial<Result>): Result {
  return {
    case: "nyc-311",
    tool: "fairspec-python",
    title: "Fairspec Python",
    version: "1.0.0",
    profile: "nyc-311",
    size: "16GB",
    rows: 30000000,
    bytes: 16e9,
    runs: 1,
    wallMs: { median: 90000, min: 90000, max: 90000 },
    peakMemoryMb: 2048,
    outcome: "ok",
    errorCount: 100,
    expectedErrors: 100,
    correct: true,
    unsupported: [],
    caveats: [],
    ...overrides,
  }
}

describe("renderReport", () => {
  it("shows each outcome with a labelled status", () => {
    const html = renderReport({
      measuredAt: "2026-10-20T00:00:00.000Z",
      machine,
      results: [
        createResult({}),
        createResult({
          tool: "pandera",
          title: "pandera",
          correct: false,
          errorCount: 80,
        }),
        createResult({
          tool: "great-expectations",
          title: "Great Expectations",
          outcome: "oom",
          correct: false,
          wallMs: undefined,
          peakMemoryMb: undefined,
          errorCount: undefined,
          failedAfterMs: 240000,
        }),
      ],
    })
    expect(html).toContain("How fast does a 16GB table validate?")
    expect(html).toContain("✓ correct")
    expect(html).toContain("≠ wrong count")
    expect(html).toContain("out of memory after 4 min 0 s")
  })

  it("lists a partial tool after every validator and says so", () => {
    const html = renderReport({
      measuredAt: "2026-10-20T00:00:00.000Z",
      machine,
      results: [
        createResult({}),
        createResult({
          tool: "soda",
          title: "Soda Core",
          isPartial: true,
          wallMs: { median: 1000, min: 1000, max: 1000 },
        }),
        createResult({ tool: "qsv", title: "qsv" }),
      ],
    })

    const rows = [...html.matchAll(/<tr><td>([^<]+)<\/td><td><span class="chip/g)].map(
      match => match[1],
    )
    expect(rows).toEqual(["Fairspec Python", "qsv", "Soda Core (not a full validator)"])
    expect(html).toContain(
      '<span class="value">1 min 30 s</span><span class="note">qsv</span>',
    )
  })

  it("escapes text from tool output", () => {
    const html = renderReport({
      measuredAt: "2026-10-20T00:00:00.000Z",
      machine,
      results: [createResult({ caveats: ["<script>alert(1)</script>"] })],
    })
    expect(html).not.toContain("<script>")
    expect(html).toContain("&lt;script&gt;")
  })
})
