---
name: fairspec-validation
description: Use when checking whether data, a table, a dataset, a file or a schema conforms to Fairspec — choosing the command, the MCP tool or the TypeScript/Python call, and reading the report's errors to fix them.
---

# Validating with Fairspec

Every check returns one report: `{ valid, errors[] }`. The CLI prints it with `--json`
and exits 1 when `valid` is false. Validate the narrowest thing that answers the question.
Use the dataset only when foreign keys or several resources matter.

## Surfaces

Install with `npm install fairspec` (TypeScript CLI, library and MCP server) or
`pip install fairspec` (Python CLI, library and MCP server). Both ship a `fairspec` binary
with the same commands and flags. `fairspec mcp` starts the MCP server on stdio.

| target       | CLI                                         | MCP tool                  | TypeScript (async)         | Python                       |
| ------------ | ------------------------------------------- | ------------------------- | -------------------------- | ---------------------------- |
| table        | `fairspec table validate <path> --schema s` | `validateTableTool`       | `validateTable(resource)`  | `validate_table(resource)`   |
| dataset      | `fairspec dataset validate dataset.json`    | `validateDatasetTool`     | `validateDataset(source)`  | `validate_dataset(source)`   |
| JSON data    | `fairspec data validate <path> --schema s`  | `validateDataTool`        | `validateData(resource)`   | `validate_data(resource)`    |
| table schema | `fairspec table validate-schema <path>`     | `validateTableSchemaTool` | `validateTableSchema(src)` | `validate_table_schema(src)` |
| data schema  | `fairspec data validate-schema <path>`      | `validateDataSchemaTool`  | `validateDataSchema(src)`  | `validate_data_schema(src)`  |
| file         | `fairspec file validate <path> --hash h`    | —                         | `validateFile(resource)`   | `validate_file(resource)`    |

A resource is `{ data, tableSchema?, fileDialect?, dataSchema?, integrity? }`. In TypeScript pass a
plain object. In Python build `fairspec.Resource(data=..., tableSchema=...)`, since a dict is
rejected. A table inside a dataset is selected with `-d dataset.json -r <name>` in place of a path.

```ts
import { validateTable } from "fairspec"
const report = await validateTable({ data: "sales.csv", tableSchema: "schema.json" })
```

```python
from fairspec import Resource, validate_table
report = validate_table(Resource(data="sales.csv", tableSchema="schema.json"))
```

## Before validating

- **Without a schema, a table is checked against an inferred one.** Inference widens any column
  holding one bad value to `string`, so the bad value passes. To catch it, give a schema
  (`--schema`, or `tableSchema` on the resource) that says what the column must be. Draft one with
  **`fairspec-inference`**, tighten it, then validate
- Paths inside descriptors must be relative to the descriptor or `https://` URLs. Python
  rejects absolute paths, so run from the data's directory
- Validate a hand-written schema (`validate-schema`) before validating data against it. Otherwise
  a typo in the schema shows up as confusing data errors

## Reading the report

Each error has a `type`, plus `resourceName` within a dataset. Cell errors add `columnName`,
`rowNumber` (1-based, header excluded) and `cell` (the raw value).

| `type` family                                                 | means                                    | usual fix                                                                     |
| ------------------------------------------------------------- | ---------------------------------------- | ----------------------------------------------------------------------------- |
| `cell/type`                                                   | the value doesn't parse as `columnType`  | fix the value, or the schema's type/format, or add it to `missingValues`      |
| `cell/missing`                                                | empty value in a `required` column       | fill it, or drop the column from `required`                                   |
| `cell/minimum`, `cell/maxLength`, `cell/pattern`, `cell/enum` | a constraint failed                      | fix the value, or loosen the constraint if the data is right                  |
| `cell/unique`, `row/primaryKey`, `row/uniqueKey`              | duplicate key (`columnNames` lists them) | dedupe rows, or the key is wrong                                              |
| `foreignKey`                                                  | a reference has no target row            | add the target row, or fix the reference; only reported by dataset validation |
| `column/missing`, `column/type`                               | header vs schema mismatch                | rename the header or the schema property; check `headerRows` in the dialect   |
| `file/integrity`, `file/textual`                              | hash or encoding mismatch                | the file changed since it was described: re-hash (`fairspec file describe`)   |
| `resource/missing`, `resource/type`, `metadata`               | the descriptor itself is malformed       | fix the descriptor; validate it before the data                               |

Report errors grouped by `type` and `columnName`, with a count and a few sample rows. Don't
list every row. When `errors` is long, the first failure usually explains the rest.
