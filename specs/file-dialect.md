---
title: Fairspec File Dialect
label: File Dialect
path: /specs/file-dialect/
order: 3
---

<table>
  <tr>
    <th>Authors</th>
    <td>Evgeny Karev</td>
  </tr>
  <tr>
    <th>Profile</th>
    <td>
      <a href="https://fairspec.org/profiles/latest/file-dialect.json">https://fairspec.org/profiles/latest/file-dialect.json</a>
    </td>
  </tr>
</table>

Fairspec File Dialect is a simple [JSON](https://json.org)-based format that defines File Dialect to describe a file's format options and features. When expedient, this specification aims to be structurally compatible with the [Data Package](https://datapackage.org/standard/data-package/) standard.

## Language

The key words `MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `RECOMMENDED`, `MAY`, and `OPTIONAL` in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## Descriptor

A Fairspec File Dialect is a [JSON](https://json.org/) resource that `MUST` be an object compatible with the [File Dialect](#file-dialect) structure outlined below.

## File Dialect

A top-level descriptor object describing a file dialect. It `MIGHT` have the following properties (all optional unless otherwise stated):

### `$schema` [#profile]

> [!NOTE]
> Data publishers `SHOULD` provide this property with an exact version when sharing a standalone descriptor publicly.

[External Path](#external-path) to one of the officially published Fairspec File Dialect profiles with default value `https://fairspec.org/profiles/latest/file-dialect.json`.

For example for version X.Y.Z of the profile:

```json
{
  "$schema": "https://fairspec.org/profiles/X.Y.Z/file-dialect.json"
}
```

### `format`

The file format of the dialect. It `MUST` be a `string`.

For example for a CSV file:

```json
{
  "fileDialect": {
    "format": "csv"
  }
}
```

### `title`

An optional human-readable title for the format.

For example:

```json
{
  "fileDialect": {
    "title": "My custom format"
  }
}
```

### `description`

An optional detailed description of the format.

For example:

```json
{
  "fileDialect": {
    "title": "My custom format",
    "description": "You can open this file with OpenOffice"
  }
}
```

### File Formats

#### CSV

A format for comma-separated values files. It `MUST` have [`format`](#format) set to `"csv"`. It `MUST` be `utf-8` encoded. Empty cells (`,,`) are `null` values.

Metadata example:

```json
{
  "fileDialect": {
    "format": "csv"
  }
}
```

Data example:

```csv
name,age,city
Alice,30,New York
Bob,25,London
Charlie,35,Tokyo
```

Format properties:

- [`delimiter`](#delimiter)
- [`lineTerminator`](#lineterminator)
- [`quoteChar`](#quotechar)
- [`nullSequence`](#nullsequence)
- [`headerRows`](#headerrows)
- [`headerJoin`](#headerjoin)
- [`commentRows`](#commentrows)
- [`commentPrefix`](#commentprefix)
- [`columnNames`](#columnnames)

#### TSV

A format for tab-separated values files. It `MUST` have [`format`](#format) set to `"tsv"`. It `MUST` be `utf-8` encoded. Empty cells (`,,`) are `null` values.

Metadata example:

```json
{
  "fileDialect": {
    "format": "tsv",
    "nullSequence": "\N"
  }
}
```

Data example:

```tsv
name	age	city
Alice	30	New York
Bob	25	London
Charlie	35	Tokyo
```

Format properties:

- [`lineTerminator`](#lineterminator)
- [`nullSequence`](#nullsequence)
- [`headerRows`](#headerrows)
- [`headerJoin`](#headerjoin)
- [`commentRows`](#commentrows)
- [`commentPrefix`](#commentprefix)
- [`columnNames`](#columnnames)

#### JSON

A format for JSON array files. It `MUST` have [`format`](#format) set to `"json"`.

Metadata example:

```json
{
  "fileDialect": {
    "format": "json",
    "jsonPointer": "/data/items"
  }
}
```

Data example:

```json
[
  { "name": "Alice", "age": 30, "city": "New York" },
  { "name": "Bob", "age": 25, "city": "London" },
  { "name": "Charlie", "age": 35, "city": "Tokyo" }
]
```

Format properties:

- [`headerRows`](#headerrows)
- [`headerJoin`](#headerjoin)
- [`commentRows`](#commentrows)
- [`commentPrefix`](#commentprefix)
- [`columnNames`](#columnnames)
- [`jsonPointer`](#jsonpointer)
- [`rowType`](#rowtype)

#### JSONL

A format for JSON Lines files (newline-delimited JSON). It `MUST` have [`format`](#format) set to `"jsonl"`.

Metadata example:

```json
{
  "fileDialect": {
    "format": "jsonl",
    "rowType": "object"
  }
}
```

Data example:

```jsonl
{"name": "Alice", "age": 30, "city": "New York"}
{"name": "Bob", "age": 25, "city": "London"}
{"name": "Charlie", "age": 35, "city": "Tokyo"}
```

Format properties:

- [`headerRows`](#headerrows)
- [`headerJoin`](#headerjoin)
- [`commentRows`](#commentrows)
- [`commentPrefix`](#commentprefix)
- [`columnNames`](#columnnames)
- [`rowType`](#rowtype)

#### XLSX

A format for Microsoft Excel files. It `MUST` have [`format`](#format) set to `"xlsx"`. Empty cells are `null` values.

Metadata example:

```json
{
  "fileDialect": {
    "format": "xlsx",
    "sheetNumber": 2
  }
}
```

Data example:

```
<binary data>
```

Format properties:

- [`headerRows`](#headerrows)
- [`headerJoin`](#headerjoin)
- [`commentRows`](#commentrows)
- [`commentPrefix`](#commentprefix)
- [`columnNames`](#columnnames)
- [`sheetName`](#sheetname)
- [`sheetNumber`](#sheetnumber)

#### ODS

A format for OpenDocument Spreadsheet files. It `MUST` have [`format`](#format) set to `"ods"`. Empty cells are `null` values.

Metadata example:

```json
{
  "fileDialect": {
    "format": "ods",
    "sheetName": "Data Sheet"
  }
}
```

Data example:

```
<binary data>
```

Format properties:

- [`headerRows`](#headerrows)
- [`headerJoin`](#headerjoin)
- [`commentRows`](#commentrows)
- [`commentPrefix`](#commentprefix)
- [`columnNames`](#columnnames)
- [`sheetName`](#sheetname)
- [`sheetNumber`](#sheetnumber)

#### Parquet

A format for Apache Parquet files. It `MUST` have [`format`](#format) set to `"parquet"`.

Metadata example:

```json
{
  "fileDialect": {
    "format": "parquet"
  }
}
```

Data example:

```
<binary data>
```

#### Arrow

A format for Apache Arrow files. It `MUST` have [`format`](#format) set to `"arrow"`.

Metadata example:

```json
{
  "fileDialect": {
    "format": "arrow"
  }
}
```

Data example:

```
<binary data>
```

#### SQLite

A format for SQLite database files. It `MUST` have [`format`](#format) set to `"sqlite"`.

Metadata example:

```json
{
  "fileDialect": {
    "format": "sqlite"
  }
}
```

Data example:

```
<binary data>
```

Format properties:

- [`tableName`](#tablename)

#### Unknown

A format for custom data. It `MUST` have [`format`](#type) not supported by the formats above.

Metadata example:

```json
{
  "format": {
    "format": "custom",
    "title": "Custom format",
    "description": "Custom format description"
  }
}
```

Data example:

```
<binary data>
```

### Format Properties

#### `name`

> [!NOTE]
> Supported formats: **custom**

The name of the custom format. It `MUST` be a string.

For example:

```json
{
  "fileDialect": {
    "name": "custom",
    "title": "Custom format",
    "description": "Custom format description"
  }
}
```

#### `delimiter`

> [!NOTE]
> Supported formats: **csv**

It `MUST` be a string of one character length. This property specifies the character sequence which separates fields in the data file.

For example:

```json
{
  "fileDialect": {
    "format": "csv",
    "delimiter": ";"
  }
}
```

For a file like:

```
id;name;price
1;apple;1.50
2;orange;2.00
```

#### `lineTerminator`

> [!NOTE]
> Supported formats: **csv**, **tsv**

It `MUST` be a string. This property specifies the character sequence which terminates rows in the file. Common values are `\n` (Unix), `\r\n` (Windows), `\r` (old Mac).

For example:

```json
{
  "fileDialect": {
    "format": "csv",
    "lineTerminator": "\r\n"
  }
}
```

#### `quoteChar`

> [!NOTE]
> Supported formats: **csv**

It `MUST` be a string of one character length. This property specifies a character to use for quoting in case the delimiter needs to be used inside a data cell.

For example:

```json
{
  "fileDialect": {
    "format": "csv",
    "quoteChar": "'"
  }
}
```

For a file like:

```
id,name
1,'apple,red'
2,'orange,citrus'
```

#### `nullSequence`

> [!NOTE]
> Supported formats: **csv**, **tsv**

It `MUST` be a string. This property specifies the null sequence representing missing values in the data.

For example:

```json
{
  "fileDialect": {
    "format": "csv",
    "nullSequence": "\N"
  }
}
```

For a file like:

```
id,name,notes
1,apple,fresh
2,orange,NA
3,banana,N/A
```

#### `headerRows`

> [!NOTE]
> Supported formats: **csv**, **tsv**, **json**, **jsonl**, **xlsx**, **ods**

It `MUST` be `false` or an array of positive integers starting from `1`. This property specifies the row numbers for the header.

For example with a single header row:

```json
{
  "fileDialect": {
    "format": "csv",
    "headerRows": [1]
  }
}
```

For example with multi-line headers:

```json
{
  "fileDialect": {
    "format": "csv",
    "headerRows": [1, 2]
  }
}
```

For a file like:

```
fruit
id,name,price
1,apple,1.50
2,orange,2.00
```

This would produce headers: "fruit id", "fruit name", "fruit price"

For example with no headers:

```json
{
  "fileDialect": {
    "format": "csv",
    "headerRows": false
  }
}
```

#### `headerJoin`

> [!NOTE]
> Supported formats: **csv**, **tsv**, **json**, **jsonl**, **xlsx**, **ods**

It `MUST` be a string. This property specifies how multiline-header files have to join the resulting header rows.

For example:

```json
{
  "fileDialect": {
    "format": "csv",
    "headerRows": [0, 1],
    "headerJoin": "_"
  }
}
```

For a file like:

```
fruit
id,name,price
1,apple,1.50
```

This would produce headers: "fruit_id", "fruit_name", "fruit_price"

#### `commentRows`

> [!NOTE]
> Supported formats: **csv**, **tsv**, **json**, **jsonl**, **xlsx**, **ods**

It `MUST` be an array of positive integers starting from `1`. This property specifies what rows have to be omitted from the data.

For example:

```json
{
  "fileDialect": {
    "format": "csv",
    "commentRows": [1, 5, 10]
  }
}
```

For a file like:

```
id,name
# This is a comment row
1,apple
2,orange
```

With `"commentRows": [2]`, the second row would be skipped.

#### `commentPrefix`

> [!NOTE]
> Supported formats: **csv**, **tsv**, **json**, **jsonl**, **xlsx**, **ods**

It `MUST` be a string. This property specifies what rows have to be omitted from the data based on the row's prefix.

For example:

```json
{
  "fileDialect": {
    "format": "csv",
    "commentPrefix": "#"
  }
}
```

For a file like:

```
id,name
# This row is ignored
1,apple
# Another comment
2,orange
```

Rows starting with `#` will be skipped.

#### `columnNames`

> [!NOTE]
> Supported formats: **csv**, **tsv**, **json**, **jsonl**, **xlsx**, **ods**

It `MUST` be an array of strings. This property specifies explicit column names to use instead of deriving them from the file.

For example:

```json
{
  "fileDialect": {
    "format": "csv",
    "headerRows": false,
    "columnNames": ["id", "name", "price"]
  }
}
```

For a file without headers:

```
1,apple,1.50
2,orange,2.00
```

#### `jsonPointer`

> [!NOTE]
> Supported formats: **json**

It `MUST` be a string in [JSON Pointer format (RFC 6901)](https://datatracker.ietf.org/doc/html/rfc6901). This property specifies where a data is located in the document.

For example:

```json
{
  "fileDialect": {
    "format": "json",
    "jsonPointer": "/data/items"
  }
}
```

For a JSON file like:

```json
{
  "metadata": { "version": "1.0" },
  "data": {
    "items": [
      { "id": 1, "name": "apple" },
      { "id": 2, "name": "orange" }
    ]
  }
}
```

#### `rowType`

> [!NOTE]
> Supported formats: **json**, **jsonl**

It `MUST` be one of the following values: `array`, `object`. This property specifies whether the data items are arrays or objects.

For example with array of objects:

```json
{
  "fileDialect": {
    "format": "json",
    "rowType": "object"
  }
}
```

For data like:

```json
[
  { "id": 1, "name": "apple" },
  { "id": 2, "name": "orange" }
]
```

For example with array of arrays:

```json
{
  "fileDialect": {
    "format": "json",
    "rowType": "array",
    "columnNames": ["id", "name"]
  }
}
```

For data like:

```json
[
  [1, "apple"],
  [2, "orange"]
]
```

#### `sheetNumber`

> [!NOTE]
> Supported formats: **xlsx**, **ods**

It `MUST` be an integer. This property specifies a sheet number of a table in the spreadsheet file. If not provided, a first sheet is used.

For example:

```json
{
  "fileDialect": {
    "format": "xlsx",
    "sheetNumber": 2
  }
}
```

This reads the second sheet from the spreadsheet.

#### `sheetName`

> [!NOTE]
> Supported formats: **xlsx**, **ods**

It `MUST` be a string. This property specifies a sheet name of a table in the spreadsheet file.

For example:

```json
{
  "fileDialect": {
    "format": "xlsx",
    "sheetName": "Data Sheet"
  }
}
```

#### `tableName`

> [!NOTE]
> Supported formats: **sqlite**

It `MUST` be a string. This property specifies a name of the table in the database. If not provided, a first table is used (sorted by name in ascending order).

For example:

```json
{
  "fileDialect": {
    "format": "sqlite",
    "tableName": "measurements"
  }
}
```

## Common

Common properties shared by multiple entities in the descriptor.

### External Path

It `MUST` be a string representing an HTTP or HTTPS URL to a remote file.

For example:

```json
{
  "data": "https://example.com/datasets/measurements.csv"
}
```

## Extension

> [!WARNING]
> Additional properties are NOT allowed.

Fairspec File Dialect does not support extension.

## Comparison

> [!NOTE]
> This section is under development.
