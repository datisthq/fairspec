import { readFile } from "node:fs/promises"
import { writeTempFile } from "@fairspec/dataset"
import { describe, expect, it } from "vite-plus/test"
import { cleanCsvRows } from "../rows.ts"

describe("cleanCsvRows", () => {
  it("should keep files without a blank line or a nullSequence", async () => {
    const path = await writeTempFile("a,b\r\n1,2\r\n")
    expect(await cleanCsvRows([path], { format: "csv" })).toBeUndefined()
  })

  it("should drop blank lines but not a blank line inside a quoted cell", async () => {
    const path = await writeTempFile('\na,b\n1,"x\n\ny"\n\r\n3,4\n\n')
    const cleaned = await cleanCsvRows([path], { format: "csv" })

    expect(await readFile(cleaned?.paths[0] ?? "", "utf8")).toBe('a,b\n1,"x\n\ny"\n3,4\n')
  })

  it("should keep a row of delimiters", async () => {
    const path = await writeTempFile("a,b\n,\n\n")
    const cleaned = await cleanCsvRows([path], { format: "csv" })

    expect(await readFile(cleaned?.paths[0] ?? "", "utf8")).toBe("a,b\n,\n")
  })

  it("should empty only unquoted data cells equal to the nullSequence", async () => {
    const path = await writeTempFile('NA,b\nNA,"NA"\n"a""NA",NA\r\n')
    const cleaned = await cleanCsvRows([path], { format: "csv", nullSequence: "NA" })

    expect(cleaned?.fileDialect.nullSequence).toBeUndefined()
    expect(await readFile(cleaned?.paths[0] ?? "", "utf8")).toBe(
      'NA,b\n,"NA"\n"a""NA",\r\n',
    )
  })

  it("should keep the quotes of tsv cells", async () => {
    const path = await writeTempFile('a\tb\n"NA\tNA"\n')
    const cleaned = await cleanCsvRows([path], { format: "tsv", nullSequence: "NA" })

    expect(await readFile(cleaned?.paths[0] ?? "", "utf8")).toBe('a\tb\n"NA\tNA"\n')
  })

  it("should not read quotes in a comment row", async () => {
    const path = await writeTempFile("a\n# it's\n\nx\n")
    const cleaned = await cleanCsvRows([path], {
      format: "csv",
      quoteChar: "'",
      commentPrefix: "#",
    })

    expect(await readFile(cleaned?.paths[0] ?? "", "utf8")).toBe("a\n# it's\nx\n")
  })
})
