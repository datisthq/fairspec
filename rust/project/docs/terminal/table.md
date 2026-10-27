---
title: Working with Tables in Terminal
label: Table
path: /rust/terminal/table/
order: 2
---

Table operations including querying, validation, statistics, and schema management for tabular data files.

## Available Commands

The `fairspec table` command provides utilities for working with tables:

- `describe` - Get table statistics and summary information
- `query` - Query tables using SQL syntax
- `validate` - Validate table data against a Table Schema
- `infer-schema` - Automatically infer Table Schema from table data
- `render-schema` - Render Table Schema through the render plugins
- `validate-schema` - Validate a Table Schema file
- `infer-dialect` - Infer file dialect
- `script` - Interactive SQL session with loaded table

## Describe Tables

Get statistical summary information about a table:

```bash
# Describe a CSV file
fairspec table describe data.csv

# Describe a remote table
fairspec table describe https://example.com/data.csv

# Describe from a dataset
fairspec table describe --dataset dataset.json --resource sales

# Output as JSON
fairspec table describe data.csv --json
```

### Output

Returns statistics for each column including:

- `count` - Number of non-null values
- `null_count` - Number of null values
- `mean` - Average value (numeric columns)
- `std` - Standard deviation (numeric columns)
- `min` - Minimum value
- `25%`, `50%`, `75%` - Quartiles
- `max` - Maximum value

### Options

- `--dataset <path>` - Load table from dataset descriptor
- `--resource <name>` - Specify resource name from dataset
- `--debug` - Show debug information
- `--json` - Output as JSON

### Format Options

All standard format options are available (see Format Options section below).

## Query Tables

Execute SQL queries on tables using Polars SQL engine:

```bash
# Basic query
fairspec table query data.csv "SELECT * FROM self WHERE age > 25"

# Aggregate data
fairspec table query sales.csv "SELECT region, SUM(amount) as total FROM self GROUP BY region"

# Filter and sort
fairspec table query users.csv "SELECT name, email FROM self WHERE active = true ORDER BY name"

# Query from dataset resource
fairspec table query --dataset dataset.json --resource users \
  "SELECT * FROM self WHERE created_at > '2024-01-01'"
```

### SQL Syntax

- Use `self` as the table name in queries
- Supports SELECT, WHERE, GROUP BY, ORDER BY, LIMIT, JOIN, etc.
- Full Polars SQL syntax supported
- Results are output as formatted tables

### Options

- `--dataset <path>` - Load table from dataset descriptor
- `--resource <name>` - Specify resource name from dataset
- `--debug` - Show debug information
- `--json` - Output as JSON

## Validate Tables

Validate table data against a Table Schema:

```bash
# Validate with explicit schema
fairspec table validate data.csv --schema schema.json

# Validate with inferred schema
fairspec table validate data.csv

# Validate from dataset (uses embedded schema)
fairspec table validate --dataset dataset.json --resource users

# Output validation report as JSON
fairspec table validate data.csv --schema schema.json --json

# Add named SQL checks every row, or the whole table, must satisfy
fairspec table validate data.csv --check 'end-after-start="end" >= start' --check "not-empty=count(*) > 0"
```

A check is `name=condition`, split at the first `=`. A row fails when its condition is false; null passes. A condition that reduces to one value, such as `count(*) > 0`, checks the whole table. Failures are reported as `row/check` or `table/check` errors, and a condition that cannot run as a `check` error.

### Validation Report

Returns a validation report with:

- `valid` - Boolean indicating if validation passed
- `errors` - Array of validation errors (if any)

Example validation errors:

```json
{
  "valid": false,
  "errors": [
    {
      "type": "cell/maximum",
      "columnName": "age",
      "rowNumber": 2,
      "cell": "200",
      "maximum": "150"
    },
    {
      "type": "cell/type",
      "columnName": "email",
      "rowNumber": 3,
      "cell": "not-an-email",
      "columnType": "email"
    }
  ]
}
```

### Options

- `--schema <path>` - Path to Table Schema file
- `--check <name=condition>` - Named SQL condition to check (repeatable)
- `--dataset <path>` - Load table from dataset descriptor
- `--resource <name>` - Specify resource name from dataset
- `--debug` - Show debug information
- `--json` - Output as JSON

## Infer Table Schema

Automatically generate a Table Schema from table data:

```bash
# Infer schema from local file
fairspec table infer-schema data.csv

# Infer from remote file
fairspec table infer-schema https://example.com/data.csv

# Save inferred schema to file
fairspec table infer-schema data.csv --json > schema.json

# Infer with custom options
fairspec table infer-schema data.csv --sample-rows 1000 --confidence 0.95
```

### Schema Inference Options

- `--sample-rows <number>` - Number of rows to sample for inference (default: 100)
- `--confidence <number>` - Confidence threshold for type detection (0-1, default: 0.9)
- `--keep-strings` - Keep original string types instead of inferring
- `--column-types <pairs>` - Override types for specific columns (e.g., `id:integer,name:string`)
- `--column-properties <json>` - Give column properties up front and infer the rest (e.g., `{"id":{"type":"string"},"price":{"title":"Price"}}`); a property without `type` keeps the inferred type and adds only `title`, `description` or `rdfType`
- `--comma-decimal` - Treat comma as decimal separator
- `--month-first` - Parse dates as month-first (MM/DD/YYYY)

### Generated Schema

The inferred schema automatically detects:

- Column types (string, integer, number, boolean) and string formats (date, date-time, local-date-time, email, etc.)
- Nullable columns, typed as `["integer", "null"]`
- Missing value indicators such as `NA`, listed in `missingValues`

### Example

Given this CSV data:

```csv
id,name,price,quantity,active,created_at
1,Product A,19.99,100,true,2024-01-15
2,Product B,29.99,50,false,2024-01-20
3,Product C,39.99,75,true,2024-02-01
```

Infer the schema:

```bash
fairspec table infer-schema products.csv --json
```

Generated schema:

```json
{
  "properties": {
    "id": {
      "type": "integer"
    },
    "name": {
      "type": "string"
    },
    "price": {
      "type": "number"
    },
    "quantity": {
      "type": "integer"
    },
    "active": {
      "type": "boolean"
    },
    "created_at": {
      "type": "string",
      "format": "date"
    }
  }
}
```

## Render Table Schema

`render-schema` takes the same arguments as in the other CLIs, `--to-format` and `--to-path`, and asks the render plugins for the format. No Rust plugin renders yet, so it prints nothing; render with the Python or TypeScript CLI.

## Validate Table Schema

Validate that a Table Schema file is valid:

```bash
# Validate a schema file
fairspec table validate-schema schema.json

# Validate from remote source
fairspec table validate-schema https://example.com/schema.json

# Output as JSON
fairspec table validate-schema schema.json --json
```

### Schema Validation

This validates that the schema itself is:

- Valid JSON
- Compliant with Table Schema specification
- Has correct property definitions
- Uses valid column types and constraints

### Validation Report

```json
{
  "valid": true,
  "errors": []
}
```

Or if invalid:

```json
{
  "valid": false,
  "errors": [
    {
      "type": "metadata",
      "message": "\"x\" is not of type \"integer\"",
      "jsonPointer": "/properties/age/maximum"
    },
    {
      "type": "metadata",
      "message": "\"x\" is not of type \"number\"",
      "jsonPointer": "/properties/age/maximum"
    }
  ]
}
```

### Options

- `--silent` - Suppress output messages
- `--debug` - Show debug information
- `--json` - Output as JSON

## Infer File Dialect

Automatically detect the dialect of a table file:

```bash
# Infer dialect from file
fairspec table infer-dialect data.csv

# Infer from remote file
fairspec table infer-dialect https://example.com/data.xlsx

# Output as JSON
fairspec table infer-dialect data.parquet --json
```

### Detected Formats

The command can detect:

- `csv` - Comma-separated values
- `tsv` - Tab-separated values
- `json` - JSON format
- `jsonl` - JSON Lines (newline-delimited JSON)
- `xlsx` - Excel spreadsheet
- `ods` - OpenDocument Spreadsheet
- `parquet` - Apache Parquet
- `arrow` - Apache Arrow/Feather
- `sqlite` - SQLite database

### Example Output

```json
{
  "format": "csv",
  "delimiter": ",",
  "lineTerminator": "\n",
  "headerRows": [1]
}
```

## Interactive Scripting

Start an interactive session with a loaded table. Each line runs as SQL against the table, registered as `self`, and an empty line exits:

```bash
# Load table and start a session
fairspec table script data.csv

# Script table from dataset
fairspec table script --dataset dataset.json --resource users
```

The Python and TypeScript CLIs open a language REPL here; a compiled binary cannot, so the Rust session takes SQL only. The table is loaded as `load_table` returns it, so a CSV column without a schema is text: cast it, or use `table query`, which infers types first.

### Example Session

```text
⚠ Each line runs as SQL against `self`, the table; an empty line exits
> SELECT name, age FROM self LIMIT 2
shape: (2, 2)
...
> SELECT count(*) AS n FROM self WHERE CAST(age AS INT) > 25
shape: (1, 1)
...
>
```

## Format Options

All table commands support these format options for loading data:

### CSV/TSV Options

- `--format <name>` - Format name (csv, tsv, etc.)
- `--delimiter <char>` - Column delimiter (default: `,`)
- `--line-terminator <chars>` - Row terminator (default: `\n`)
- `--quote-char <char>` - Quote character (default: `"`)
- `--null-sequence <string>` - Null value indicator
- `--header-rows <numbers>` - Header row indices (e.g., `[1,2]`)
- `--header-join <char>` - Character to join multi-row headers
- `--comment-rows <numbers>` - Comment row indices to skip
- `--comment-prefix <char>` - Comment line prefix (e.g., `#`)
- `--column-names <names>` - Override column names (JSON array)

### JSON Options

- `--json-pointer <pointer>` - JSON pointer to data array (e.g., `/data/users`)
- `--row-type <type>` - Row format: `object` or `array`

### Excel/ODS Options

- `--sheet-number <number>` - Sheet index (0-based)
- `--sheet-name <name>` - Sheet name

### SQLite Options

- `--table-name <name>` - Table name in database

## Table Schema Options

All table commands support these schema-related options:

### Type Inference

- `--sample-rows <number>` - Sample size for type inference
- `--confidence <number>` - Confidence threshold (0-1)
- `--keep-strings` - Don't infer types, keep as strings
- `--column-types <pairs>` - Override types (e.g., `age:integer`)
- `--column-properties <json>` - Override column properties (e.g., `{"age":{"type":"integer","maximum":150}}`)

### Value Parsing

- `--missing-values <values>` - Missing value indicators (JSON array)
- `--decimal-char <char>` - Decimal separator (default: `.`)
- `--group-char <char>` - Thousands separator (default: `,`)
- `--comma-decimal` - Use comma as decimal (shorthand)
- `--true-values <values>` - Custom true values (JSON array)
- `--false-values <values>` - Custom false values (JSON array)

### Date/Time Parsing

- `--datetime-format <format>` - Datetime format string
- `--date-format <format>` - Date format string
- `--time-format <format>` - Time format string
- `--month-first` - Parse dates as month-first

### Array/List Parsing

- `--array-type <type>` - Array item type
- `--list-delimiter <char>` - List delimiter (default: `,`)
- `--list-item-type <type>` - List item type

## Common Workflows

### Explore Unknown Data

```bash
# 1. Infer the dialect
fairspec table infer-dialect unknown-data.txt

# 2. Get basic statistics
fairspec table describe unknown-data.txt

# 3. Infer the schema
fairspec table infer-schema unknown-data.txt --json > schema.json

# 4. Query the data
fairspec table query unknown-data.txt "SELECT * FROM self LIMIT 10"
```

### Schema-Driven Validation

```bash
# 1. Create schema from sample data
fairspec table infer-schema sample.csv --json > schema.json

# 2. Validate the schema itself
fairspec table validate-schema schema.json

# 3. Validate production data
fairspec table validate production.csv --schema schema.json
```

### Data Quality Checks

```bash
# Check for data quality issues
fairspec table validate data.csv --schema schema.json

# Get detailed statistics
fairspec table describe data.csv

# Query for specific issues
fairspec table query data.csv "SELECT * FROM self WHERE email NOT LIKE '%@%'"

# Find duplicates
fairspec table query data.csv "SELECT id, COUNT(*) as cnt FROM self GROUP BY id HAVING cnt > 1"
```

### Interactive Analysis

```bash
# Start interactive session
fairspec table script data.csv

# Each line runs as SQL against `self`:
# - Explore: SELECT * FROM self LIMIT 10
# - Filter: SELECT * FROM self WHERE status = 'active'
# - Aggregate: SELECT category, SUM(CAST(amount AS DOUBLE)) FROM self GROUP BY category
```

### Format Conversion

```bash
# Query and output as JSON
fairspec table query data.csv "SELECT * FROM self" --json > output.json

# Get statistics and save
fairspec table describe large-file.parquet --json > stats.json
```

## Output Formats

### Text Output (default)

Human-readable output with formatted tables:

```bash
fairspec table describe data.csv
```

Output:

```text
shape: (9, 4)
┌────────────┬──────────┬───────────┬───────────┐
│ #          ┆ id       ┆ price     ┆ quantity  │
│ ---        ┆ ---      ┆ ---       ┆ ---       │
│ str        ┆ f64      ┆ f64       ┆ f64       │
╞════════════╪══════════╪═══════════╪═══════════╡
│ count      ┆ 4.0      ┆ 4.0       ┆ 4.0       │
│ null_count ┆ 0.0      ┆ 0.0       ┆ 0.0       │
│ mean       ┆ 2.5      ┆ 39.99     ┆ 69.0      │
│ std        ┆ 1.290994 ┆ 40.824829 ┆ 62.134263 │
│ min        ┆ 1.0      ┆ 9.99      ┆ 1.0       │
│ 25%        ┆ 2.0      ┆ 19.99     ┆ 50.0      │
│ 50%        ┆ 3.0      ┆ 29.99     ┆ 75.0      │
│ 75%        ┆ 3.0      ┆ 29.99     ┆ 75.0      │
│ max        ┆ 4.0      ┆ 99.99     ┆ 150.0     │
└────────────┴──────────┴───────────┴───────────┘
```

### JSON Output

Machine-readable JSON for automation:

```bash
fairspec table describe data.csv --json
```

## Examples

### CSV Data Analysis

```bash
# Get overview of sales data
fairspec table describe sales.csv

# Find top customers
fairspec table query sales.csv \
  "SELECT customer, SUM(amount) as total FROM self GROUP BY customer ORDER BY total DESC LIMIT 10"

# Validate data quality
fairspec table validate sales.csv --schema sales-schema.json
```

### Multi-Format Pipeline

```bash
# Load Excel data
fairspec table describe report.xlsx --sheet-name "Q1 Sales"

# Query specific sheet
fairspec table query report.xlsx --sheet-name "Q1 Sales" \
  "SELECT region, SUM(revenue) FROM self GROUP BY region"

# Validate against schema
fairspec table validate report.xlsx --sheet-name "Q1 Sales" --schema schema.json
```

### Remote Data Validation

```bash
# Infer schema from remote data
fairspec table infer-schema https://api.example.com/export.csv --json > remote-schema.json

# Validate local data against remote schema
fairspec table validate local-data.csv --schema remote-schema.json
```

### Database Export Validation

```bash
# Validate SQLite export
fairspec table validate export.db --table-name users --schema expected-schema.json

# Get statistics from database
fairspec table describe export.db --table-name users

# Query database table
fairspec table query export.db --table-name users \
  "SELECT status, COUNT(*) FROM self GROUP BY status"
```
