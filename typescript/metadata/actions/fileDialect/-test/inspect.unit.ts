import { describe, expect, it } from "vite-plus/test"
import { inspectFileDialect } from "../inspect.ts"

describe("inspectFileDialect", () => {
  it("should report a row listed in headerRows and commentRows", () => {
    const errors = inspectFileDialect(
      { format: "csv", headerRows: [1, 2], commentRows: [2] },
      { rootJsonPointer: "/fileDialect" },
    )

    expect(errors).toEqual([
      expect.objectContaining({
        type: "metadata",
        jsonPointer: "/fileDialect/commentRows",
      }),
    ])
  })

  it("should take the default header row", () => {
    expect(inspectFileDialect({ format: "csv", commentRows: [1] })).toHaveLength(1)
  })

  it("should accept commentRows without a header", () => {
    expect(
      inspectFileDialect({ format: "csv", headerRows: false, commentRows: [1] }),
    ).toEqual([])
  })

  it("should skip object rows, which commentRows does not apply to", () => {
    expect(
      inspectFileDialect({ format: "jsonl", rowType: "object", commentRows: [1] }),
    ).toEqual([])
  })

  it("should report headerRows out of ascending order", () => {
    const errors = inspectFileDialect(
      { format: "csv", headerRows: [1, 3, 2] },
      { rootJsonPointer: "/fileDialect" },
    )

    expect(errors).toEqual([
      expect.objectContaining({
        type: "metadata",
        jsonPointer: "/fileDialect/headerRows",
      }),
    ])
  })

  it("should accept headerRows in ascending order with a gap", () => {
    expect(inspectFileDialect({ format: "csv", headerRows: [1, 3] })).toEqual([])
  })
})
