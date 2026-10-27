import { writeTempFile } from "@fairspec/dataset"
import { describe, expect, it } from "vite-plus/test"
import { useRecording } from "@fairspec/metadata/testing"
import { loadCsvTable } from "../load.ts"

useRecording()

describe("loadCsvTable", () => {
  it("should load local file", async () => {
    const path = await writeTempFile("id,name\n1,english\n2,中文")
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv" },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "english" },
      { id: 2, name: "中文" },
    ])
  })

  it("should load local file (multipart)", async () => {
    const path1 = await writeTempFile("id,name\n1,english")
    const path2 = await writeTempFile("id,name\n2,中文\n3,german")

    const table = await loadCsvTable({
      data: [path1, path2],
      fileDialect: { format: "csv" },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "english" },
      { id: 2, name: "中文" },
      { id: 3, name: "german" },
    ])
  })

  it("should fail on a row longer than the header", async () => {
    const path = await writeTempFile("id,name\n1,a,extra\n")
    const load = async () =>
      (await loadCsvTable({ data: path, fileDialect: { format: "csv" } })).collect()

    await expect(load()).rejects.toThrow()
  })

  it("should pad a short row with nulls", async () => {
    const path = await writeTempFile("id,name\n1\n")
    const table = await loadCsvTable({ data: path, fileDialect: { format: "csv" } })

    expect((await table.collect()).toRecords()).toEqual([{ id: 1, name: null }])
  })

  it("should fail on a repeated header name", async () => {
    const path = await writeTempFile("id,id\n1,2\n")

    await expect(
      loadCsvTable({ data: path, fileDialect: { format: "csv" } }),
    ).rejects.toThrow()
  })

  it("should skip blank lines without numbering them", async () => {
    const path = await writeTempFile("id\n\n1\n\n2\n3\n")
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", commentRows: [3] },
    })

    expect((await table.collect()).toRecords()).toEqual([{ id: 1 }, { id: 3 }])
  })

  it("should keep a quoted nullSequence as text", async () => {
    const path = await writeTempFile('id,name\n1,\\N\n2,"\\N"\n')
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", nullSequence: "\\N" },
      tableSchema: { properties: { id: { type: "integer" }, name: { type: "string" } } },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: null },
      { id: 2, name: "\\N" },
    ])
  })

  it.skip("should load remote file", async () => {
    const table = await loadCsvTable({
      data: "https://raw.githubusercontent.com/datisthq/fairspec-typescript/refs/heads/main/table/plugins/csv/actions/table/-test/fixtures/table.csv",
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "english" },
      { id: 2, name: "中文" },
    ])
  })

  it.skip("should load remote file (multipart)", async () => {
    const table = await loadCsvTable({
      data: [
        "https://raw.githubusercontent.com/datisthq/fairspec-typescript/refs/heads/main/table/plugins/csv/actions/table/-test/fixtures/table.csv",
        "https://raw.githubusercontent.com/datisthq/fairspec-typescript/refs/heads/main/table/plugins/csv/actions/table/-test/fixtures/table.csv",
      ],
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "english" },
      { id: 2, name: "中文" },
      { id: 1, name: "english" },
      { id: 2, name: "中文" },
    ])
  })

  it("should load remote file with previewBytes limit", async () => {
    const table = await loadCsvTable(
      {
        data: "https://raw.githubusercontent.com/datisthq/fairspec-typescript/refs/heads/main/table/plugins/csv/actions/table/-test/fixtures/table.csv",
      },
      { previewBytes: 18 },
    )

    expect((await table.collect()).toRecords()).toEqual([{ id: 1, name: "english" }])
  })

  it("should handle windows line terminator set in format", async () => {
    const path = await writeTempFile("id,name\r\n1,english\r\n2,中文\r\n")
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", lineTerminator: "\r\n" },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "english" },
      { id: 2, name: "中文" },
    ])
  })

  it("should handle custom delimiter", async () => {
    const path = await writeTempFile("id|name\n1|alice\n2|bob")
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", delimiter: "|" },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "alice" },
      { id: 2, name: "bob" },
    ])
  })

  it("should split cells on a multi-byte delimiter", async () => {
    const path = await writeTempFile('id¦name\n1¦"alice¦smith"\n2¦bob\n')
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", delimiter: "¦" },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "alice¦smith" },
      { id: 2, name: "bob" },
    ])
  })

  it("should handle files without header", async () => {
    const path = await writeTempFile("1,alice\n2,bob")
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", headerRows: false },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { column1: 1, column2: "alice" },
      { column1: 2, column2: "bob" },
    ])
  })

  it("should handle files without header using columnNames", async () => {
    const path = await writeTempFile("1,alice\n2,bob")
    const table = await loadCsvTable({
      data: path,
      fileDialect: {
        format: "csv",
        headerRows: false,
        columnNames: ["id", "name"],
      },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { id: 1, name: "alice" },
      { id: 2, name: "bob" },
    ])
  })

  it("should take the default headerRows when the dialect declares only the format", async () => {
    const path = await writeTempFile("1,100\n2,200\n3,300")
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv" },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { "1": 2, "100": 200 },
      { "1": 3, "100": 300 },
    ])
  })

  it("should handle custom line terminator", async () => {
    const path = await writeTempFile("id,name|1,alice|2,bob")
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", lineTerminator: "|" },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "alice" },
      { id: 2, name: "bob" },
    ])
  })

  it("should handle custom quote character", async () => {
    const path = await writeTempFile("id,name\n1,'alice smith'\n2,'bob jones'")

    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", quoteChar: "'" },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "alice smith" },
      { id: 2, name: "bob jones" },
    ])
  })

  it("should handle comment character", async () => {
    const path = await writeTempFile(
      "# This is a comment\nid,name\n1,alice\n# Another comment\n2,bob",
    )

    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", commentPrefix: "#" },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { id: 1, name: "alice" },
      { id: 2, name: "bob" },
    ])
  })

  it("should support headerRows", async () => {
    const path = await writeTempFile("#comment\nid,name\n1,alice\n2,bob")

    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", headerRows: [2] },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { id: 1, name: "alice" },
      { id: 2, name: "bob" },
    ])
  })

  it("should support headerJoin", async () => {
    const path = await writeTempFile("#comment\nid,name\nint,str\n1,alice\n2,bob")

    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", headerRows: [2, 3], headerJoin: "_" },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { id_int: 1, name_str: "alice" },
      { id_int: 2, name_str: "bob" },
    ])
  })

  it("should support commentRows", async () => {
    const path = await writeTempFile("id,name\n1,alice\ncomment\n2,bob")

    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", commentRows: [3] },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { id: 1, name: "alice" },
      { id: 2, name: "bob" },
    ])
  })

  it("should support headerRows and commentRows", async () => {
    const path = await writeTempFile("#comment\nid,name\n1,alice\n#comment\n2,bob")

    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", headerRows: [2], commentRows: [4] },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { id: 1, name: "alice" },
      { id: 2, name: "bob" },
    ])
  })

  it("should number commentRows before commentPrefix removes rows", async () => {
    const path = await writeTempFile("id,name\n#x,y\na,b\nskip,me\nc,d\n")

    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", commentRows: [4], commentPrefix: "#" },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { id: "a", name: "b" },
      { id: "c", name: "d" },
    ])
  })

  it("should support headerJoin and commentRows", async () => {
    const path = await writeTempFile(
      "#comment\nid,name\nint,str\n1,alice\n#comment\n2,bob",
    )

    const table = await loadCsvTable({
      data: path,
      fileDialect: {
        format: "csv",
        headerRows: [2, 3],
        headerJoin: "_",
        commentRows: [5],
      },
    })

    const records = (await table.collect()).toRecords()
    expect(records).toEqual([
      { id_int: 1, name_str: "alice" },
      { id_int: 2, name_str: "bob" },
    ])
  })

  it("should handle null sequence", async () => {
    const path = await writeTempFile("id,name,age\n1,alice,25\n2,N/A,30\n3,bob,N/A")
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "csv", nullSequence: "N/A" },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "alice", age: 25 },
      { id: 2, name: null, age: 30 },
      { id: 3, name: "bob", age: null },
    ])
  })

  it("should handle multiple format options together", async () => {
    const path = await writeTempFile(
      "#comment\nid|'full name'|age\n1|'alice smith'|25\n2|'bob jones'|30",
    )
    const table = await loadCsvTable({
      data: path,
      fileDialect: {
        format: "csv",
        delimiter: "|",
        quoteChar: "'",
        commentPrefix: "#",
        headerRows: [1],
      },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, "full name": "alice smith", age: 25 },
      { id: 2, "full name": "bob jones", age: 30 },
    ])
  })
})

describe("loadCsvTable (format=tsv)", () => {
  it("should load local file", async () => {
    const path = await writeTempFile("id\tname\n1\tenglish\n2\t中文")
    const table = await loadCsvTable({
      data: path,
      fileDialect: { format: "tsv" },
    })

    expect((await table.collect()).toRecords()).toEqual([
      { id: 1, name: "english" },
      { id: 2, name: "中文" },
    ])
  })
})
