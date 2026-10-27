---
title: Working with Tables in TypeScript
label: Table
path: /typescript/library/table/
order: 2
---

Load, save, validate, describe, query, and reshape tabular data across every supported format. Tables are backed by **nodejs-polars** (a Rust-based DataFrame library), so loaded values are lazy and high-throughput.

## Installation

```bash
npm install fairspec
```

## Getting Started

The table API is split into two layers — format-agnostic I/O and in-memory manipulation:

- `loadTable`, `saveTable`, `validateTable` — read, write, and validate tables across CSV, TSV, JSON, JSONL, XLSX, ODS, Arrow, Parquet, SQLite, and inline data
- `queryTable`, `normalizeTable`, `denormalizeTable`, `inspectTable`, `inferTableSchemaFromTable` — operate on a loaded `Table` value

```ts
import { loadTable } from "fairspec"

const table = await loadTable({ data: "data.csv" })
const frame = await table?.collect()
console.log(frame?.toRecords())
```

## Loading a Table

`loadTable(resource, options?)` dispatches to the first plugin that recognises the resource. Format detection is automatic when `fileDialect` is omitted.

```ts
import { loadTable } from "fairspec"

const csv = await loadTable({ data: "data.csv" })

const remote = await loadTable({ data: "https://example.com/data.csv" })

const concat = await loadTable({ data: ["part1.csv", "part2.csv"] })

const xlsx = await loadTable({
  data: "report.xlsx",
  fileDialect: { sheetName: "Q1 Sales" },
})
```

The returned value is a `Table` — an alias for `pl.LazyDataFrame`. Call `.collect()` to materialise it.

```ts
import { loadTable } from "fairspec"

const table = await loadTable({ data: "data.csv" })
if (!table) throw new Error("No plugin matched the resource")

const frame = await table.head(10).collect()
console.log(frame.toRecords())
```

Pass `denormalized: true` to skip schema normalization and receive raw column types.

```ts
import { loadTable } from "fairspec"

const raw = await loadTable({ data: "data.csv" }, { denormalized: true })
```

## Saving a Table

`saveTable(table, options)` writes a table to a path. The plugin is chosen by the file extension on `options.path` unless `fileDialect` is provided explicitly.

```ts
import { loadTable, saveTable } from "fairspec"

const table = await loadTable({ data: "data.csv" })
if (!table) throw new Error("Could not load table")

await saveTable(table, { path: "output.csv" })

await saveTable(table, {
  path: "output.parquet",
  overwrite: true,
})

await saveTable(table, {
  path: "output.tsv",
  fileDialect: { name: "tsv" },
})
```

This is the simplest way to convert between formats — load with one extension, save with another.

## Validating a Table

`validateTable(resource, options?)` checks that the table data matches its schema. If the resource has no embedded `tableSchema`, one is inferred from a sample.

```ts
import { validateTable } from "fairspec"

const report = await validateTable({
  data: "data.csv",
  tableSchema: "schema.json",
})

if (!report.valid) {
  for (const error of report.errors) {
    console.error(error.type, error)
  }
}
```

To use only an embedded schema and fail when none is present, pass `noInfer: true`:

```ts
import { validateTable } from "fairspec"

const report = await validateTable(
  { data: "data.csv", tableSchema: "schema.json" },
  { noInfer: true, maxErrors: 100 },
)
```

Inline schemas work the same way:

```ts
import type { TableSchema } from "fairspec"
import { validateTable } from "fairspec"

const tableSchema: TableSchema = {
  properties: {
    id: { type: "integer" },
    email: { type: "string", pattern: "^[^@]+@[^@]+\\.[^@]+$" },
    age: { type: "integer", minimum: 0, maximum: 150 },
  },
  required: ["id", "email"],
}

const report = await validateTable({ data: "users.csv", tableSchema })
```

### Custom Checks

`checks` adds rules a Table Schema cannot express. Each check has a `name` and a `condition`: a [Polars SQL](https://docs.pola.rs/api/python/stable/reference/sql/index.html) condition or a nodejs-polars expression. A row fails when its condition is `false`; `null` passes, as in a SQL `CHECK` constraint. Conditions see the values typed by the schema.

```ts
import * as pl from "nodejs-polars"
import { validateTable } from "fairspec"

const report = await validateTable(
  { data: "orders.csv" },
  {
    checks: [
      { name: "end-after-start", condition: '"end" >= start' },
      { name: "positive-price", condition: pl.col("price").gt(0), columnName: "price" },
      {
        name: "repeat-customer",
        condition: "count(*) OVER (PARTITION BY customer) >= 2",
      },
      { name: "not-empty", condition: "count(*) > 0", message: "The export is empty" },
    ],
  },
)
```

- A condition that reduces to one value, such as `count(*) > 0`, checks the whole table and reports `table/check`
- Any other condition reports `row/check` for each failing row, or `cell/check` with the cell value when the check names a `columnName`
- A window (`OVER (PARTITION BY …)` in SQL, `.over(…)` in Polars) checks each row against its group
- `message` is carried on every error the check reports
- A check that cannot run (an unknown column, a syntax error, a condition that is not boolean) reports `check`, and the other checks still run
- Quote column names that are SQL keywords: `"end" >= start`

## Describing a Table

A loaded `Table` is a Polars `LazyDataFrame`, so describe-style summaries use Polars' `.describe()` directly after collecting.

```ts
import { loadTable } from "fairspec"

const table = await loadTable({ data: "data.csv" })
if (!table) throw new Error("Could not load table")

const frame = await table.collect()
const stats = frame.describe()
console.log(stats.toString())
```

For row counts and column names:

```ts
import { loadTable } from "fairspec"

const table = await loadTable({ data: "data.csv" })
const frame = await table?.collect()
console.log(frame?.height, "rows")
console.log(frame?.columns)
console.log(frame?.schema)
```

## Querying a Table

`queryTable(table, sql)` runs a SQL query against the loaded table using Polars' SQL engine. The table is exposed as `self` inside the query. An `ORDER BY` item that names no null order sorts nulls last, in ascending and descending order alike, as the [Table Schema](https://fairspec.org/specs/table-schema/#sorting) sets.

```ts
import { loadTable, queryTable } from "fairspec"

const table = await loadTable({ data: "sales.csv" })
if (!table) throw new Error("Could not load table")

const result = queryTable(
  table,
  "SELECT region, SUM(amount) AS total FROM self GROUP BY region ORDER BY total DESC",
)

const frame = await result.collect()
console.log(frame.toRecords())
```

Joins, window functions, CTEs — anything Polars SQL supports — work in the same way:

```ts
import { loadTable, queryTable } from "fairspec"

const orders = await loadTable({ data: "orders.csv" })
if (!orders) throw new Error("Could not load orders")

const top = await queryTable(
  orders,
  `SELECT customer, SUM(amount) AS total
   FROM self
   GROUP BY customer
   HAVING SUM(amount) > 1000
   ORDER BY total DESC
   LIMIT 10`,
).collect()
```

## Schema Inference

`inferTableSchemaFromTable` infers a Table Schema from a loaded `Table`. It samples rows and detects types, required columns, and enum values.

```ts
import * as pl from "nodejs-polars"
import { inferTableSchemaFromTable } from "fairspec"

const table = pl
  .DataFrame({
    id: ["1", "2", "3"],
    price: ["10.50", "25.00", "15.75"],
    date: ["2023-01-15", "2023-02-20", "2023-03-25"],
    active: ["true", "false", "true"],
  })
  .lazy()

const schema = await inferTableSchemaFromTable(table, {
  sampleRows: 100,
  confidence: 0.9,
})
```

Customise inference with `keepStrings`, per-column overrides, or value-parsing options:

```ts
import { inferTableSchemaFromTable } from "fairspec"

const schema = await inferTableSchemaFromTable(table, {
  sampleRows: 100,
  confidence: 0.9,
  keepStrings: false,
  columnTypes: {
    id: "integer",
  },
})
```

`columnProperties` gives part of the schema up front and infers the rest. A property with a `type`
replaces the inferred one as written; one without a `type` only adds `title`, `description` or
`rdfType` to the inferred property, so give the `type` to add constraints. It takes precedence over `columnTypes`, and naming a column the
table lacks throws. A resource's own `tableSchema` still replaces inference entirely.

```ts
const schema = await inferTableSchemaFromTable(table, {
  columnProperties: {
    id: { type: "string" },
    status: { type: "string", format: "categorical", categories: ["open", "closed"] },
    price: { title: "Price" },
  },
})
```

## Table Normalization

`normalizeTable(table, schema)` converts string columns into typed columns according to a schema — useful when a loader returned everything as strings.

```ts
import * as pl from "nodejs-polars"
import type { TableSchema } from "fairspec"
import { normalizeTable } from "fairspec"

const table = pl
  .DataFrame({
    id: ["1", "2", "3"],
    price: ["10.50", "25.00", "15.75"],
    active: ["true", "false", "true"],
    date: ["2023-01-15", "2023-02-20", "2023-03-25"],
  })
  .lazy()

const schema: TableSchema = {
  properties: {
    id: { type: "integer" },
    price: { type: "number" },
    active: { type: "boolean" },
    date: { type: "date" },
  },
}

const normalized = await normalizeTable(table, schema)
const result = await normalized.collect()
```

`denormalizeTable(table, schema, options?)` is the inverse — convert typed columns back to a writable form (dates to strings, etc.) for export.

```ts
import { denormalizeTable } from "fairspec"

const writable = await denormalizeTable(table, schema, {
  nativeTypes: ["string", "number", "boolean"],
})
```

## Table Schema

Define schemas as plain JSON-compatible objects. Column types match the Fairspec Table Schema specification.

```ts
import type { TableSchema } from "fairspec"

const schema: TableSchema = {
  properties: {
    id: {
      type: "integer",
      minimum: 1,
    },
    name: {
      type: "string",
      minLength: 1,
      maxLength: 100,
    },
    email: {
      type: "string",
      pattern: "^[^@]+@[^@]+\\.[^@]+$",
    },
    age: {
      type: "integer",
      minimum: 0,
      maximum: 150,
    },
    status: {
      type: "string",
      enum: ["active", "inactive", "pending"],
    },
  },
  required: ["id", "name", "email"],
  primaryKey: ["id"],
  uniqueKeys: [["email"]],
  missingValues: ["", "N/A", "null"],
}
```

## Supported Column Types

### Primitive Types

- `string` - Text data
- `integer` - Whole numbers
- `number` - Decimal numbers
- `boolean` - True/false values

### Temporal Types

- `date` - Calendar dates
- `date-time` - Instants: a date and time with a UTC offset, read as UTC
- `local-date-time` - A date and time without an offset, in no time zone
- `time` - Time of day with a UTC offset, read as UTC
- `local-time` - Time of day without an offset
- `duration` - Time spans

### Spatial Types

- `geojson` - GeoJSON geometries
- `wkt` - Well-Known Text geometries
- `wkb` - Well-Known Binary geometries

### Complex Types

- `array` - Fixed-length arrays
- `list` - Variable-length lists
- `object` - JSON objects

### Specialized Types

- `email` - Email addresses
- `url` - URLs
- `categorical` - Categorical data
- `base64` - Base64 encoded data
- `hex` - Hexadecimal data

## Table Type

The `Table` type is an alias for `pl.LazyDataFrame` from nodejs-polars, so any Polars lazy operation is available.

```ts
import type { Table } from "fairspec"
import * as pl from "nodejs-polars"

const table: Table = pl.DataFrame({ id: [1, 2, 3] }).lazy()
const filtered = await table.filter(pl.col("id").gt(1)).collect()
```

## Common Workflows

### Convert between formats

```ts
import { loadTable, saveTable } from "fairspec"

const table = await loadTable({ data: "data.csv" })
if (!table) throw new Error("Could not load table")

await saveTable(table, { path: "data.parquet" })
```

### Validate before saving

```ts
import { loadTable, saveTable, validateTable } from "fairspec"

const resource = { data: "input.csv", tableSchema: "schema.json" }

const report = await validateTable(resource)
if (!report.valid) {
  console.error(report.errors)
  process.exit(1)
}

const table = await loadTable(resource)
if (!table) throw new Error("Could not load table")

await saveTable(table, { path: "validated.parquet" })
```

### Infer schema and validate against it

```ts
import { inferTableSchemaFromTable, loadTable, validateTable } from "fairspec"

const sample = await loadTable({ data: "sample.csv" })
if (!sample) throw new Error("Could not load sample")

const tableSchema = await inferTableSchemaFromTable(sample, {
  sampleRows: 1000,
})

const report = await validateTable({ data: "production.csv", tableSchema })
console.log(report.valid)
```

## Examples

### Top customers report

```ts
import { loadTable, queryTable } from "fairspec"

const sales = await loadTable({ data: "sales.csv" })
if (!sales) throw new Error("Could not load sales")

const top = await queryTable(
  sales,
  `SELECT customer, SUM(amount) AS total
   FROM self
   GROUP BY customer
   ORDER BY total DESC
   LIMIT 10`,
).collect()

console.table(top.toRecords())
```

### Round-trip with schema validation

```ts
import { loadTable, saveTable, validateTable } from "fairspec"

const resource = { data: "users.csv", tableSchema: "users.schema.json" }

const before = await validateTable(resource)
if (!before.valid) throw new Error("Source data invalid")

const table = await loadTable(resource)
if (!table) throw new Error("Could not load")

await saveTable(table, {
  path: "users.parquet",
  tableSchema: resource.tableSchema,
})

const after = await validateTable({
  data: "users.parquet",
  tableSchema: resource.tableSchema,
})
console.log("Round-trip valid:", after.valid)
```
