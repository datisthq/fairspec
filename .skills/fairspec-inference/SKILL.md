---
name: fairspec-inference
description: Use when describing data files with Fairspec — inferring a file dialect, a table schema, a JSON data schema or a whole dataset descriptor from files, via the CLI, the MCP server or the TypeScript/Python library.
---

# Inferring Fairspec metadata

Inference reads a sample of the data and proposes metadata. It returns a draft, and you
review it before anyone relies on it. Then check the data against it with
**`fairspec-validation`**.

## Surfaces

Install with `npm install fairspec` or `pip install fairspec`. Both ship a `fairspec` binary with the
same commands and flags, and `fairspec mcp` (TypeScript only) serves the MCP tools. Add `--json` to get
machine-readable output on stdout, then redirect it to save a descriptor.

| infer              | CLI                                          | MCP tool               | TypeScript (async)           | Python                         |
| ------------------ | -------------------------------------------- | ---------------------- | ---------------------------- | ------------------------------ |
| dataset descriptor | `fairspec dataset infer a.csv b.xlsx --json` | `inferDatasetTool`     | `inferDataset(dataset)`      | `infer_dataset(dataset)`       |
| table schema       | `fairspec table infer-schema <path> --json`  | `inferTableSchemaTool` | `inferTableSchema(resource)` | `infer_table_schema(resource)` |
| file dialect       | `fairspec table infer-dialect <path> --json` | `inferFileDialectTool` | `inferFileDialect(resource)` | `infer_file_dialect(resource)` |
| data schema (JSON) | `fairspec data infer-schema <path> --json`   | `inferDataSchemaTool`  | `inferDataSchema(resource)`  | `infer_data_schema(resource)`  |

`dataset infer` produces each resource's `name`, `fileDialect`, `textual`, `integrity` (sha256)
and `tableSchema` in one pass. It is usually the right start:

```sh
fairspec dataset infer sales.csv stores.csv --json > dataset.json
```

```ts
import { inferDataset, saveDatasetDescriptor } from "fairspec"
const dataset = await inferDataset({ resources: [{ data: "sales.csv" }] })
await saveDatasetDescriptor(dataset, { path: "dataset.json" })
```

```python
from fairspec import Dataset, Resource, infer_dataset, save_dataset_descriptor
dataset = infer_dataset(Dataset(resources=[Resource(data="sales.csv")]))
save_dataset_descriptor(dataset, path="dataset.json")
```

## Steering inference

The table commands take the same hints: on the CLI as flags, in the libraries as options. Use them
when the draft is wrong, instead of hand-editing every column:

- `--sample-rows <n>`, `--confidence <0..1>`: sample more rows, or require more agreement before
  choosing a type
- `--comma-decimal`, `--month-first`: European numbers, US dates
- `--keep-strings`: skip typing entirely, every column stays `string`
- `--column-types id:integer,price:number`: pin some columns' types
- dialect flags (`--delimiter`, `--header-rows 1,2`, `--sheet-name`, `--json-pointer`) when the
  file's layout itself is misread. Check `infer-dialect` first

## Reviewing the draft

- **A column with one bad value is inferred as `string`.** A text column that should be numeric or a
  date is a data problem that inference hid. Set the intended type, then validate to find the rows
- Inference never adds `required`, `primaryKey`, `uniqueKeys`, `foreignKeys`, `enum` or
  `minimum`/`maximum`. Add the ones the data owner can vouch for
- A table schema is JSON-Schema shaped: `{ "primaryKey": [...], "properties": { "col": { "type":
"integer" } } }`. Dates are `"type": "string", "format": "date"`
- `integrity` hashes the file as it is now, so re-infer or re-hash after editing the data
- Keep `data` paths relative to the descriptor. Python rejects absolute ones
