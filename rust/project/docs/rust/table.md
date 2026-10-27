---
title: Working with Tables in Rust
label: Table
path: /rust/library/table/
order: 2
---

Load, save, query, and validate tabular data in Rust. Tables are represented as **Polars** `LazyFrame` values for high-performance processing.

## Available Functions

The table API provides utilities for working with tabular data:

- `load_table` - Load a table from a `Resource` (format-agnostic)
- `save_table` - Save a table to disk in a specified format
- `infer_table` - Load a table, inferring its missing dialect and schema
- `validate_table` - Validate table data against a Table Schema
- `query_table` - Run a SQL query against a loaded table
- `inspect_table` - Validate an in-memory table and collect errors
- `infer_table_schema` - Infer a Table Schema from a resource
- `infer_table_schema_from_table` - Infer a Table Schema from an in-memory table
- `render_table_schema_as` - Render a Table Schema through the render plugins
- `normalize_table` / `denormalize_table` - Convert between typed and string representations

`load_table`, `save_table`, `infer_table`, `validate_table`, `infer_table_schema` and `render_table_schema_as` live in `fairspec`; the rest, and the per-format loaders, in `fairspec::fairspec_table`.

## Loading a Table

Load a table from a `Resource`. The format is detected from the file extension or dialect, and dispatched to the appropriate plugin:

```rust
use fairspec::fairspec_table::LoadTableOptions;
use fairspec::load_table;
use serde_json::json;

let options = LoadTableOptions::default();

// Load a local CSV
let resource = serde_json::from_value(json!({"data": "data.csv"}))?;
let table = load_table(&resource, &options)?;

// Load a remote file
let resource = serde_json::from_value(json!({"data": "https://example.com/data.csv"}))?;
let table = load_table(&resource, &options)?;

// Load with an explicit dialect
let resource = serde_json::from_value(json!({
    "data": "data.csv",
    "fileDialect": {"format": "csv", "delimiter": ";"},
}))?;
let table = load_table(&resource, &options)?;
```

`load_table` returns `Option<Table>`: `None` means no plugin recognised the resource's format. `Table` is an alias for `polars::prelude::LazyFrame`, so call `.collect()` to materialise it:

```rust
use fairspec::fairspec_table::LoadTableOptions;
use fairspec::load_table;

if let Some(table) = load_table(&resource, &LoadTableOptions::default())? {
    let df = table.collect()?;
    println!("{df}");
}
```

Format-specific loaders are available when you want to bypass detection: `load_csv_table`, `load_json_table`, `load_xlsx_table`, `load_arrow_table`, `load_parquet_table`, `load_sqlite_table`, `load_inline_table`. See the per-format guides for their options.

## Saving a Table

Save a table to disk. The `path` is required; the `file_dialect` controls the output format:

```rust
use fairspec::fairspec_table::SaveTableOptions;
use fairspec::save_table;
use serde_json::json;

save_table(table.clone(), &SaveTableOptions {
    path: "output.csv".into(),
    ..Default::default()
})?;

// Custom dialect, overwriting the existing file
save_table(table, &SaveTableOptions {
    path: "output.csv".into(),
    file_dialect: Some(serde_json::from_value(json!({"format": "csv", "delimiter": "\t"}))?),
    overwrite: Some(true),
    ..Default::default()
})?;
```

`SaveTableOptions`:

- `path` (required) — output path
- `file_dialect` — output dialect (auto-detected from path if omitted)
- `table_schema` — optional Table Schema (drives type-aware serialisation)
- `overwrite` — replace an existing file instead of erroring

A save takes the table by value; `clone()` a `LazyFrame` to save it more than once — it only copies the query plan.

## Inferring a Table

`infer_table` loads a table, inferring any dialect and schema the resource leaves out:

```rust
use fairspec::fairspec_table::{InferTableSchemaOptions, LoadTableOptions};
use fairspec::{infer_resource, infer_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "data.csv"}))?;
let table = infer_table(&resource, &LoadTableOptions::default())?;

// The resource is borrowed, not filled in: infer_resource returns it with metadata
let resource = infer_resource(&resource, None, &InferTableSchemaOptions::default())?;
println!("{:?}", resource.file_dialect);
println!("{:?}", resource.table_schema);
```

Python's `infer_table` fills the resource in place; in Rust, `infer_resource` returns the resource with its `name`, `file_dialect`, `table_schema` and `integrity` populated.

## Validating a Table

Validate table data against a Table Schema:

```rust
use fairspec::fairspec_metadata::{PathOr, Resource};
use fairspec::{ValidateTableOptions, validate_table};
use serde_json::json;

// Validate with an inferred schema
let mut resource: Resource = serde_json::from_value(json!({"data": "data.csv"}))?;
let report = validate_table(&resource, &ValidateTableOptions::default())?;

// Validate against a provided schema
resource.table_schema = Some(PathOr::Value(schema));
let report = validate_table(&resource, &ValidateTableOptions::default())?;

if !report.valid {
    for error in &report.errors {
        let error = serde_json::to_value(error)?;
        println!("row {}: {}", error["rowNumber"], error["type"]);
    }
}
```

The returned `Report` has `valid: bool` and `errors: Vec<FairspecError>`. `FairspecError` is an enum over the error families; serialise it with serde to read its wire fields: `type` (for example `cell/type`, `row/primary-key`, `column/missing`), `rowNumber`, `columnName`, `cell`. Error messages from metadata validation are jsonschema-rs wording, not Python's.

### Custom Checks

`checks` adds rules a Table Schema cannot express. Each check has a `name` and a `condition`: a [Polars SQL](https://docs.pola.rs/api/python/stable/reference/sql/index.html) condition or a Polars expression. A row fails when its condition is `false`; null passes, as in a SQL `CHECK` constraint. Conditions see the values typed by the schema.

```rust
use fairspec::fairspec_table::polars::prelude::{col, lit};
use fairspec::fairspec_table::{TableCheck, TableCheckCondition};
use fairspec::{ValidateTableOptions, validate_table};
use serde_json::json;

let check = |name: &str, condition: TableCheckCondition| TableCheck {
    name: name.into(),
    condition,
    column_name: None,
    message: None,
};

let resource = serde_json::from_value(json!({"data": "orders.csv"}))?;
let report = validate_table(&resource, &ValidateTableOptions {
    checks: Some(vec![
        check("end-after-start", TableCheckCondition::Sql(r#""end" >= start"#.into())),
        TableCheck {
            column_name: Some("price".into()),
            ..check("positive-price", TableCheckCondition::Expr(col("price").gt(lit(0))))
        },
        check(
            "repeat-customer",
            TableCheckCondition::Sql("count(*) OVER (PARTITION BY customer) >= 2".into()),
        ),
        TableCheck {
            message: Some("The export is empty".into()),
            ..check("not-empty", TableCheckCondition::Sql("count(*) > 0".into()))
        },
    ]),
    ..Default::default()
})?;
```

- A condition that reduces to one value, such as `count(*) > 0`, checks the whole table and reports `table/check`
- Any other condition reports `row/check` for each failing row, or `cell/check` with the cell value when the check names a `column_name`
- A window (`OVER (PARTITION BY …)` in SQL, `.over(…)` in Polars) checks each row against its group
- `message` is carried on every error the check reports
- A check that cannot run (an unknown column, a syntax error, a condition that is not boolean) reports `check`, and the other checks still run
- Quote column names that are SQL keywords: `"end" >= start`

SQL-only checks also deserialise from JSON as `SqlTableCheck` and convert with `.into()`.

For an already-loaded table, use `inspect_table` to collect errors against an in-memory schema:

```rust
use fairspec::fairspec_table::inspect_table;

let errors = inspect_table(table, Some(&schema), None, Some(1000), Some(100), None)?;
```

Its arguments are the table, the schema, the checks, `sample_rows`, `max_errors` and `concurrency`.

## Querying Tables

Run SQL queries against a loaded table using the Polars SQL engine. The table is registered as `self`:

```rust
use fairspec::fairspec_table::query_table;

let filtered = query_table(table.clone(), "SELECT * FROM self WHERE age > 25")?;

let aggregated = query_table(
    table,
    "SELECT region, SUM(amount) AS total FROM self GROUP BY region",
)?;

println!("{}", filtered.collect()?);
```

`query_table` returns a `Table` (lazy), so further Polars operations can be chained before calling `.collect()`. `query_tables` takes several `(name, table)` pairs, each exposed under its name.

## Inferring a Table Schema

Two entry points depending on what you have:

```rust
use fairspec::fairspec_table::{InferTableSchemaOptions, infer_table_schema_from_table};
use fairspec::infer_table_schema;
use serde_json::json;

// From a Resource (file)
let resource = serde_json::from_value(json!({"data": "data.csv"}))?;
let schema = infer_table_schema(&resource, &InferTableSchemaOptions::default())?;

// From an already-loaded Table
let schema = infer_table_schema_from_table(table, &InferTableSchemaOptions {
    sample_rows: Some(100),
    confidence: Some(0.9),
    ..Default::default()
})?;
```

`InferTableSchemaOptions` includes `sample_rows`, `confidence`, `keep_strings`, `comma_decimal`, `month_first` and `concurrency`, plus the `table_schema_options` (`column_types`, `column_properties`, `missing_values`, date/time format strings). Use these to bias inference when defaults pick the wrong type. It deserialises from the same camelCase keys as Python's options.

`columnProperties` gives part of the schema up front and infers the rest. A property with a `type`
replaces the inferred one as written; one without a `type` keeps the inferred type and adds
`title`, `description` or `rdfType` to it, so give the `type` to add constraints. It takes precedence over `columnTypes`, and
naming a column the table lacks is an error. A resource's own `tableSchema` still replaces inference
entirely.

```rust
use fairspec::infer_table_schema;
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "data.csv"}))?;
let schema = infer_table_schema(&resource, &serde_json::from_value(json!({
    "columnProperties": {
        "id": {"type": "string"},
        "price": {"title": "Price"},
    },
}))?)?;
```

The inferred schema detects:

- Column types — string, integer, number, boolean, date, date-time, local-date-time, time, local-time, duration, etc.
- Required columns based on null presence
- Enum values for low-cardinality columns
- Numeric constraints (minimum, maximum)

Example:

```rust
use fairspec::fairspec_table::polars::prelude::{IntoLazy, df};
use fairspec::fairspec_table::{InferTableSchemaOptions, infer_table_schema_from_table};

let table = df!(
    "id" => ["1", "2", "3"],
    "price" => ["10.50", "25.00", "15.75"],
    "date" => ["2023-01-15", "2023-02-20", "2023-03-25"],
    "active" => ["true", "false", "true"],
)?
.lazy();

let schema = infer_table_schema_from_table(table, &InferTableSchemaOptions {
    sample_rows: Some(100),
    confidence: Some(0.9),
    ..Default::default()
})?;
```

## Rendering a Table Schema

`render_table_schema_as` asks the render plugins for the requested format:

```rust
use fairspec::fairspec_metadata::RenderTableSchemaOptions;
use fairspec::render_table_schema_as;

let markdown = render_table_schema_as(&schema, &RenderTableSchemaOptions {
    format: "markdown".into(),
    ..Default::default()
})?;
```

No Rust plugin renders yet, so the result is `None` for every format.

## Inferring a File Dialect

Detect a file's dialect (CSV delimiter, JSON pointer, Excel sheet, etc.) without loading the table:

```rust
use fairspec::fairspec_dataset::InferFileDialectOptions;
use fairspec::infer_file_dialect;
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "data.csv"}))?;
let dialect = infer_file_dialect(&resource, &InferFileDialectOptions::default())?;
```

See [Files](/rust/library/file/) for the broader file-operations API.

## Normalizing Tables

`normalize_table` converts string-typed columns into their schema-declared types (integer, number, date, boolean, etc.):

```rust
use fairspec::fairspec_metadata::TableSchema;
use fairspec::fairspec_table::normalize_table;
use fairspec::fairspec_table::polars::prelude::{IntoLazy, df};
use serde_json::json;

let table = df!(
    "id" => ["1", "2", "3"],
    "price" => ["10.50", "25.00", "15.75"],
    "active" => ["true", "false", "true"],
    "date" => ["2023-01-15", "2023-02-20", "2023-03-25"],
)?
.lazy();

let schema: TableSchema = serde_json::from_value(json!({
    "properties": {
        "id": {"type": "integer"},
        "price": {"type": "number"},
        "active": {"type": "boolean"},
        "date": {"type": "string", "format": "date"},
    },
}))?;

let typed = normalize_table(table, &schema)?;
```

`denormalize_table` does the reverse — converts a typed table back into a representation suitable for saving as CSV or JSON:

```rust
use fairspec::fairspec_table::{DenormalizeColumnOptions, denormalize_table};

let raw = denormalize_table(table, &schema, &DenormalizeColumnOptions {
    native_types: Some(vec!["string".into(), "number".into(), "boolean".into()]),
})?;
```

## Defining a Table Schema

Build schemas from a descriptor with column properties and constraints:

```rust
use fairspec::fairspec_metadata::TableSchema;
use serde_json::json;

let schema: TableSchema = serde_json::from_value(json!({
    "properties": {
        "id": {"type": "integer", "minimum": 1},
        "name": {"type": "string", "minLength": 1, "maxLength": 100},
        "email": {"type": "string", "pattern": r"^[^@]+@[^@]+\.[^@]+$"},
        "age": {"type": "integer", "minimum": 0, "maximum": 150},
        "status": {"type": "string", "enum": ["active", "inactive", "pending"]},
    },
    "required": ["id", "name", "email"],
    "primaryKey": ["id"],
    "uniqueKeys": [["email"]],
}))?;
```

Serde does not enforce the standard's constraints; check a schema descriptor with `validate_table_schema` from `fairspec::fairspec_metadata`.

### Supported Column Types

- **Primitive** — `string`, `integer`, `number`, `boolean`
- **Temporal** — `date`, `time`, `local-time`, `date-time`, `local-date-time`, `duration`; `date-time` and `time` values carry a UTC offset and are read as UTC
- **Spatial** — `geojson`, `wkt`, `wkb`
- **Complex** — `array`, `list`, `object`
- **Specialised** — `email`, `url`, `categorical`, `base64`, `hex`

## The Table Type

`Table` is an alias for `polars::prelude::LazyFrame`, and `fairspec::fairspec_table::polars` re-exports the Polars version Fairspec is built with:

```rust
use fairspec::fairspec_table::Table;
use fairspec::fairspec_table::polars::prelude::{IntoLazy, df};

let table: Table = df!("id" => [1, 2, 3])?.lazy();
```

This means the full Polars lazy API (filters, joins, group-by, window functions, …) is available directly on any loaded table — `load_table`, `query_table`, and `normalize_table` all return `LazyFrame`. Call `.collect()` when you want a materialised `DataFrame`.

## Common Workflows

### Explore Unknown Data

```rust
use fairspec::fairspec_dataset::InferFileDialectOptions;
use fairspec::fairspec_table::{LoadTableOptions, query_table};
use fairspec::{infer_file_dialect, infer_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "unknown-data.txt"}))?;

println!("{:?}", infer_file_dialect(&resource, &InferFileDialectOptions::default())?);

if let Some(table) = infer_table(&resource, &LoadTableOptions::default())? {
    println!("{}", table.clone().limit(10).collect()?);

    let preview = query_table(table, "SELECT * FROM self LIMIT 10")?;
    println!("{}", preview.collect()?);
}
```

### Schema-Driven Validation

```rust
use fairspec::fairspec_metadata::Resource;
use fairspec::fairspec_table::InferTableSchemaOptions;
use fairspec::{ValidateTableOptions, infer_table_schema, validate_table};
use serde_json::json;

let sample = serde_json::from_value(json!({"data": "sample.csv"}))?;
let schema = infer_table_schema(&sample, &InferTableSchemaOptions::default())?;

let resource: Resource = serde_json::from_value(json!({
    "data": "production.csv",
    "tableSchema": schema,
}))?;
let report = validate_table(&resource, &ValidateTableOptions::default())?;
assert!(report.valid, "{:?}", report.errors);
```

### Format Conversion

```rust
use fairspec::fairspec_table::{LoadTableOptions, SaveTableOptions};
use fairspec::{load_table, save_table};
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "data.csv"}))?;
if let Some(table) = load_table(&resource, &LoadTableOptions::default())? {
    save_table(table, &SaveTableOptions {
        path: "data.parquet".into(),
        file_dialect: Some(serde_json::from_value(json!({"format": "parquet"}))?),
        ..Default::default()
    })?;
}
```

## Examples

### CSV Data Analysis

```rust
use fairspec::fairspec_table::{LoadTableOptions, query_table};
use fairspec::load_table;
use serde_json::json;

let resource = serde_json::from_value(json!({"data": "sales.csv"}))?;
let table = load_table(&resource, &LoadTableOptions::default())?.ok_or("unsupported format")?;

let top_customers = query_table(
    table,
    "
    SELECT customer, SUM(amount) AS total
    FROM self
    GROUP BY customer
    ORDER BY total DESC
    LIMIT 10
    ",
)?;

println!("{}", top_customers.collect()?);
```

### Multi-Format Pipeline

```rust
use fairspec::fairspec_table::{LoadTableOptions, query_table};
use fairspec::{ValidateTableOptions, load_table, validate_table};
use serde_json::json;

let resource = serde_json::from_value(json!({
    "data": "report.xlsx",
    "fileDialect": {"format": "xlsx", "sheetName": "Q1 Sales"},
}))?;

let table = load_table(&resource, &LoadTableOptions::default())?.ok_or("unsupported format")?;
let totals = query_table(table, "SELECT region, SUM(revenue) FROM self GROUP BY region")?;
println!("{}", totals.collect()?);

let report = validate_table(&resource, &ValidateTableOptions::default())?;
assert!(report.valid);
```

### Remote Data Validation

```rust
use fairspec::fairspec_metadata::Resource;
use fairspec::fairspec_table::InferTableSchemaOptions;
use fairspec::{ValidateTableOptions, infer_table_schema, validate_table};
use serde_json::json;

let remote = serde_json::from_value(json!({"data": "https://api.example.com/export.csv"}))?;
let schema = infer_table_schema(&remote, &InferTableSchemaOptions::default())?;

let local: Resource = serde_json::from_value(json!({
    "data": "local-data.csv",
    "tableSchema": schema,
}))?;
let report = validate_table(&local, &ValidateTableOptions::default())?;

for error in &report.errors {
    let error = serde_json::to_value(error)?;
    println!("row {}: {}", error["rowNumber"], error["type"]);
}
```
