---
title: Working with Tables in Python
label: Table
path: /python/table/
order: 2
---
Load, save, query, and validate tabular data in Python. Tables are represented as **Polars** `LazyFrame` objects for high-performance processing.

## Available Functions

The table API provides utilities for working with tabular data:

- `load_table` - Load a table from a `Resource` (format-agnostic)
- `save_table` - Save a table to disk in a specified format
- `infer_table` - Load a table and infer its missing dialect and schema
- `validate_table` - Validate table data against a Table Schema
- `query_table` - Run a SQL query against a loaded table
- `inspect_table` - Validate an in-memory table and collect errors
- `infer_table_schema` - Infer a Table Schema from a resource
- `infer_table_schema_from_table` - Infer a Table Schema from an in-memory table
- `render_table_schema_as` - Render a Table Schema as Markdown or HTML
- `normalize_table` / `denormalize_table` - Convert between typed and string representations

## Loading a Table

Load a table from a `Resource`. The format is detected from the file extension or dialect, and dispatched to the appropriate plugin:

```python
from fairspec import Resource, load_table

# Load a local CSV
table = load_table(Resource(data="data.csv"))

# Load a remote file
table = load_table(Resource(data="https://example.com/data.csv"))

# Load with an explicit dialect
from fairspec import CsvFileDialect
table = load_table(Resource(
    data="data.csv",
    fileDialect=CsvFileDialect(delimiter=";"),
))
```

`load_table` returns `Table | None` — `None` means no plugin recognised the resource's format. The returned `Table` is an alias for `polars.LazyFrame`, so call `.collect()` to materialise it:

```python
df = table.collect()
print(df)
```

Format-specific loaders are available when you want to bypass detection: `load_csv_table`, `load_json_table`, `load_xlsx_table`, `load_arrow_table`, `load_parquet_table`, `load_sqlite_table`, `load_inline_table`. See the per-format guides for their options.

## Saving a Table

Save a table to disk. The `path` is required; the `fileDialect` controls the output format:

```python
from fairspec import CsvFileDialect, save_table

save_table(table, path="output.csv")

# Custom dialect
save_table(table, path="output.csv", fileDialect=CsvFileDialect(delimiter="\t"))

# Overwrite existing file
save_table(table, path="output.csv", overwrite=True)
```

`SaveTableOptions`:

- `path` (required) — output path
- `fileDialect` — output dialect (auto-detected from path if omitted)
- `tableSchema` — optional Table Schema (drives type-aware serialisation)
- `overwrite` — replace an existing file instead of erroring

## Inferring a Table

`infer_table` loads a table **and** fills in any missing dialect / schema metadata on the resource in one call:

```python
from fairspec import Resource, infer_table

resource = Resource(data="data.csv")
table = infer_table(resource)

# resource.fileDialect and resource.tableSchema are now populated
print(resource.fileDialect)
print(resource.tableSchema)
```

This is the preferred entry point when you have a path and want both the data and its inferred metadata.

## Validating a Table

Validate table data against a Table Schema:

```python
from fairspec import Resource, validate_table

# Validate with an inferred schema
report = validate_table(Resource(data="data.csv"))

# Validate against a provided schema
resource = Resource(data="data.csv", tableSchema=schema)
report = validate_table(resource)

if not report.valid:
    for error in report.errors:
        print(f"row {error.rowNumber}: {error.message}")
```

The returned `Report` has `valid: bool` and `errors: list[FairspecError]`. Each error carries context such as `rowNumber`, `propertyName`, `cellValue`, and a `type` discriminator (for example, `cell/type`, `row/primary-key`, `column/missing`).

For an already-loaded table, use `inspect_table` to collect errors against an in-memory schema:

```python
from fairspec import inspect_table

errors = inspect_table(table, table_schema=schema, sample_rows=1000, max_errors=100)
```

## Querying Tables

Run SQL queries against a loaded table using the Polars SQL engine. The table is registered as `self`:

```python
from fairspec import query_table

filtered = query_table(table, "SELECT * FROM self WHERE age > 25")

aggregated = query_table(
    table,
    "SELECT region, SUM(amount) AS total FROM self GROUP BY region",
)

print(filtered.collect())
```

`query_table` returns a `Table` (lazy), so further Polars operations can be chained before calling `.collect()`.

## Inferring a Table Schema

Two entry points depending on what you have:

```python
from fairspec import Resource, infer_table_schema, infer_table_schema_from_table

# From a Resource (file)
schema = infer_table_schema(Resource(data="data.csv"))

# From an already-loaded Table
schema = infer_table_schema_from_table(table, sample_rows=100, confidence=0.9)
```

`InferTableSchemaOptions` includes `sampleRows`, `confidence`, `keepStrings`, `columnTypes`, `missingValues`, `commaDecimal`, `monthFirst`, and date/time format strings. Use these to bias inference when defaults pick the wrong type.

The inferred schema detects:

- Column types — string, integer, number, boolean, date, datetime, time, duration, etc.
- Required columns based on null presence
- Enum values for low-cardinality columns
- Numeric constraints (minimum, maximum)

Example:

```python
import polars as pl
from fairspec import infer_table_schema_from_table

table = pl.DataFrame({
    "id": ["1", "2", "3"],
    "price": ["10.50", "25.00", "15.75"],
    "date": ["2023-01-15", "2023-02-20", "2023-03-25"],
    "active": ["true", "false", "true"],
}).lazy()

schema = infer_table_schema_from_table(table, sample_rows=100, confidence=0.9)
```

## Rendering a Table Schema

Render a Table Schema as Markdown or HTML documentation:

```python
from fairspec import RenderTableSchemaOptions, render_table_schema_as

markdown = render_table_schema_as(schema, RenderTableSchemaOptions(format="markdown"))
html = render_table_schema_as(schema, RenderTableSchemaOptions(format="html"))
```

## Inferring a File Dialect

Detect a file's dialect (CSV delimiter, JSON pointer, Excel sheet, etc.) without loading the table:

```python
from fairspec import Resource, infer_file_dialect

dialect = infer_file_dialect(Resource(data="data.csv"))
```

See [Files](/python/file/) for the broader file-operations API.

## Normalizing Tables

`normalize_table` converts string-typed columns into their schema-declared types (integer, number, date, boolean, etc.):

```python
import polars as pl
from fairspec import (
    BooleanColumnProperty,
    DateColumnProperty,
    IntegerColumnProperty,
    NumberColumnProperty,
    TableSchema,
    normalize_table,
)

table = pl.DataFrame({
    "id": ["1", "2", "3"],
    "price": ["10.50", "25.00", "15.75"],
    "active": ["true", "false", "true"],
    "date": ["2023-01-15", "2023-02-20", "2023-03-25"],
}).lazy()

schema = TableSchema(properties={
    "id": IntegerColumnProperty(),
    "price": NumberColumnProperty(),
    "active": BooleanColumnProperty(),
    "date": DateColumnProperty(),
})

typed = normalize_table(table, schema)
```

`denormalize_table` does the reverse — converts a typed table back into a representation suitable for saving as CSV or JSON:

```python
from fairspec import denormalize_table

raw = denormalize_table(typed, schema, native_types=["string", "number", "boolean"])
```

## Defining a Table Schema

Build schemas with column properties and constraints:

```python
from fairspec import (
    IntegerColumnProperty,
    StringColumnProperty,
    TableSchema,
    UniqueKey,
)

schema = TableSchema(
    properties={
        "id": IntegerColumnProperty(minimum=1),
        "name": StringColumnProperty(minLength=1, maxLength=100),
        "email": StringColumnProperty(pattern=r"^[^@]+@[^@]+\.[^@]+$"),
        "age": IntegerColumnProperty(minimum=0, maximum=150),
        "status": StringColumnProperty(enum=["active", "inactive", "pending"]),
    },
    required=["id", "name", "email"],
    primaryKey=["id"],
    uniqueKeys=[UniqueKey(columnNames=["email"])],
)
```

### Supported Column Types

- **Primitive** — `string`, `integer`, `number`, `boolean`
- **Temporal** — `date`, `datetime`, `time`, `duration`
- **Spatial** — `geojson`, `wkt`, `wkb`
- **Complex** — `array`, `list`, `object`
- **Specialised** — `email`, `url`, `categorical`, `base64`, `hex`

## The Table Type

`Table` is an alias for `polars.LazyFrame`:

```python
import polars as pl
from fairspec import Table

table: Table = pl.DataFrame({"id": [1, 2, 3]}).lazy()
```

This means the full Polars lazy API (filters, joins, group-by, window functions, …) is available directly on any loaded table — `load_table`, `query_table`, and `normalize_table` all return `LazyFrame`. Call `.collect()` when you want a materialised `DataFrame`.

## Common Workflows

### Explore Unknown Data

```python
from fairspec import Resource, infer_file_dialect, infer_table, query_table

resource = Resource(data="unknown-data.txt")

print(infer_file_dialect(resource))

table = infer_table(resource)
print(table.head(10).collect())

preview = query_table(table, "SELECT * FROM self LIMIT 10")
print(preview.collect())
```

### Schema-Driven Validation

```python
from fairspec import Resource, infer_table_schema, render_table_schema_as, RenderTableSchemaOptions, validate_table

schema = infer_table_schema(Resource(data="sample.csv"))

markdown = render_table_schema_as(schema, RenderTableSchemaOptions(format="markdown"))
with open("schema.md", "w") as fp:
    fp.write(markdown or "")

report = validate_table(Resource(data="production.csv", tableSchema=schema))
assert report.valid, report.errors
```

### Format Conversion

```python
from fairspec import CsvFileDialect, ParquetFileDialect, Resource, load_table, save_table

table = load_table(Resource(data="data.csv"))
save_table(table, path="data.parquet", fileDialect=ParquetFileDialect())
```

## Examples

### CSV Data Analysis

```python
from fairspec import Resource, load_table, query_table

table = load_table(Resource(data="sales.csv"))

top_customers = query_table(
    table,
    """
    SELECT customer, SUM(amount) AS total
    FROM self
    GROUP BY customer
    ORDER BY total DESC
    LIMIT 10
    """,
)

print(top_customers.collect())
```

### Multi-Format Pipeline

```python
from fairspec import Resource, XlsxFileDialect, load_table, query_table, validate_table

resource = Resource(
    data="report.xlsx",
    fileDialect=XlsxFileDialect(sheetName="Q1 Sales"),
)

table = load_table(resource)
totals = query_table(table, "SELECT region, SUM(revenue) FROM self GROUP BY region")
print(totals.collect())

report = validate_table(resource)
assert report.valid
```

### Remote Data Validation

```python
from fairspec import Resource, infer_table_schema, validate_table

remote = Resource(data="https://api.example.com/export.csv")
schema = infer_table_schema(remote)

local = Resource(data="local-data.csv", tableSchema=schema)
report = validate_table(local)

for error in report.errors:
    print(f"row {error.rowNumber}: {error.message}")
```
