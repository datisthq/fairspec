import type { Dataset, FairspecError, ForeignKey } from "@fairspec/metadata"
import { createReport, inferResourceName, resolveTableSchema } from "@fairspec/metadata"
import type { Table } from "@fairspec/table"
import * as pl from "nodejs-polars"
import { loadTable } from "../../actions/table/load.ts"

// TODO: review temporary files creation from validateDataset call

export async function validateDatasetForeignKeys(
  dataset: Dataset,
  options?: { maxErrors?: number },
) {
  const { maxErrors = 1000 } = options ?? {}

  const errors: FairspecError[] = []
  const tables: Record<string, Table> = {}

  const resources = (dataset.resources ?? []).map((resource, index) => ({
    resource,
    name: resource.name ?? inferResourceName(resource, { resourceNumber: index + 1 }),
  }))

  for (const [resourceIndex, { resource, name: resourceName }] of resources.entries()) {
    const tableSchema = await resolveTableSchema(resource.tableSchema)
    if (!tableSchema) continue

    const foreignKeys = tableSchema.foreignKeys
    if (!foreignKeys) continue

    const names = [
      resourceName,
      ...foreignKeys.flatMap(it =>
        it.reference.resource ? [it.reference.resource] : [],
      ),
    ]

    for (const name of names) {
      const resource = resources.find(it => it.name === name)?.resource

      if (!resource) {
        errors.push({
          type: "resource/missing",
          resourceName: name,
          referencingResourceName: resourceName,
        })

        continue
      }

      if (!tables[name]) {
        const table = await loadTable(resource)

        if (!table) {
          errors.push({
            type: "resource/type",
            resourceName: name,
            expectedResourceType: "table",
            referencingResourceName: resourceName,
          })

          continue
        }

        tables[name] = table
      }
    }

    for (const [foreignKeyIndex, foreignKey] of foreignKeys.entries()) {
      const left = tables[resourceName]
      const right = tables[foreignKey.reference.resource ?? resourceName]

      // Missing resource error added in the previous step
      if (!left || !right) {
        continue
      }

      const keyTypes = await getKeyTypes(left, right, foreignKey)
      if (!keyTypes) {
        errors.push({
          type: "metadata",
          message: "foreign key columns and reference columns differ in type",
          jsonPointer: `/resources/${resourceIndex}/tableSchema/foreignKeys/${foreignKeyIndex}`,
        })
        continue
      }

      const keyNames = keyTypes.map((_, index) => `fairspec:key:${index}`)
      const foreignKeyCheckTable = left
        .select(...foreignKey.columns)
        .dropNulls(foreignKey.columns)
        .withColumns(
          ...foreignKey.columns.map((name, index) =>
            castKey(pl.col(name), keyTypes[index]).alias(keyNames[index] ?? name),
          ),
        )
        .join(
          right.select(
            ...foreignKey.reference.columns.map((name, index) =>
              castKey(pl.col(name), keyTypes[index]).alias(keyNames[index] ?? name),
            ),
          ),
          { how: "anti", on: keyNames },
        )
        .select(...foreignKey.columns)

      const foreignKeyCheckFrame = await foreignKeyCheckTable.head(maxErrors).collect()

      for (const row of foreignKeyCheckFrame.toRecords() as any[]) {
        errors.push({
          type: "foreignKey",
          foreignKey,
          cells: Object.values(row).map(String),
          resourceName,
        })
      }
    }
  }

  return createReport(errors, { maxErrors })
}

const NUMERIC_VARIANTS = [
  "Int8",
  "Int16",
  "Int32",
  "Int64",
  "UInt8",
  "UInt16",
  "UInt32",
  "UInt64",
]
const FLOAT_VARIANTS = ["Float32", "Float64"]

/**
 * The type each key pair joins on: numeric columns of differing types compare by
 * value, any other differing pair is undefined, which never joins
 */
async function getKeyTypes(left: Table, right: Table, foreignKey: ForeignKey) {
  const leftSchema = (await left.head(0).collect()).schema
  const rightSchema = (await right.head(0).collect()).schema
  const keyTypes: (pl.DataType | undefined)[] = []

  for (const [index, name] of foreignKey.columns.entries()) {
    const leftType = leftSchema[name]
    const rightType = rightSchema[foreignKey.reference.columns[index] ?? ""]
    if (!leftType || !rightType) return undefined
    if (leftType.variant === "Categorical" || leftType.variant === "Enum") {
      if (rightType.variant !== "Categorical" && rightType.variant !== "Enum")
        return undefined
      keyTypes.push(pl.String)
    } else if (getIsNumeric(leftType) && getIsNumeric(rightType)) {
      if (leftType.equals(rightType)) {
        keyTypes.push(undefined)
      } else if (
        FLOAT_VARIANTS.includes(leftType.variant) ||
        FLOAT_VARIANTS.includes(rightType.variant)
      ) {
        keyTypes.push(pl.Float64)
      } else {
        keyTypes.push(pl.Decimal(38, 18))
      }
    } else if (leftType.equals(rightType)) {
      keyTypes.push(undefined)
    } else {
      return undefined
    }
  }

  return keyTypes
}

function getIsNumeric(type: pl.DataType) {
  return [...NUMERIC_VARIANTS, ...FLOAT_VARIANTS, "Decimal"].includes(type.variant)
}

function castKey(columnExpr: pl.Expr, type?: pl.DataType) {
  return type ? columnExpr.cast(type) : columnExpr
}
