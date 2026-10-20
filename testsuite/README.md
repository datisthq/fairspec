# Testsuite

The language-neutral conformance test suite that every Fairspec implementation runs: the same input data,
the same operation, the same expected result. The TypeScript and Python libraries both run
it, so a difference between them is a failing test rather than a surprise for a user.

The format follows [JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite).

## Layout

```
testsuite/
  test-schema.json             JSON Schema of a test file
  tests/<operation>/<feature>.json
```

Each directory under `tests/` is one operation, named after the CLI command both libraries
share (`table infer-schema` is `table-infer-schema`). Each file covers one feature of it.

## Test files

A file is an array of groups. A group fixes the subject; its tests vary only the data:

```json
[
  {
    "description": "integer column",
    "tableSchema": { "properties": { "id": { "type": "integer" } } },
    "tests": [
      { "description": "integers are valid", "data": "id\n1\n2\n", "valid": true },
      {
        "description": "a word is a cell/type error",
        "data": "id\nx\n",
        "valid": false,
        "errors": [
          {
            "type": "cell/type",
            "columnName": "id",
            "columnType": "integer",
            "rowNumber": 1,
            "cell": "x"
          }
        ]
      }
    ]
  }
]
```

A group has a `description` and `tests`. It may also carry:

- `comment`
- `specification`: links to the spec sections it exercises
- the subject of the operation: `format` (the file extension, `csv` by default),
  `fileDialect`, `tableSchema`, `dataSchema`, `integrity`, `textual`, `dataset`, `query` and
  `options`. Options use the camelCase names of the TypeScript API; `file-describe` reads
  `options.hashType`
- `inline: true`: pass the test's `data` to the resource as inline data instead of writing a
  file

A test has a `description`, its `data`, and exactly one outcome:

- `data` is one of:
  - **a string**: the exact file content, so `\r\n` stays `\r\n`
  - `{ "base64": "…" }`: binary content
  - **an array**: rows for a JSON format
  - **an object**: inline JSON with `inline: true`, or for the dataset operations a map of
    each file name to its content (a string or `{ "base64": … }`)
- The outcome is one of:
  - `valid` with optional `errors`, for validate operations. When `valid` is true, or `errors`
    is given, the report must equal `{ valid, errors }` exactly, in order
  - `expected`, for infer, describe and load operations. The result must equal it exactly
  - `throws: true`, when the operation must raise

## Operations

| operation             | runs                                                                  | compared                                  |
| --------------------- | --------------------------------------------------------------------- | ----------------------------------------- |
| `data-infer-schema`   | infer the data schema                                                 | `expected`: the JSON Schema               |
| `data-validate`       | validate the data against the group's `dataSchema`                    | `valid`, `errors`                         |
| `dataset-infer`       | write the dataset, infer it, make paths relative again                | `expected`: the dataset                   |
| `dataset-list`        | write the dataset, list its resource names (inferred when absent)     | `expected`: the names                     |
| `dataset-query`       | write the dataset and run the group's `query`, each resource by name  | `expected`: `{columns, rows}`             |
| `dataset-validate`    | write the files and the `dataset` as `dataset.json`, then validate it | `valid`, `errors`                         |
| `file-describe`       | describe the file written from `data`                                 | `expected`: `{bytes, textual, integrity}` |
| `file-validate`       | validate a resource with the group's `integrity` / `textual`          | `valid`, `errors`                         |
| `table-infer-dialect` | infer the file dialect                                                | `expected`: the dialect                   |
| `table-infer-schema`  | infer the table schema                                                | `expected`: the schema                    |
| `table-load`          | load the table                                                        | `expected`: `{columns, rows}`             |
| `table-query`         | load the table and run the group's `query` against it as `self`       | `expected`: `{columns, rows}`             |
| `table-validate`      | validate the table                                                    | `valid`, `errors`                         |

## Runners

- TypeScript: `typescript/library/-test/testsuite.unit.ts`, run by `pnpm -C typescript test:unit`
- Python: `python/library/fairspec_library/_test/testsuite_unit.py`, run by `pnpm -C python test:unit`

Each runner:

1. writes `data` to a temporary file with the group's `format` as its extension
2. calls the library on a resource pointing at that file
3. serialises the result to JSON, with no further normalisation, and compares

A difference in number type (`1` against `1.0`), key presence or order is a failure. Fix it
in the library, never by loosening a runner. Two rules are fixed in both runners alike:

- Errors of type `data`, `metadata` and `cell/json` are compared without `message`: its text
  comes from each language's JSON Schema validator and is not normative. Leave `message` out
  of the expected error
- Date, time and date-time columns in `{columns, rows}` are rendered as text, `%Y-%m-%d`,
  `%H:%M:%S` and `%Y-%m-%dT%H:%M:%S`, through the library's own table query

Binary inputs (xlsx, ods, parquet, arrow, sqlite) are tiny files generated once with the Python
library and embedded as base64; the group's `comment` says how each was made.

Each file is also validated against `test-schema.json` by both runners.
