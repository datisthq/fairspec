import type { ColumnProperty, Dataset } from "@fairspec/metadata"
import { describe, expect, it } from "vite-plus/test"
import { validateDatasetForeignKeys } from "../foreignKey.ts"

describe("validateDatasetForeignKeys", () => {
  it("should validate dataset with valid foreign keys", async () => {
    const source: Dataset = {
      resources: [
        {
          name: "users",
          data: [
            { id: 1, name: "Alice" },
            { id: 2, name: "Bob" },
          ],
          tableSchema: {
            properties: {
              id: { type: "integer" },
              name: { type: "string" },
            },
          },
        },
        {
          name: "posts",
          data: [
            { id: 1, user_id: 1, title: "Post 1" },
            { id: 2, user_id: 2, title: "Post 2" },
          ],
          tableSchema: {
            properties: {
              id: { type: "integer" },
              user_id: { type: "integer" },
              title: { type: "string" },
            },
            foreignKeys: [
              {
                columns: ["user_id"],
                reference: {
                  resource: "users",
                  columns: ["id"],
                },
              },
            ],
          },
        },
      ],
    }

    const report = await validateDatasetForeignKeys(source)

    expect(report.valid).toBe(true)
    expect(report.errors).toHaveLength(0)
  })

  it("should detect foreign key violations", async () => {
    const source: Dataset = {
      resources: [
        {
          name: "users",
          data: [
            { id: 1, name: "Alice" },
            { id: 2, name: "Bob" },
          ],
          tableSchema: {
            properties: {
              id: { type: "integer" },
              name: { type: "string" },
            },
          },
        },
        {
          name: "posts",
          data: [
            { id: 1, user_id: 1, title: "Post 1" },
            { id: 2, user_id: 999, title: "Post 2" },
          ],
          tableSchema: {
            properties: {
              id: { type: "integer" },
              user_id: { type: "integer" },
              title: { type: "string" },
            },
            foreignKeys: [
              {
                columns: ["user_id"],
                reference: {
                  resource: "users",
                  columns: ["id"],
                },
              },
            ],
          },
        },
      ],
    }

    const report = await validateDatasetForeignKeys(source)

    expect(report.valid).toBe(false)
    expect(report.errors).toHaveLength(1)
    expect(report.errors).toContainEqual({
      type: "foreignKey",
      foreignKey: {
        columns: ["user_id"],
        reference: {
          resource: "users",
          columns: ["id"],
        },
      },
      cells: ["999"],
      resourceName: "posts",
    })
  })

  it("should handle self-referencing foreign keys", async () => {
    const source: Dataset = {
      resources: [
        {
          name: "categories",
          data: [
            { id: 1, parent_id: 1, name: "Root" },
            { id: 2, parent_id: 1, name: "Child 1" },
            { id: 3, parent_id: 2, name: "Child 2" },
          ],
          tableSchema: {
            properties: {
              id: { type: "integer" },
              parent_id: { type: "integer" },
              name: { type: "string" },
            },
            foreignKeys: [
              {
                columns: ["parent_id"],
                reference: {
                  columns: ["id"],
                },
              },
            ],
          },
        },
      ],
    }

    const report = await validateDatasetForeignKeys(source)

    expect(report.valid).toBe(true)
    expect(report.errors).toHaveLength(0)
  })

  it("should detect missing referenced resource", async () => {
    const source: Dataset = {
      resources: [
        {
          name: "posts",
          data: [{ id: 1, user_id: 1, title: "Post 1" }],
          tableSchema: {
            properties: {
              id: { type: "integer" },
              user_id: { type: "integer" },
              title: { type: "string" },
            },
            foreignKeys: [
              {
                columns: ["user_id"],
                reference: {
                  resource: "users",
                  columns: ["id"],
                },
              },
            ],
          },
        },
      ],
    }

    const report = await validateDatasetForeignKeys(source)

    expect(report.valid).toBe(false)
    expect(report.errors).toHaveLength(1)
    expect(report.errors).toContainEqual({
      type: "resource/missing",
      resourceName: "users",
      referencingResourceName: "posts",
    })
  })

  it("should skip a key with any null part", async () => {
    const source: Dataset = {
      resources: [
        {
          name: "tree",
          data: [
            { id: 1, sub: 1, parent_id: null, parent_sub: 1 },
            { id: 2, sub: 1, parent_id: 1, parent_sub: null },
            { id: 3, sub: 1, parent_id: 9, parent_sub: 9 },
          ],
          tableSchema: {
            properties: {
              id: { type: "integer" },
              sub: { type: "integer" },
              parent_id: { type: ["integer", "null"] },
              parent_sub: { type: ["integer", "null"] },
            },
            foreignKeys: [
              {
                columns: ["parent_id", "parent_sub"],
                reference: { columns: ["id", "sub"] },
              },
            ],
          },
        },
      ],
    }

    const report = await validateDatasetForeignKeys(source)

    expect(report.errors).toEqual([
      expect.objectContaining({ type: "foreignKey", cells: ["9", "9"] }),
    ])
  })

  it("should compare numeric columns of differing types by value", async () => {
    const report = await validateDatasetForeignKeys(
      createDataset([{ id: 1.0 }, { id: 2.5 }], { type: "number" }),
    )

    expect(report.errors.map(error => "cells" in error && error.cells)).toEqual([["2"]])
  })

  it("should report differing column types as a metadata error", async () => {
    const report = await validateDatasetForeignKeys(
      createDataset([{ id: "1" }, { id: "2" }], { type: "string" }),
    )

    expect(report.errors).toEqual([
      {
        type: "metadata",
        message: "foreign key columns and reference columns differ in type",
        jsonPointer: "/resources/1/tableSchema/foreignKeys/0",
      },
    ])
  })
})

function createDataset(
  referencedData: Record<string, unknown>[],
  referencedProperty: ColumnProperty,
): Dataset {
  return {
    resources: [
      {
        name: "users",
        data: referencedData,
        tableSchema: { properties: { id: referencedProperty } },
      },
      {
        name: "posts",
        data: [{ user_id: 1 }, { user_id: 2 }],
        tableSchema: {
          properties: { user_id: { type: "integer" } },
          foreignKeys: [
            { columns: ["user_id"], reference: { resource: "users", columns: ["id"] } },
          ],
        },
      },
    ],
  }
}
