import { validateTableSchema } from "@fairspec/metadata"
import { describe, expect, it } from "vite-plus/test"
import { createRandom } from "../generate/random.ts"
import { PROFILES } from "../profiles/index.ts"

describe.each(Object.values(PROFILES))("$name profile", profile => {
  it("has a valid Table Schema", async () => {
    const report = await validateTableSchema(profile.tableSchema)
    expect(report.errors).toEqual([])
  })

  it("generates exactly the columns its schema describes", () => {
    expect(profile.columns.map(column => column.name)).toEqual(
      Object.keys(profile.tableSchema.properties ?? {}),
    )
  })

  it("faults only known columns, with values it never generates", () => {
    const random = createRandom(1)
    const rows = Array.from({ length: 2000 }, (_, index) =>
      profile.columns.map(column => column.generate(random, index + 1)),
    )
    for (const fault of profile.faults) {
      const index = profile.columns.findIndex(column => column.name === fault.column)
      expect(index).toBeGreaterThanOrEqual(0)
      expect(rows.some(row => row[index] === fault.value)).toBe(false)
    }
  })

  it("never generates a null in a required column", () => {
    const random = createRandom(2)
    const required = new Set(profile.tableSchema.required ?? [])
    for (let row = 1; row <= 2000; row++) {
      for (const column of profile.columns) {
        const value = column.generate(random, row)
        if (required.has(column.name)) expect(value, column.name).not.toBe("")
      }
    }
  })
})
