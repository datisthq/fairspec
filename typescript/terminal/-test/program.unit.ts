import { writeTempFile } from "@fairspec/library"
import { afterEach, beforeEach, describe, expect, it, vi } from "vite-plus/test"
import { program } from "../program.ts"

describe("program", () => {
  let originalExitCode: typeof process.exitCode
  let text: string[]

  beforeEach(() => {
    originalExitCode = process.exitCode
    process.exitCode = undefined
    text = []
    vi.spyOn(process.stdout, "write").mockImplementation((msg: string | Uint8Array) => {
      text.push(typeof msg === "string" ? msg : msg.toString())
      return true
    })
    vi.spyOn(process.stderr, "write").mockImplementation(() => true)
  })

  afterEach(() => {
    vi.restoreAllMocks()
    process.exitCode = originalExitCode
  })

  it("should list every command once in the --llms manifest", async () => {
    await program.parseAsync(["--llms"], { from: "user" })

    const names: string[] = JSON.parse(text.join("")).tools.map(
      (tool: { name: string }) => tool.name,
    )

    expect(new Set(names).size).toBe(names.length)
    expect(names).toContain("table validate")
    expect(names).toContain("table infer-dialect")
    expect(names).toContain("data infer-dialect")
    expect(names).toContain("file infer-dialect")
    expect(names).toContain("mcp")
  })

  it("should take a command's input as JSON", async () => {
    const csvPath = await writeTempFile("id,name\n1,alice\n2,bob", { format: "csv" })

    await program.parseAsync(
      ["table", "preview", "--json", JSON.stringify({ path: csvPath })],
      { from: "user" },
    )

    expect(JSON.parse(text.join(""))).toEqual([
      { id: 1, name: "alice" },
      { id: 2, name: "bob" },
    ])
  })

  it("should parse a comma-separated option from the command line", async () => {
    const csvPath = await writeTempFile("id,name\n1,alice\n2,-", { format: "csv" })

    await program.parseAsync(
      ["table", "infer-schema", csvPath, "--missing-values", "-,n/a", "--json"],
      { from: "user" },
    )

    expect(JSON.parse(text.join(""))).toHaveProperty("missingValues", ["-", "n/a"])
  })

  it("should collect a repeated --check option", async () => {
    const csvPath = await writeTempFile("id\n1\n2", { format: "csv" })

    await program.parseAsync(
      [
        "table",
        "validate",
        csvPath,
        "--check",
        "a=id > 1",
        "--check",
        "b=id > 2",
        "--json",
      ],
      { from: "user" },
    )

    const report = JSON.parse(text.join(""))
    const checkNames = report.errors.map(
      (error: { checkName: string }) => error.checkName,
    )
    expect(new Set(checkNames)).toEqual(new Set(["a", "b"]))
    expect(process.exitCode).toBe(1)
  })
})
