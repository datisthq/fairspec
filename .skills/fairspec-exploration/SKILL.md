---
name: fairspec-exploration
description: Use when looking into tabular data with Fairspec — previewing rows, column stats, SQL queries over a CSV/Excel/JSON/Parquet table, listing a dataset's resources, a file's size and hash, from the CLI, MCP server or TypeScript/Python/Rust library.
---

# Exploring data with Fairspec

Fairspec loads any supported table (CSV, TSV, Excel, ODS, JSON, JSONL, Parquet, Arrow, SQLite, …)
as a Polars LazyFrame, typed by its schema. Explore before you infer or validate: a quick look
shows the delimiter, header and type problems those steps would trip on.

## Surfaces

Install with `npm install fairspec`, `pip install fairspec`, or the Rust binary
(`curl -fsSL https://fairspec.org/install.sh | sh -s -- terminal`, or `cargo install fairspec`;
`cargo add fairspec` for the library). Each ships a `fairspec` binary with the same commands and flags,
and `fairspec mcp` serves the MCP tools. Each also prints every
command's input and output schema with `fairspec --llms`.

| task                 | CLI                                                  | MCP tool           | TypeScript / Python                                    | Rust                                               |
| -------------------- | ---------------------------------------------------- | ------------------ | ------------------------------------------------------ | -------------------------------------------------- |
| SQL query            | `fairspec table query <path> "<sql>" --json`         | `queryTableTool`   | `queryTable(table, sql)` / `query_table(table, sql)`   | `fairspec_table::query_table(table, sql)`          |
| SQL across a dataset | `fairspec dataset query dataset.json "<sql>" --json` | `queryDatasetTool` | `queryDataset(path, sql)` / `query_dataset(path, sql)` | `query_dataset(path, sql, &options)`               |
| first rows           | `fairspec table preview <path>`                      | —                  | `loadTable(resource)` / `load_table(resource)`         | `load_table(&resource, &options)`                  |
| raw source           | `fairspec file preview <path>`                       | `previewFileTool`  | `previewFile(path)` / `preview_file(path)`             | `fairspec_dataset::preview_file(path, None, None)` |
| column stats         | `fairspec table describe <path> --json`              | —                  | load, then Polars `describe()`                         | load, then SQL aggregates                          |
| dataset's resources  | `fairspec dataset list dataset.json --json`          | —                  | `loadDataset(path)` / `load_dataset(path)`             | `load_dataset(path)`                               |
| file size, hash      | `fairspec file describe <path> --json`               | —                  | `describeFile(path)` / `describe_file(path)`           | `fairspec_dataset::describe_file(path, None)`      |

In Rust `options` is `LoadTableOptions::default()` (from `fairspec::fairspec_table`), and every
call is synchronous and returns `Result`.

In SQL the table is named `self`: `SELECT name, sum(price) FROM self GROUP BY name`. A dataset
query names each table after its resource instead (`dataset list` shows the names), so it can join
resources: `SELECT u.name, count(*) FROM orders o JOIN users u ON o.user_id = u.id GROUP BY u.name`.
The query runs on Polars SQL, not a database, so stick to standard `SELECT` syntax. A table inside a dataset is
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

```rust
use fairspec::fairspec_table::{LoadTableOptions, query_table};
use fairspec::load_table;
let resource = serde_json::from_value(serde_json::json!({"data": "sales.csv"}))?;
if let Some(table) = load_table(&resource, &LoadTableOptions::default())? {
    println!("{}", query_table(table, "SELECT count(*) AS n FROM self")?.collect()?);
}
```

All libraries return a lazy frame: call `.collect()` before reading rows, and narrow with SQL or
`limit` first on large files. `loadTable` returns `undefined` (`None` in Python and Rust) for a
non-tabular resource.

## Gotchas

- `queryTableTool` and `queryDatasetTool` return at most `limit` records (default 100, up to 1000), fewer when they
  would exceed ~50k characters; `truncated: true` means rows were left out. Aggregate in SQL
  rather than raising `limit` to read a whole table
- `previewFileTool` returns at most `maxLines` lines (default 20, up to 200) or `maxBytes` bytes
  (default 10,000, up to 20,000); `truncated: true` means the file continues
- The TypeScript `table describe` fails on any table with a date column
  (`type Date is incompatible with expected type Datetime`). Use the Python CLI's `table describe`,
  or SQL aggregates through `table query`
- Without `--schema`, columns are typed by inference, and a column with one bad value comes back as
  `string`. Numbers you expected sorting as text, or a `min`/`max` that looks wrong, mean dirty
  values: validate against a schema (**`fairspec-validation`**) to find the rows
- A column read as one `string` blob, or a header row read as data, means the dialect is wrong.
  Check `fairspec table infer-dialect <path> --json` and override it with dialect flags
- Paths inside a descriptor are relative to it. An absolute one is a `metadata` error in every
  library, so run from the data's directory
- The MCP server fetches only public URLs: one on `localhost` or a private network fails with
  `Host … resolves to non-public address …`. Set `FAIRSPEC_HOST_POLICY=any` in the server's `env`
  to reach it; the CLI and the libraries fetch any URL unless it is set to `public`
