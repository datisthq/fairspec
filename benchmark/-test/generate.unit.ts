import { readFile, rename } from "node:fs/promises"
import { basename, join } from "node:path"
import { describe, expect, it } from "vite-plus/test"
import { generateDataset } from "../generate/generate.ts"
import { createRandom } from "../generate/random.ts"
import { parseSize } from "../generate/size.ts"
import { simpleProfile } from "../profiles/simple.ts"
import { useTempDirectory } from "./helpers.ts"

describe("createRandom", () => {
  it("repeats its sequence for a seed", () => {
    const left = createRandom(7)
    const right = createRandom(7)
    expect(Array.from({ length: 5 }, left.float)).toEqual(
      Array.from({ length: 5 }, right.float),
    )
  })

  it("keeps integers within bounds", () => {
    const random = createRandom(1)
    const values = Array.from({ length: 1000 }, () => random.integer(3, 5))
    expect(new Set(values)).toEqual(new Set([3, 4, 5]))
  })
})

describe("parseSize", () => {
  it("reads a row count or a byte target", () => {
    expect(parseSize(500)).toEqual({ rows: 500 })
    expect(parseSize("16GB")).toEqual({ bytes: 16e9 })
    expect(parseSize("1.5MB")).toEqual({ bytes: 1.5e6 })
    expect(() => parseSize("16GiB")).toThrow()
  })
})

describe("generateDataset", () => {
  const directory = useTempDirectory()

  it("writes the same bytes for the same seed", async () => {
    const options = { profile: simpleProfile, size: 200, faults: 5 }
    const first = await generateDataset({ ...options, directory: directory() })
    const second = await generateDataset({
      ...options,
      directory: join(directory(), "again"),
    })
    expect(await readFile(second.csvPath, "utf8")).toBe(
      await readFile(first.csvPath, "utf8"),
    )
  })

  it("places each fault value on its planned row", async () => {
    const dataset = await generateDataset({
      profile: simpleProfile,
      size: 1000,
      directory: directory(),
      faults: 10,
    })
    const lines = (await readFile(dataset.csvPath, "utf8")).trimEnd().split("\n")
    const header = lines[0]?.split(",") ?? []

    expect(lines).toHaveLength(1001)
    expect(dataset.faults).toHaveLength(10)
    expect(Object.values(dataset.expected).reduce((sum, count) => sum + count, 0)).toBe(
      10,
    )
    for (const fault of dataset.faults) {
      const cells = lines[fault.rowNumber]?.split(",") ?? []
      const value = simpleProfile.faults.find(item => item.column === fault.column)?.value
      expect(cells[header.indexOf(fault.column)]).toBe(value)
    }
  })

  it("reuses a moved dataset with paths in its new directory", async () => {
    const first = await generateDataset({
      profile: simpleProfile,
      size: 100,
      directory: join(directory(), "old"),
    })
    await rename(join(directory(), "old"), join(directory(), "new"))
    const moved = await generateDataset({
      profile: simpleProfile,
      size: 100,
      directory: join(directory(), "new"),
    })
    expect(moved.bytes).toBe(first.bytes)
    expect(moved.csvPath).toBe(join(directory(), "new", basename(first.csvPath)))
  })

  it("aims a byte target within a few percent", async () => {
    const dataset = await generateDataset({
      profile: simpleProfile,
      size: "1MB",
      directory: directory(),
    })
    expect(Math.abs(dataset.bytes - 1e6) / 1e6).toBeLessThan(0.05)
  })
})
