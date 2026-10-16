# Conformance

A language-neutral test suite that every Fairspec implementation runs: the same input data,
the same operation, the same expected result. The TypeScript and Python libraries both run
it, so a difference between them is a failing test rather than a surprise for a user.

The format follows [JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite).

## Layout

```
conformance/
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
  `fileDialect`, `tableSchema`, `integrity`, `dataset` and `options`. Options use the camelCase
  names of the TypeScript API

A test has a `description`, its `data`, and exactly one outcome:

- `data` is one of:
  - **a string**: the exact file content, so `\r\n` stays `\r\n`
  - `{ "base64": "…" }`: binary content
  - **an array**: rows for a JSON format
  - **an object**: for `dataset-validate`, mapping each file name to its content
- The outcome is one of:
  - `valid` with optional `errors`, for validate operations. When `valid` is true, or `errors`
    is given, the report must equal `{ valid, errors }` exactly, in order
  - `expected`, for infer, describe and load operations. The result must equal it exactly
  - `throws: true`, when the operation must raise

## Operations

| operation             | runs                                                                  | compared                                  |
| --------------------- | --------------------------------------------------------------------- | ----------------------------------------- |
| `file-describe`       | describe the file written from `data`                                 | `expected`: `{bytes, textual, integrity}` |
| `file-validate`       | validate a resource with the group's `integrity`                      | `valid`, `errors`                         |
| `table-infer-dialect` | infer the file dialect                                                | `expected`: the dialect                   |
| `table-infer-schema`  | infer the table schema                                                | `expected`: the schema                    |
| `table-load`          | load the table                                                        | `expected`: `{columns, rows}`             |
| `table-validate`      | validate the table                                                    | `valid`, `errors`                         |
| `dataset-validate`    | write the files and the `dataset` as `dataset.json`, then validate it | `valid`, `errors`                         |

## Runners

- TypeScript: `typescript/library/-test/conformance.unit.ts`, run by `pnpm -C typescript test:unit`
- Python: `python/library/fairspec_library/_test/conformance_unit.py`, run by `pnpm -C python test:unit`

Each runner:

1. writes `data` to a temporary file with the group's `format` as its extension
2. calls the library on a resource pointing at that file
3. serialises the result to JSON, with no further normalisation, and compares

A difference in number type (`1` against `1.0`), key presence or order is a failure. Fix it
in the library, never by loosening a runner.

Each file is also validated against `test-schema.json` by both runners.
