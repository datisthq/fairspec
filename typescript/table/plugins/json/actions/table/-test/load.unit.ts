import * as pl from "nodejs-polars"
import { writeTempFile } from "@fairspec/dataset"
import { describe, expect, it } from "vite-plus/test"
import { useRecording } from "@fairspec/metadata/testing"
import { loadJsonTable } from "../load.ts"

useRecording()

describe("loadJsonTable", () => {
  describe("file variations", () => {
    it("should load local file", async () => {
      const body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "json" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it("should read an array of objects natively", async () => {
      const path = await writeTempFile('[{"id":1},{"id":2}]', { format: "json" })

      const table = await loadJsonTable({ data: path }, { denormalized: true })

      expect((await table?.collect())?.getColumn("id").dtype).toEqual(pl.Int64)
    })

    it("should fall back when the native reader refuses the rows", async () => {
      const path = await writeTempFile('[{"id":1},5,{"id":2}]')

      const table = await loadJsonTable({ data: path, fileDialect: { format: "json" } })

      expect((await table?.collect())?.toRecords()).toEqual([{ id: 1 }, { id: 2 }])
    })

    it("should load local file (multipart)", async () => {
      const body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
      const path1 = await writeTempFile(body)
      const path2 = await writeTempFile(body)

      const table = await loadJsonTable({
        data: [path1, path2],
        fileDialect: { format: "json" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it.skip("should load remote file", async () => {
      const table = await loadJsonTable({
        data: "https://github.com/datisthq/fairspec-typescript/raw/refs/heads/main/table/plugins/json/table/fixtures/table.json",
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it.skip("should load remote file (multipart)", async () => {
      const table = await loadJsonTable({
        data: [
          "https://github.com/datisthq/fairspec-typescript/raw/refs/heads/main/table/plugins/json/table/fixtures/table.json",
          "https://github.com/datisthq/fairspec-typescript/raw/refs/heads/main/table/plugins/json/table/fixtures/table.json",
        ],
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })
  })

  describe("format variations", () => {
    it("should handle property", async () => {
      const body = '{"key": [{"id":1,"name":"english"},{"id":2,"name":"中文"}]}'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "json", jsonPointer: "/key" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it("should handle a nested and escaped jsonPointer", async () => {
      const body = '{"meta":{},"a/b":{"c~d":[{"id":1,"name":"english"}]}}'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "json", jsonPointer: "/a~1b/c~0d" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([{ id: 1, name: "english" }])
    })

    it("should take columnNames for an empty array", async () => {
      const path = await writeTempFile('{"items":[]}')

      const table = await loadJsonTable({
        data: path,
        fileDialect: {
          format: "json",
          jsonPointer: "/items",
          rowType: "object",
          columnNames: ["id", "name"],
        },
      })

      const frame = await table?.collect()
      expect(frame?.columns).toEqual(["id", "name"])
      expect(frame?.height).toEqual(0)
    })

    it("should not load a top-level object as a table", async () => {
      const path = await writeTempFile('{"id":1,"name":"english"}')

      const table = await loadJsonTable({ data: path, fileDialect: { format: "json" } })

      expect(table).toBeUndefined()
    })

    it("should handle item type (array) without a header row", async () => {
      const body = '[[1,"english"],[2,"中文"]]'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: {
          format: "json",
          rowType: "array",
          headerRows: false,
          columnNames: ["id", "name"],
        },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it("should handle item keys", async () => {
      const body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "json", columnNames: ["name"] },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { name: "english" },
        { name: "中文" },
      ])
    })

    it("should handle item type (array)", async () => {
      const body = '[["id","name"],[1,"english"],[2,"中文"]]'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "json", rowType: "array" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it("should load item type (object)", async () => {
      const body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "json", rowType: "object" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })
  })
})

describe("loadJsonTable (format=jsonl)", () => {
  describe("file variations", () => {
    it("should load local file", async () => {
      const body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "jsonl" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it("should scan a default dialect natively", async () => {
      const path = await writeTempFile('{"id":1}\n{"id":2}')

      const table = await loadJsonTable(
        { data: path, fileDialect: { format: "jsonl" } },
        { denormalized: true },
      )

      expect((await table?.collect())?.getColumn("id").dtype).toEqual(pl.Int64)
    })

    it("should scan an inferred object-row dialect natively", async () => {
      const path = await writeTempFile('{"id":1}\n{"id":2}', { format: "jsonl" })

      const table = await loadJsonTable({ data: path }, { denormalized: true })

      expect((await table?.collect())?.getColumn("id").dtype).toEqual(pl.Int64)
    })

    it("should load local file (multipart)", async () => {
      const body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
      const path1 = await writeTempFile(body)
      const path2 = await writeTempFile(body)

      const table = await loadJsonTable({
        data: [path1, path2],
        fileDialect: { format: "jsonl" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it.skip("should load remote file", async () => {
      const table = await loadJsonTable({
        data: "https://github.com/datisthq/fairspec-typescript/raw/refs/heads/main/table/plugins/json/table/fixtures/table.jsonl",
        fileDialect: { format: "jsonl" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it.skip("should load remote file (multipart)", async () => {
      const table = await loadJsonTable({
        data: [
          "https://github.com/datisthq/fairspec-typescript/raw/refs/heads/main/table/plugins/json/table/fixtures/table.jsonl",
          "https://github.com/datisthq/fairspec-typescript/raw/refs/heads/main/table/plugins/json/table/fixtures/table.jsonl",
        ],
        fileDialect: { format: "jsonl" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })
  })

  describe("format variations", () => {
    it("should handle item keys", async () => {
      const body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "jsonl", columnNames: ["name"] },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { name: "english" },
        { name: "中文" },
      ])
    })

    it("should handle item type (array)", async () => {
      const body = '["id","name"]\n[1,"english"]\n[2,"中文"]'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "jsonl", rowType: "array" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })

    it("should read array rows when rowType is absent", async () => {
      const body = '["id","name"]\n[1,"english"]'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({ data: path, fileDialect: { format: "jsonl" } })

      expect((await table?.collect())?.toRecords()).toEqual([{ id: 1, name: "english" }])
    })

    it("should load item type (object)", async () => {
      const body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
      const path = await writeTempFile(body)

      const table = await loadJsonTable({
        data: path,
        fileDialect: { format: "jsonl", rowType: "object" },
      })

      expect((await table?.collect())?.toRecords()).toEqual([
        { id: 1, name: "english" },
        { id: 2, name: "中文" },
      ])
    })
  })
})
