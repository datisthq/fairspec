import type { TableSchema } from "@fairspec/metadata"
import * as pl from "nodejs-polars"
import { describe, expect, it } from "vite-plus/test"
import { inspectTable } from "../../table/inspect.ts"
import { inspectTableChecks } from "../inspect.ts"

const table = pl
  .DataFrame({
    id: [1, 2, 3],
    zip: ["a", "a", "b"],
    start: [1, 3, 1],
    end: [2, 2, 5],
    price: [10, null, -1],
  })
  .lazy()

describe("inspectTableChecks", () => {
  it("should report rows failing a SQL condition", async () => {
    const errors = await inspectTableChecks(
      table,
      [{ name: "end-after-start", condition: '"end" >= start' }],
      { maxErrors: 100 },
    )

    expect(errors).toEqual([
      { type: "row/check", checkName: "end-after-start", rowNumber: 2 },
    ])
  })

  it("should report rows failing a polars condition", async () => {
    const errors = await inspectTableChecks(
      table,
      [{ name: "end-after-start", condition: pl.col("end").gtEq(pl.col("start")) }],
      { maxErrors: 100 },
    )

    expect(errors).toEqual([
      { type: "row/check", checkName: "end-after-start", rowNumber: 2 },
    ])
  })

  it("should pass rows where the condition is null", async () => {
    const errors = await inspectTableChecks(
      table,
      [{ name: "positive-price", condition: "price > 0" }],
      { maxErrors: 100 },
    )

    expect(errors).toEqual([
      { type: "row/check", checkName: "positive-price", rowNumber: 3 },
    ])
  })

  it("should report a cell when the check names a column", async () => {
    const errors = await inspectTableChecks(
      table,
      [
        {
          name: "positive-price",
          condition: pl.col("price").gt(0),
          columnName: "price",
          message: "Price must be positive",
        },
      ],
      { maxErrors: 100 },
    )

    expect(errors).toEqual([
      {
        type: "cell/check",
        checkName: "positive-price",
        columnName: "price",
        rowNumber: 3,
        cell: "-1",
        message: "Price must be positive",
      },
    ])
  })

  it("should report the table when an aggregate condition fails", async () => {
    const errors = await inspectTableChecks(
      table,
      [
        { name: "sql-size", condition: "count(*) >= 5" },
        { name: "polars-size", condition: pl.len().gtEq(5) },
        { name: "sql-sum", condition: "sum(price) > 0" },
        { name: "polars-sum", condition: pl.col("price").sum().gt(0) },
      ],
      { maxErrors: 100 },
    )

    expect(errors).toEqual([
      { type: "table/check", checkName: "sql-size" },
      { type: "table/check", checkName: "polars-size" },
    ])
  })

  it("should check rows against their group with a window", async () => {
    const errors = await inspectTableChecks(
      table,
      [
        { name: "sql-zip", condition: "count(*) OVER (PARTITION BY zip) >= 2" },
        { name: "polars-zip", condition: pl.len().over("zip").gtEq(2) },
      ],
      { maxErrors: 100 },
    )

    expect(errors).toEqual([
      { type: "row/check", checkName: "sql-zip", rowNumber: 3 },
      { type: "row/check", checkName: "polars-zip", rowNumber: 3 },
    ])
  })

  it("should report checks that cannot be evaluated", async () => {
    const errors = await inspectTableChecks(
      table,
      [
        { name: "sql-column", condition: "missing > 1" },
        { name: "polars-column", condition: pl.col("missing").gt(1) },
        { name: "sql-syntax", condition: "price >" },
        { name: "not-boolean", condition: "price + 1" },
        { name: "cell-column", condition: "price > 0", columnName: "missing" },
        { name: "valid", condition: "price > 0" },
      ],
      { maxErrors: 100 },
    )

    expect(
      errors.map(error => [error.type, "checkName" in error && error.checkName]),
    ).toEqual([
      ["check", "sql-column"],
      ["check", "polars-column"],
      ["check", "sql-syntax"],
      ["check", "not-boolean"],
      ["check", "cell-column"],
      ["row/check", "valid"],
    ])
    expect(errors[3]).toEqual({
      type: "check",
      checkName: "not-boolean",
      message: "Condition is Float64 instead of Boolean",
    })
    expect(errors[4]).toEqual({
      type: "check",
      checkName: "cell-column",
      message: 'Column "missing" is not found',
    })
  })

  it("should stop at the maximum number of failing rows", async () => {
    const errors = await inspectTableChecks(
      table,
      [{ name: "never", condition: pl.lit(false).and(pl.col("id").isNotNull()) }],
      { maxErrors: 2 },
    )

    expect(errors).toEqual([
      { type: "row/check", checkName: "never", rowNumber: 1 },
      { type: "row/check", checkName: "never", rowNumber: 2 },
    ])
  })
})

describe("inspectTable with checks", () => {
  it("should run conditions on values typed by the schema", async () => {
    const table = pl.DataFrame({ id: ["1", "2", "3"], amount: ["9", "10", "bad"] }).lazy()

    const tableSchema: TableSchema = {
      properties: {
        id: { type: "integer" },
        amount: { type: "integer" },
      },
    }

    const errors = await inspectTable(table, {
      tableSchema,
      checks: [{ name: "small-amount", condition: "amount < 10", columnName: "amount" }],
    })

    expect(errors).toEqual([
      {
        type: "cell/type",
        columnName: "amount",
        columnType: "integer",
        rowNumber: 3,
        cell: "bad",
      },
      {
        type: "cell/check",
        checkName: "small-amount",
        columnName: "amount",
        rowNumber: 2,
        cell: "10",
      },
    ])
  })

  it("should run checks without a schema", async () => {
    const errors = await inspectTable(table, {
      checks: [{ name: "end-after-start", condition: '"end" >= start' }],
    })

    expect(errors).toEqual([
      { type: "row/check", checkName: "end-after-start", rowNumber: 2 },
    ])
  })
})
