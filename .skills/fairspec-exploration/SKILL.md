---
name: fairspec-exploration
description: Use when looking into tabular data with Fairspec — previewing rows, column stats, SQL queries over a CSV/Excel/JSON/Parquet table, listing a dataset's resources, or a file's size and hash, from the CLI, MCP server or TypeScript/Python library.
---

# Exploring data with Fairspec

Fairspec loads any supported table (CSV, TSV, Excel, ODS, JSON, JSONL, Parquet, Arrow, SQLite, …)
as a Polars LazyFrame, typed by its schema. Explore before you infer or validate: a quick look
shows the delimiter, header and type problems those steps would trip on.

## Surfaces

Install with `npm install fairspec` or `pip install fairspec`. Both ship a `fairspec` binary with the
same commands and flags, and `fairspec mcp` (TypeScript only) serves the MCP tools.

| task                | CLI                                          | MCP tool         | TypeScript / Python                                  |
| ------------------- | -------------------------------------------- | ---------------- | ---------------------------------------------------- |
| SQL query           | `fairspec table query <path> "<sql>" --json` | `queryTableTool` | `queryTable(table, sql)` / `query_table(table, sql)` |
| first rows          | `fairspec table preview <path>`              | —                | `loadTable(resource)` / `load_table(resource)`       |
| column stats        | `fairspec table describe <path> --json`      | —                | load, then Polars `describe()`                       |
| dataset's resources | `fairspec dataset list dataset.json --json`  | —                | `loadDataset(path)` / `load_dataset(path)`           |
| file size, hash     | `fairspec file describe <path> --json`       | —                | `describeFile(path)` / `describe_file(path)`         |

In SQL the table is named `self`: `SELECT name, sum(price) FROM self GROUP BY name`. The query runs
on Polars SQL, not a database, so stick to standard `SELECT` syntax. A table inside a dataset is
selected with `-d dataset.json -r <name>` in place of a path. All table commands take the same
dialect flags (`--delimiter`, `--header-rows`, `--sheet-name`, …) and `--schema`.

```ts
import { loadTable, queryTable } from "fairspec"
const table = await loadTable({ data: "sales.csv" })
if (table)
  console.log(
    (await queryTable(table, "SELECT count(*) AS n FROM self").collect()).toRecords(),
  )
```

```python
from fairspec import Resource, load_table, query_table
table = load_table(Resource(data="sales.csv"))
print(query_table(table, "SELECT count(*) AS n FROM self").collect())
```

Both libraries return a lazy frame: call `.collect()` before reading rows, and narrow with SQL or
`limit` first on large files. `loadTable` returns `undefined` (`None`) for a non-tabular resource.

## Gotchas

- The TypeScript `table describe` fails on any table with a date column
  (`type Date is incompatible with expected type Datetime`). Use the Python CLI's `table describe`,
  or SQL aggregates through `table query`
- Without `--schema`, columns are typed by inference, and a column with one bad value comes back as
  `string`. Numbers you expected sorting as text, or a `min`/`max` that looks wrong, mean dirty
  values: validate against a schema (**`fairspec-validation`**) to find the rows
- A column read as one `string` blob, or a header row read as data, means the dialect is wrong.
  Check `fairspec table infer-dialect <path> --json` and override it with dialect flags
- Paths inside a descriptor are relative to it. Python rejects absolute ones, so run from the data's
  directory
