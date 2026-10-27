---
title: Fairspec Table Schema
label: Table Schema
path: /specs/table-schema/
order: 5
---

<table>
  <tr>
    <th>Authors</th>
    <td>Evgeny Karev</td>
  </tr>
  <tr>
    <th>Profile</th>
    <td>
      <a href="https://fairspec.org/profiles/latest/table-schema.json">https://fairspec.org/profiles/latest/table-schema.json</a>
    </td>
  </tr>
</table>

Fairspec Table Schema is a simple [JSON](https://json.org)-based format that defines Table Schema to describe a class of tabular data resources. A Table Schema is a [JSON Schema 2020-12](https://json-schema.org/draft/2020-12) schema of one row, restricted to the [subset](#json-schema) that describes one typed column per property. When expedient, this specification aims to be structurally compatible with the [Data Package](https://datapackage.org/standard/table-schema/) standard.

## Language

The key words `MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `RECOMMENDED`, `MAY`, and `OPTIONAL` in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## Descriptor

A Fairspec Table Schema is a [JSON](https://json.org/) resource that `MUST` be an object compatible with the [Table Schema](#table-schema) structure outlined below.

## Table Schema

A top-level descriptor object defining a schema of tabular data resources. It `MAY` have the following properties (all optional unless otherwise stated):

### `$schema` [#profile]

> [!NOTE]
> Data publisher `SHOULD` provide this property with an exact version when sharing a standalone descriptor publicly.

[External Path](#external-path) to one of the officially published Fairspec Table Schema profiles or `https://json-schema.org/draft/2020-12/schema` with default value `https://fairspec.org/profiles/latest/table-schema.json`. A Table Schema declaring the JSON Schema dialect `MUST` be validated as one declaring the default profile.

For example for version X.Y.Z of the profile:

```json
{
  "$schema": "https://fairspec.org/profiles/X.Y.Z/table-schema.json"
}
```

### `title`

An optional human-readable title for the table schema. It `MUST` be a string.

For example:

```json
{
  "title": "Experimental Measurements Dataset"
}
```

### `description`

An optional detailed description explaining the purpose and contents of the table schema. It `MUST` be a string.

For example:

```json
{
  "title": "Experimental Measurements Dataset",
  "description": "Temperature and pressure measurements collected during chemical reaction experiments performed in laboratory conditions."
}
```

### `properties`

An object defining the schema for table columns. Each key represents a column name, and its value `MUST` be a valid [Column](#column) definition.

A key matches a column of the data whose name is equal to it character for character: names are case-sensitive and never trimmed, so `ID` in the data is not the column `id`. The same holds for every column name this specification lists, such as in [`required`](#required) or the keys.

For example, for a simple table with different column types:

```json
{
  "properties": {
    "id": {
      "type": "integer",
      "minimum": 1
    },
    "name": {
      "type": "string",
      "maxLength": 100
    },
    "email": {
      "type": "string",
      "format": "email"
    },
    "active": {
      "type": "boolean"
    }
  }
}
```

### `required`

An optional list of column names that are required to present. Each item `MUST` be a string matching a key in the [`properties`](#properties) object.

For example, to require specific columns:

```json
{
  "required": ["experiment_id", "temperature", "pressure"],
  "properties": {
    "experiment_id": {
      "type": "integer"
    },
    "temperature": {
      "type": "number"
    },
    "pressure": {
      "type": "number"
    },
    "notes": {
      "type": "string"
    }
  }
}
```

### `allRequired`

An optional boolean flag indicating whether all columns defined in [`properties`](#properties) are required. When set to `true`, every column `MUST` be present in the data. It `MUST` be a boolean value.

For example, to require all columns:

```json
{
  "allRequired": true,
  "properties": {
    "experiment_id": {
      "type": "integer"
    },
    "temperature": {
      "type": "number"
    },
    "pressure": {
      "type": "number"
    }
  }
}
```

### `primaryKey`

An optional array of column names that form the table's primary key. The combination of values in these columns `MUST` uniquely identify each row. At least one column name `MUST` be specified if this property is present, and every name `MUST` be a column in [`properties`](#properties).

A primary key column `MUST` be present in the data, as if it were [`required`](#required), and its cells `MUST NOT` be null, even where the column's `type` allows `null`.

Keys compare the typed values of their cells, as parsed by the column's type, not the text the file holds: in a number column `1.0` and `1` are the same value, and in a [DateTime](#datetime) column `2024-01-01T12:00:00+02:00` and `2024-01-01T10:00:00Z` are. This holds for [`primaryKey`](#primarykey), [`uniqueKeys`](#uniquekeys) and [`foreignKeys`](#foreignkeys) alike.

For example, with a single-column primary key:

```json
{
  "primaryKey": ["experiment_id"],
  "properties": {
    "experiment_id": {
      "type": "integer"
    },
    "measurement": {
      "type": "number"
    }
  }
}
```

For example, with a composite primary key:

```json
{
  "primaryKey": ["sample_id", "timestamp"],
  "properties": {
    "sample_id": {
      "type": "string"
    },
    "timestamp": {
      "type": "string",
      "format": "date-time"
    },
    "value": {
      "type": "number"
    }
  }
}
```

### `uniqueKeys`

An optional array of unique key constraints. Each unique key is an array of column names whose combined values `MUST` be unique across all rows in the table. Each unique key array `MUST` contain at least one column name, and every name `MUST` be a column in [`properties`](#properties).

A row in which any of the key's columns is null is not compared, as in SQL: such rows never conflict with each other or with any other row. Values are compared as typed values, as for [`primaryKey`](#primarykey).

For example, with multiple unique constraints:

```json
{
  "uniqueKeys": [["email"], ["username"], ["department", "employee_number"]],
  "properties": {
    "email": {
      "type": "string",
      "format": "email"
    },
    "username": {
      "type": "string"
    },
    "department": {
      "type": "string"
    },
    "employee_number": {
      "type": "integer"
    }
  }
}
```

### `foreignKeys`

An optional array of foreign key constraints that define relationships between this table and other tables. Each foreign key specifies local `columns` and their `reference` to columns in another resource (identified by resource [`name`](../dataset#name) in the dataset context). Both `columns` lists `MUST` name at least one column, and they `MUST` have the same length: the n-th local column refers to the n-th referenced column. Every local column `MUST` be a column in [`properties`](#properties).

A row in which any of the local `columns` is null is not checked against the reference, as SQL's `MATCH SIMPLE`: a composite key with one null part is skipped whatever its other parts hold. Every other row `MUST` have its `columns` values present in the referenced `columns` of some row of the referenced resource, compared as typed values, as for [`primaryKey`](#primarykey). The referenced values need not be unique: a value present in several referenced rows satisfies the key.

Each local column and the column it refers to `MUST` have the same [column type](#column-types), a `null` in either column's `type` array aside (`["integer", "null"]` matches `"integer"`), with one exception: [Integer](#integer), [Number](#number) and [Decimal](#decimal) columns `MAY` refer to one another, and their values compare by numeric value, so `1`, `1.0` and `1.00` are the same key. Any other pair of differing types `MUST` be reported as a metadata error at `/foreignKeys/<i>`, the foreign key's position in the array, and that foreign key is not checked against the data. Columns of the same type compare their typed values, as above.

The referenced resource `MAY` be any table resource of the dataset, whether its data is a file, inline or multipart. A `reference.resource` naming a resource that is not a table, such as one described by a [Data Schema](../data-schema), `MUST` be reported as a metadata error.

For example, referencing another resource:

```json
{
  "foreignKeys": [
    {
      "columns": ["customer_id"],
      "reference": {
        "resource": "customers",
        "columns": ["id"]
      }
    }
  ],
  "properties": {
    "order_id": {
      "type": "integer"
    },
    "customer_id": {
      "type": "integer"
    },
    "amount": {
      "type": "number"
    }
  }
}
```

For example, with a self-reference (omitting `resource`):

```json
{
  "foreignKeys": [
    {
      "columns": ["parent_id"],
      "reference": {
        "columns": ["id"]
      }
    }
  ],
  "properties": {
    "id": {
      "type": "integer"
    },
    "parent_id": {
      "type": "integer"
    },
    "name": {
      "type": "string"
    }
  }
}
```

For example, with a composite foreign key:

```json
{
  "foreignKeys": [
    {
      "columns": ["supplier_id", "product_code"],
      "reference": {
        "resource": "catalog",
        "columns": ["supplier_id", "code"]
      }
    }
  ],
  "properties": {
    "supplier_id": {
      "type": "integer"
    },
    "product_code": {
      "type": "string"
    }
  }
}
```

### `additionalProperties`

An optional boolean stating whether the data `MAY` have columns that [`properties`](#properties) does not list. It defaults to `true`. When `false`, every such column is an error. A schema value, which JSON Schema allows, is not supported and `MUST` be rejected.

For example, to allow only the listed columns:

```json
{
  "additionalProperties": false,
  "properties": {
    "id": {
      "type": "integer"
    },
    "name": {
      "type": "string"
    }
  }
}
```

### `missingValues`

> [!NOTE]
> Table Schema level missing values works in addition to [File Dialect](../file-dialect#nullsequence) null sequences. For example, CSV Format treats no values between commas (`,,`) as `null` so there is no need to provide `''` as a Table Schema missing value in this case; a quoted `""` stays an empty string.

An optional list of values that represent missing or null data across all columns in the table. Each item can be either a simple value or an object with `value` and `label` properties for documentation purposes. The values type `MUST` be `"string"` or `"integer"`.

A cell is missing when it equals an item literally, before any type conversion and without coercion: a string matches a cell the format delivers as text, character for character, and an integer matches a cell the format delivers as a number. A text format such as CSV delivers every cell as text, so its missing values are written as strings (`"-999"`, not `-999`). A column's own [`missingValues`](#column-missingvalues) apply in addition to these.

For example, with simple values:

```json
{
  "missingValues": ["NA", "N/A", "", -999]
}
```

For example, with labeled values:

```json
{
  "missingValues": [
    { "value": "NA", "label": "Not Available" },
    { "value": "NR", "label": "Not Recorded" },
    { "value": -999, "label": "Sensor Error" }
  ]
}
```

For example, with mixed values:

```json
{
  "missingValues": ["NA", { "value": -999, "label": "Sensor Error" }]
}
```

## Column

A column definition that specifies the data type, constraints, and metadata for a table column. The schema is routed based on the [`type`](#type) property and optionally the [`format`](#format) property to determine which specific column type applies.

#### `type`

The data type of the column. It `MUST` be one of the following values:

- `boolean` - True/false values
- `integer` - Whole numbers
- `number` - Numeric values
- `string` - Text values
- `array` - Array/list values
- `object` - Object/dictionary values

As in JSON Schema, the type `MAY` also be an array of these values with unique items, in any order. A column allows missing values when the array includes `null`:

- `"integer"` and `["integer"]` are the same type
- `["integer", "null"]` and `["null", "integer"]` - Whole numbers or missing values
- `["integer", "number"]`, with or without `null`, is `number`, since every integer is a number

Any other array, holding two or more of the values above, `MUST` be rejected. A column without `type` is [Unknown](#unknown).

Metadata example:

```json
{
  "properties": {
    "age": {
      "type": "integer"
    },
    "title": {
      "type": ["string", "null"]
    }
  }
}
```

Data example:

```csv
age
25
32
18
```

#### `title` [#column-title]

An optional human-readable title for the column. It `MUST` be a string.

Metadata example:

```json
{
  "properties": {
    "temp_c": {
      "type": "number",
      "title": "Temperature (Celsius)"
    }
  }
}
```

Data example:

```csv
temp_c
23.5
-10.2
98.6
```

#### `description` [#column-description]

An optional detailed description of the column. It `MUST` be a string.

Metadata example:

```json
{
  "properties": {
    "pressure": {
      "type": "number",
      "description": "Atmospheric pressure measured in hectopascals (hPa) at the time of observation."
    }
  }
}
```

Data example:

```csv
pressure
1013.25
1020.50
995.30
```

#### `rdfType`

An optional property that provides a richer, "semantic" description of the type of data in a column. The value `MUST` be the URI of a RDF Class, that is an instance or subclass of [RDF Schema Class object](https://www.w3.org/TR/rdf-schema/#ch_class).

Metadata example:

```json
{
  "properties": {
    "country": {
      "type": "string",
      "rdfType": "http://schema.org/Country"
    }
  }
}
```

Data example:

```csv
country
US
UK
DE
FR
```

#### `enum`

An optional array of allowed values for the column. The values `MUST` match the column's type.

For example, with string values:

```json
{
  "properties": {
    "status": {
      "type": "string",
      "enum": ["pending", "active", "completed", "cancelled"]
    }
  }
}
```

For example, with integer values:

```json
{
  "properties": {
    "priority": {
      "type": "integer",
      "enum": [1, 2, 3, 4, 5]
    }
  }
}
```

Data example:

```csv
status
pending
active
completed
cancelled
```

#### `const`

An optional constant value for the column. The value `MUST` match the column's type.

For example, with string values:

```json
{
  "properties": {
    "status": {
      "type": "string",
      "const": "pending"
    }
  }
}
```

For example, with integer values:

```json
{
  "properties": {
    "priority": {
      "type": "integer",
      "const": 1
    }
  }
}
```

Data example:

```csv
status
pending
pending
```

#### `default`

An optional default value for the column. The value `MUST` match the column's type. This property is for documentation purpose and it is not used to fill missing values.

Metadata example:

```json
{
  "properties": {
    "status": {
      "type": "string",
      "default": "pending",
      "missingValues": ["N/A"]
    }
  }
}
```

Data example:

```csv
status
done
N/A
```

#### `examples`

An optional array of example values for the column. The values `MUST` match the column's type and can be used for documentation, testing, or generating sample data.

Metadata example:

```json
{
  "properties": {
    "temperature": {
      "type": "number",
      "examples": [20.5, 25.3, 18.7]
    }
  }
}
```

Data example:

```csv
temperature
20.5
25.3
18.7
```

#### `missingValues` [#column-missingvalues]

An optional column-level list of values that represent missing or null data for this column. Each item can be either a simple value or an object with `value` and `label` properties for documentation purposes. The missing values type `MUST` be:

- `"string"` or `"integer"` for boolean, integer, and number columns
- `"string"` for all other columns

If table-level missing values are provided, the effective missing values `MUST` include all the column-level values and all the compatible table-level values.

Metadata example:

```json
{
  "properties": {
    "measurement": {
      "type": "number",
      "missingValues": [
        { "value": -999, "label": "Sensor malfunction" },
        { "value": "NA", "label": "Not measured" }
      ]
    }
  }
}
```

Data example:

```csv
measurement
25.3
-999
NA
42.1
```

### Column Types

#### Boolean

A column for true/false values. It `MUST` have [`type`](#type) set to `"boolean"` and `MUST NOT` have a [`format`](#format) property.

Metadata example:

```json
{
  "properties": {
    "is_active": {
      "type": "boolean"
    }
  }
}
```

Data example:

```csv
is_active
true
false
true
```

Type properties:

- [`trueValues`](#truevalues)
- [`falseValues`](#falsevalues)

#### Categorical

A column for categorical values. It `MUST` have [`type`](#type) set to `"integer"` or `"string"` and [`format`](#format) set to `"categorical"`.

A value is read as a value of its `type`: an integer categorical column reads its cells as an [Integer](#integer) column does, and a string categorical column matches a cell against its [`categories`](#categories) exactly, character for character, without trimming or case folding. The constraints below apply to every value in addition to `categories`, so a listed category that fails one, such as a category below [`minimum`](#minimum), is an error.

Metadata example:

```json
{
  "properties": {
    "severity": {
      "type": "integer",
      "format": "categorical",
      "categories": [
        { "value": 1, "label": "Low" },
        { "value": 2, "label": "Medium" },
        { "value": 3, "label": "High" }
      ]
    }
  }
}
```

Data example:

```csv
severity
1
2
3
1
```

Type properties:

- [`categories`](#categories)
- [`withOrder`](#withorder)

In addition, type properties if type is `"integer"`:

- [`minimum`](#minimum)
- [`maximum`](#maximum)
- [`exclusiveMinimum`](#exclusiveminimum)
- [`exclusiveMaximum`](#exclusivemaximum)
- [`multipleOf`](#multipleof)
- [`groupChar`](#groupchar)
- [`withText`](#withtext)

In addition, type properties if type is `"string"`:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### Integer

A column for whole number values. It `MUST` have [`type`](#type) set to `"integer"` and `MUST NOT` have a [`format`](#format) property.

A cell delivered as text `MUST` be a number in the grammar of a [Number](#number) column whose fraction is zero, as an integer is in [JSON Schema 2020-12](https://json-schema.org/draft/2020-12/json-schema-validation#section-6.1.1): `1.0` and `1e3` are integers, `1.5` and `1e-1` are not.

An integer value `MUST` lie in the signed 64-bit range, from `-9223372036854775808` to `9223372036854775807`; a value outside it is a `cell/type` error. This is a deliberate deviation from JSON Schema, whose integers are unbounded, so that every implementation holds the same values exactly: a whole number that needs more digits belongs in a [Decimal](#decimal) column. The range applies to every column of type `"integer"`, including an integer [Categorical](#categorical) one.

Metadata example:

```json
{
  "properties": {
    "age": {
      "type": "integer"
    }
  }
}
```

Data example:

```csv
age
25
32
18
```

Type properties:

- [`minimum`](#minimum)
- [`maximum`](#maximum)
- [`exclusiveMinimum`](#exclusiveminimum)
- [`exclusiveMaximum`](#exclusivemaximum)
- [`multipleOf`](#multipleof)
- [`groupChar`](#groupchar)
- [`withText`](#withtext)

#### Number

A column for numeric values including decimals. It `MUST` have [`type`](#type) set to `"number"` and `MUST NOT` have a [`format`](#format) property.

A cell delivered as text `MUST` be a JSON number, as [RFC 8259](https://datatracker.ietf.org/doc/html/rfc8259#section-6) section 6 defines it, once these have been applied, in this order: [`withText`](#withtext) strips the text around the number, every [`groupChar`](#groupchar) is removed, and [`decimalChar`](#decimalchar) is read as `.`. The grammar is an optional minus sign and never a plus sign, an integer part without leading zeros (`0` alone is allowed), an optional fraction of a `.` with digits on both sides, and an optional exponent of `e` or `E`, an optional sign and digits. A value `MUST NOT` contain whitespace, and `NaN`, `Infinity` and their variants are not numbers. So `0.5`, `-2` and `1E-3` are numbers, and `+1`, `007`, `.5`, `5.`, ` 1` and `0x10` are not. `-0` is a number equal to `0`.

Metadata example:

```json
{
  "properties": {
    "temperature": {
      "type": "number"
    }
  }
}
```

Data example:

```csv
temperature
23.5
-10.2
98.6
```

Type properties:

- [`minimum`](#minimum)
- [`maximum`](#maximum)
- [`exclusiveMinimum`](#exclusiveminimum)
- [`exclusiveMaximum`](#exclusivemaximum)
- [`multipleOf`](#multipleof)
- [`decimalChar`](#decimalchar)
- [`groupChar`](#groupchar)
- [`withText`](#withtext)

#### Decimal

A column for decimal values. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"decimal"`.

Metadata example:

```json
{
  "properties": {
    "price": {
      "type": "string",
      "format": "decimal"
    }
  }
}
```

Data example:

```csv
price
19.99
5.50
123.45
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)
- [`minimum`](#minimum)
- [`maximum`](#maximum)
- [`exclusiveMinimum`](#exclusiveminimum)
- [`exclusiveMaximum`](#exclusivemaximum)
- [`multipleOf`](#multipleof)
- [`decimalChar`](#decimalchar)
- [`groupChar`](#groupchar)
- [`withText`](#withtext)

#### String

A column for text values. It `MUST` have [`type`](#type) set to `"string"` and `MUST NOT` have a [`format`](#format) property.

Metadata example:

```json
{
  "properties": {
    "name": {
      "type": "string"
    }
  }
}
```

Data example:

```csv
name
Alice
Bob
Charlie
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### List

A column for delimited list values stored as strings. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"list"`.

A cell is split at every [`delimiter`](#delimiter), literally: there is no quoting or escaping inside a cell, and items are not trimmed. So `a,,b` has three items, the second empty, `a,b,` has a trailing empty item, and ` a` is an item with a leading space. An empty cell, such as a quoted `""` in CSV, is an empty list. Each item is then read as an [`itemType`](#itemtype) value.

Since the column is of type `"string"`, as in JSON Schema [`minLength`](#minlength), [`maxLength`](#maxlength) and [`pattern`](#pattern) apply to the cell's whole text before it is split, and [`minItems`](#minitems) and [`maxItems`](#maxitems) to the number of items. Missing values match the whole cell only, never an item: with `"NA"` a missing value, the cell `NA` is null and the cell `a,NA` is a list of two items.

Metadata example:

```json
{
  "properties": {
    "tags": {
      "type": "string",
      "format": "list"
    }
  }
}
```

Data example:

```csv
tags
"red,blue,green"
"small,compact"
"new,sale,featured"
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)
- [`delimiter`](#delimiter)
- [`itemType`](#itemtype)
- [`minItems`](#minitems)
- [`maxItems`](#maxitems)

#### URL

A column for URLs with HTTP/HTTPS protocol. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"url"`.

A value `MUST` match the `URI` production of [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986#section-3), so it has a scheme and is never a relative reference, and it `MAY` end in a `#` fragment. In addition: its scheme `MUST` be `http` or `https`, in any case (`HTTPS://example.com` is valid), its host `MUST NOT` be empty (`http:///path` is invalid), and it `MUST` consist of ASCII characters only, so a non-ASCII character is written percent-encoded.

Metadata example:

```json
{
  "properties": {
    "homepage": {
      "type": "string",
      "format": "url"
    }
  }
}
```

Data example:

```csv
homepage
https://example.com
https://example.org/page
https://domain.net/path/to/resource
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### Email

A column for email addresses. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"email"`.

A value `MUST` match the `Mailbox` production of [RFC 5321](https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.2): a local part, as a dot-string or a quoted string (`"john doe"@example.com`), then `@` and a domain or an address literal (`user@[192.0.2.1]`). A domain of a single label, such as `user@localhost`, is valid. A value with a non-ASCII character is invalid, as RFC 5321 allows none.

Metadata example:

```json
{
  "properties": {
    "contact_email": {
      "type": "string",
      "format": "email"
    }
  }
}
```

Data example:

```csv
contact_email
alice@example.com
bob@company.org
charlie@domain.net
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### Date

A column for ISO 8601 calendar dates. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"date"`, as the JSON Schema `date` format. A date is not associated with any time zone.

Without [`temporalFormat`](#temporalformat), a value `MUST` be a calendar date `YYYY-MM-DD` with nothing before or after it. Its fields follow RFC 3339 as the [default forms](#temporalformat) set out, so the year `0000` is valid.

Metadata example:

```json
{
  "properties": {
    "birth_date": {
      "type": "string",
      "format": "date"
    }
  }
}
```

Data example:

```csv
birth_date
2023-12-01
1990-06-15
2005-03-20
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)
- [`temporalFormat`](#temporalformat)

#### Time

A column for times of day with a UTC offset, as the RFC 3339 `full-time` and the JSON Schema `time` format. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"time"`. For times of day without an offset, use [LocalTime](#localtime).

Without [`temporalFormat`](#temporalformat), a value `MUST` be `HH:MM:SS`, optionally followed by fractional seconds (`.` and one or more digits), and then by `Z` or a `+HH:MM` or `-HH:MM` offset, with nothing before or after it. A value without an offset is invalid. Its fields and offset follow RFC 3339 as the [default forms](#temporalformat) set out, so a lowercase `z` is valid in place of `Z`.

A value denotes a time of day in UTC: an implementation `MUST` apply the offset, so `14:30:00+02:00` and `12:30:00Z` are the same value.

Metadata example:

```json
{
  "properties": {
    "start_time": {
      "type": "string",
      "format": "time"
    }
  }
}
```

Data example:

```csv
start_time
14:30:00Z
09:45:30+02:00
18:00:00.5-05:00
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)
- [`temporalFormat`](#temporalformat)

#### LocalTime

A column for times of day without a UTC offset. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"local-time"`. A local time is not associated with any time zone, including the reader's: an implementation `MUST NOT` convert it to UTC.

Without [`temporalFormat`](#temporalformat), a value `MUST` be `HH:MM:SS`, optionally followed by fractional seconds (`.` and one or more digits), with nothing before or after it. A value with an offset is invalid. Its fields follow RFC 3339 as the [default forms](#temporalformat) set out.

Metadata example:

```json
{
  "properties": {
    "opening_time": {
      "type": "string",
      "format": "local-time"
    }
  }
}
```

Data example:

```csv
opening_time
09:00:00
13:30:00
17:45:30.25
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)
- [`temporalFormat`](#temporalformat)

#### DateTime

A column for instants: a date with a time of day and a UTC offset, as the RFC 3339 `date-time` and the JSON Schema `date-time` format. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"date-time"`. For dates with times without an offset, use [LocalDateTime](#localdatetime).

Without [`temporalFormat`](#temporalformat), a value `MUST` be `YYYY-MM-DDTHH:MM:SS`, optionally followed by fractional seconds (`.` and one or more digits), and then by `Z` or a `+HH:MM` or `-HH:MM` offset, with nothing before or after it. A value without an offset is invalid. Its fields, separators and offset follow RFC 3339 as the [default forms](#temporalformat) set out, so a lowercase `t` or `z` is valid in place of `T` or `Z`.

A value denotes an instant: an implementation `MUST` apply the offset, so `2024-03-20T18:00:00-05:00` and `2024-03-20T23:00:00Z` are the same value.

Metadata example:

```json
{
  "properties": {
    "created_at": {
      "type": "string",
      "format": "date-time"
    }
  }
}
```

Data example:

```csv
created_at
2023-12-01T14:30:00Z
2024-01-15T09:45:30+00:00
2024-03-20T18:00:00-05:00
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)
- [`temporalFormat`](#temporalformat)

#### LocalDateTime

A column for dates with times of day without a UTC offset. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"local-date-time"`. A local date-time is not associated with any time zone, including the reader's: an implementation `MUST NOT` convert it to an instant.

Without [`temporalFormat`](#temporalformat), a value `MUST` be `YYYY-MM-DDTHH:MM:SS`, optionally followed by fractional seconds (`.` and one or more digits), with nothing before or after it. A value with an offset is invalid. Its fields and separator follow RFC 3339 as the [default forms](#temporalformat) set out, so a lowercase `t` is valid in place of `T`.

Metadata example:

```json
{
  "properties": {
    "recorded_at": {
      "type": "string",
      "format": "local-date-time"
    }
  }
}
```

Data example:

```csv
recorded_at
2023-12-01T14:30:00
2024-01-15T09:45:30.125
2024-03-20T18:00:00
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)
- [`temporalFormat`](#temporalformat)

#### Duration

A column for ISO 8601 duration values. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"duration"`.

A value `MUST` match the `duration` production of [RFC 3339 Appendix A](https://datatracker.ietf.org/doc/html/rfc3339#appendix-A), letters in any case. So weeks stand alone (`P1W`, never `P1W2D`), every number is a whole number (`PT1.5S` is invalid), a value has no sign (`-P1D` is invalid), and `P` or `PT` with no component is invalid.

Metadata example:

```json
{
  "properties": {
    "elapsed_time": {
      "type": "string",
      "format": "duration"
    }
  }
}
```

Data example:

```csv
elapsed_time
PT1H30M
P1DT12H
PT45M30S
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### WKT

A column for Well-Known Text (WKT) geometry data. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"wkt"`.

Metadata example:

```json
{
  "properties": {
    "geometry": {
      "type": "string",
      "format": "wkt"
    }
  }
}
```

Data example:

```csv
geometry
"POINT (30 10)"
"LINESTRING (30 10, 10 30, 40 40)"
"POLYGON ((30 10, 40 40, 20 40, 10 20, 30 10))"
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### WKB

A column for Well-Known Binary (WKB) geometry data. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"wkb"`.

Metadata example:

```json
{
  "properties": {
    "geometry": {
      "type": "string",
      "format": "wkb"
    }
  }
}
```

Data example:

```csv
geometry
01010000000000000000003E400000000000002440
0102000000030000000000000000003E40000000000000244000000000000024400000000000003E4000000000000044400000000000004440
010300000001000000050000000000000000003E4000000000000024400000000000004440000000000000444000000000000034400000000000004440000000000000244000000000000034400000000000003E400000000000002440
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### Hex

A column for hexadecimal encoded data. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"hex"`.

A value `MUST` be an even number of hexadecimal digits, `0`–`9` and `a`–`f` in any case, two per byte. It `MUST NOT` have a `0x` prefix or any whitespace.

Metadata example:

```json
{
  "properties": {
    "color": {
      "type": "string",
      "format": "hex"
    }
  }
}
```

Data example:

```csv
color
FF5733
00BFFF
32CD32
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### Base64

A column for Base64 encoded binary data. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"base64"`.

A value `MUST` be Base64 in the standard alphabet of [RFC 4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4) section 4, with the `=` padding that makes its length a multiple of four. The URL-safe alphabet (`-` and `_`), a value without its padding and a value with whitespace or line breaks are invalid.

Metadata example:

```json
{
  "properties": {
    "thumbnail": {
      "type": "string",
      "format": "base64"
    }
  }
}
```

Data example:

```csv
thumbnail
iVBORw0KGgoAAAANSUhEUgAAAAUA
R0lGODlhAQABAIAAAAAAAP///yH5
aGVsbG8gd29ybGQ=
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### Array

A column for array/list values. It `MUST` have [`type`](#type) set to `"array"` and `MUST NOT` have a [`format`](#format) property.

Metadata example:

```json
{
  "properties": {
    "coordinates": {
      "type": "array"
    }
  }
}
```

Data example:

```csv
coordinates
"[1.5, 2.3]"
"[10, 20, 30]"
"[-5.2, 8.9, 12.1]"
```

Type properties:

- [`<jsonSchema>`](#jsonschema)

#### Object

A column for object/dictionary values. It `MUST` have [`type`](#type) set to `"object"` and `MUST NOT` have a [`format`](#format) property.

Metadata example:

```json
{
  "properties": {
    "metadata": {
      "type": "object"
    }
  }
}
```

Data example:

```csv
metadata
"{""author"": ""John"", ""version"": 1}"
"{""author"": ""Jane"", ""version"": 2}"
"{""author"": ""Bob"", ""version"": 1}"
```

Type properties:

- [`<jsonSchema>`](#jsonschema)

#### GeoJSON

A column for GeoJSON geometry objects. It `MUST` have [`type`](#type) set to `"object"` and [`format`](#format) set to `"geojson"`.

Metadata example:

```json
{
  "properties": {
    "location": {
      "type": "object",
      "format": "geojson"
    }
  }
}
```

Data example:

```csv
location
"{""type"": ""Point"", ""coordinates"": [30, 10]}"
"{""type"": ""LineString"", ""coordinates"": [[30, 10], [10, 30], [40, 40]]}"
"{""type"": ""Polygon"", ""coordinates"": [[[30, 10], [40, 40], [20, 40], [10, 20], [30, 10]]]}"
```

Type properties:

- [`<jsonSchema>`](#jsonschema)

#### TopoJSON

A column for TopoJSON geometry objects. It `MUST` have [`type`](#type) set to `"object"` and [`format`](#format) set to `"topojson"`.

Metadata example:

```json
{
  "properties": {
    "topology": {
      "type": "object",
      "format": "topojson"
    }
  }
}
```

Data example:

```csv
topology
"{""type"": ""Topology"", ""objects"": {""example"": {""type"": ""Point"", ""coordinates"": [0, 0]}}}"
"{""type"": ""Topology"", ""arcs"": [[[0, 0], [1, 1]]], ""objects"": {""line"": {""type"": ""LineString"", ""arcs"": [0]}}}"
"{""type"": ""Topology"", ""objects"": {""polygon"": {""type"": ""Polygon"", ""arcs"": [[0]]}}}"
```

Type properties:

- [`<jsonSchema>`](#jsonschema)

#### Unknown

A column for values of unknown type. It has no [`type`](#type). Any value is valid in it, including a missing one.

Metadata example:

```json
{
  "properties": {
    "column": {
      "title": "Column",
      "description": "Column description"
    }
  }
}
```

Data example:

```csv
column
a
1
false
```

### Type Properties

#### `format`

> [!NOTE]
> Supported types: **Categorical**, **Decimal**, **List**, **Base64**, **Hex**, **Email**, **URL**, **Date**, **Time**, **LocalTime**, **DateTime**, **LocalDateTime**, **Duration**, **WKT**, **WKB**, **GeoJSON**, **TopoJSON**

An optional format qualifier that specifies a more specific subtype of the base type. A `format` takes effect only when it is one listed above for the column's type; any other `format`, on any type, is an annotation, as in JSON Schema, and the column is the plain column of its type. A validator `SHOULD` warn about a `format` it does not recognise.

Metadata example:

```json
{
  "properties": {
    "email": {
      "type": "string",
      "format": "email"
    }
  }
}
```

Data example:

```csv
email
alice@example.com
bob@company.org
charlie@domain.net
```

#### `trueValues`

> [!NOTE]
> Supported types: **Boolean**

An optional array of string values that `MUST` be interpreted as `true` when parsing data. It `MUST` be an array of strings. It defaults to `["true", "True", "TRUE", "1"]`; a declared list replaces the default rather than adding to it, so with `["yes"]` the cell `true` is not a boolean.

A cell matches an item exactly, character for character, without trimming or case folding. A value `MUST NOT` be both in `trueValues` and in [`falseValues`](#falsevalues), counting the default of a list the column does not declare: a Table Schema with such a value is invalid.

Metadata example:

```json
{
  "properties": {
    "is_active": {
      "type": "boolean",
      "trueValues": ["yes", "true", "1", "Y"]
    }
  }
}
```

Data example:

```csv
is_active
yes
true
1
Y
```

#### `falseValues`

> [!NOTE]
> Supported types: **Boolean**

An optional array of string values that `MUST` be interpreted as `false` when parsing data. It `MUST` be an array of strings. It defaults to `["false", "False", "FALSE", "0"]`; a declared list replaces the default rather than adding to it. A cell matches an item exactly, as for [`trueValues`](#truevalues), and a value `MUST NOT` be in both lists.

Metadata example:

```json
{
  "properties": {
    "is_active": {
      "type": "boolean",
      "falseValues": ["no", "false", "0", "N"]
    }
  }
}
```

Data example:

```csv
is_active
no
false
0
N
```

#### `minimum`

> [!NOTE]
> Supported types: **Integer**, **Number**, **Decimal**, **Categorical**

An optional minimum value constraint (inclusive). It `MUST` be an integer for an integer column or an integer [Categorical](#categorical) one and a number for a number or decimal column; a decimal column's bound is a JSON number although its values are text, so a bound with more precision than a JSON number holds is not exact. It applies only to the supported types; on any other column, such as a temporal one, it `MUST` be ignored.

Metadata example:

```json
{
  "properties": {
    "temperature": {
      "type": "number",
      "minimum": -273.15
    }
  }
}
```

Data example:

```csv
temperature
-200.5
25.3
100.0
```

#### `maximum`

> [!NOTE]
> Supported types: **Integer**, **Number**, **Decimal**, **Categorical**

An optional maximum value constraint (inclusive). It `MUST` be an integer for an integer column or an integer [Categorical](#categorical) one and a number for a number or decimal column; a decimal column's bound is a JSON number although its values are text, so a bound with more precision than a JSON number holds is not exact. It applies only to the supported types; on any other column, such as a temporal one, it `MUST` be ignored.

Metadata example:

```json
{
  "properties": {
    "temperature": {
      "type": "number",
      "maximum": 1000
    }
  }
}
```

Data example:

```csv
temperature
25.5
100.0
999.9
```

#### `exclusiveMinimum`

> [!NOTE]
> Supported types: **Integer**, **Number**, **Decimal**, **Categorical**

An optional minimum value constraint (exclusive). It `MUST` be an integer for an integer column or an integer [Categorical](#categorical) one and a number for a number or decimal column; a decimal column's bound is a JSON number although its values are text, so a bound with more precision than a JSON number holds is not exact. It applies only to the supported types; on any other column, such as a temporal one, it `MUST` be ignored.

Metadata example:

```json
{
  "properties": {
    "probability": {
      "type": "number",
      "exclusiveMinimum": 0
    }
  }
}
```

Data example:

```csv
probability
0.1
0.5
0.999
```

#### `exclusiveMaximum`

> [!NOTE]
> Supported types: **Integer**, **Number**, **Decimal**, **Categorical**

An optional maximum value constraint (exclusive). It `MUST` be an integer for an integer column or an integer [Categorical](#categorical) one and a number for a number or decimal column; a decimal column's bound is a JSON number although its values are text, so a bound with more precision than a JSON number holds is not exact. It applies only to the supported types; on any other column, such as a temporal one, it `MUST` be ignored.

Metadata example:

```json
{
  "properties": {
    "probability": {
      "type": "number",
      "exclusiveMaximum": 1
    }
  }
}
```

Data example:

```csv
probability
0.001
0.5
0.999
```

#### `multipleOf`

> [!NOTE]
> Supported types: **Integer**, **Number**, **Decimal**, **Categorical**

An optional constraint that values `MUST` be a multiple of this number. For integers, including an integer [Categorical](#categorical) column, it `MUST` be a positive integer. For numbers and decimals, it `MUST` be a positive number.

The check `MUST` use exact decimal arithmetic on the value as written, not binary floating point: a value is a multiple when dividing it by `multipleOf` gives an integer, so `99.99` is a multiple of `0.01`.

Metadata example:

```json
{
  "properties": {
    "price": {
      "type": "number",
      "multipleOf": 0.01
    }
  }
}
```

Data example:

```csv
price
10.00
25.50
99.99
```

#### `decimalChar`

> [!NOTE]
> Supported types: **Number**, **Decimal**

An optional single character used as the decimal separator in the data. It defaults to a dot (`.`).

Metadata example:

```json
{
  "properties": {
    "price": {
      "type": "number",
      "decimalChar": ","
    }
  }
}
```

Data example:

```csv
price
19,99
5,50
123,45
```

#### `groupChar`

> [!NOTE]
> Supported types: **Integer**, **Number**, **Decimal**

An optional single character used as the thousands separator in the data. It `MUST` be a string of length 1. Without it, a value has no group separator.

Metadata example:

```json
{
  "properties": {
    "population": {
      "type": "integer",
      "groupChar": ","
    }
  }
}
```

Data example:

```csv
population
1,234,567
890,123
45,678
```

#### `withText`

> [!NOTE]
> Supported types: **Integer**, **Number**, **Decimal**

An optional boolean indicating whether numeric values may include text around the number, such as a currency or a unit. When `true`, the text before the first digit and after the last digit `MUST` be stripped before parsing; a [`decimalChar`](#decimalchar) directly before the first digit belongs to the number. If the stripped prefix contains a minus sign (`-`), the number `MUST` be read as negative. Text between the first and the last digit is not stripped, so a cell such as `12 USD 5` is not a number. No scaling is applied: `5%` is `5`.

Metadata example:

```json
{
  "properties": {
    "price": {
      "type": "number",
      "withText": true
    }
  }
}
```

Data example:

```csv
price
$19.99
€25.50
£12.34
-$5.00
```

#### `minLength`

> [!NOTE]
> Supported types: **Categorical**, **Decimal**, **String**, **List**, **Base64**, **Hex**, **Email**, **URL**, **Date**, **Time**, **LocalTime**, **DateTime**, **LocalDateTime**, **Duration**, **WKT**, **WKB**

An optional minimum length constraint for string values. It `MUST` be a non-negative integer.

Metadata example:

```json
{
  "properties": {
    "username": {
      "type": "string",
      "minLength": 3
    }
  }
}
```

Data example:

```csv
username
alice
bob123
charlie
```

#### `maxLength`

> [!NOTE]
> Supported types: **Categorical**, **Decimal**, **String**, **List**, **Base64**, **Hex**, **Email**, **URL**, **Date**, **Time**, **LocalTime**, **DateTime**, **LocalDateTime**, **Duration**, **WKT**, **WKB**

An optional maximum length constraint for string values. It `MUST` be a non-negative integer.

Metadata example:

```json
{
  "properties": {
    "username": {
      "type": "string",
      "maxLength": 20
    }
  }
}
```

Data example:

```csv
username
alice
bob
charlie
```

#### `pattern`

> [!NOTE]
> Supported types: **Categorical**, **Decimal**, **String**, **List**, **Base64**, **Hex**, **Email**, **URL**, **Date**, **Time**, **LocalTime**, **DateTime**, **LocalDateTime**, **Duration**, **WKT**, **WKB**

An optional regular expression pattern that values `MUST` match. It `MUST` be a valid regular expression in the ECMA-262 dialect, as in JSON Schema. An implementation that cannot compile a pattern, such as one using look-around its regex engine lacks, `MUST` report it as an error at the pattern rather than fail. As in JSON Schema, the pattern is not anchored: a value matches when the pattern matches any part of it, so a pattern meant for the whole value uses `^` and `$`.

Metadata example:

```json
{
  "properties": {
    "product_code": {
      "type": "string",
      "pattern": "^[A-Z]{3}-[0-9]{4}$"
    }
  }
}
```

Data example:

```csv
product_code
ABC-1234
XYZ-5678
DEF-9012
```

#### `categories`

> [!NOTE]
> Supported types: **Categorical**

An optional array of categorical values with optional labels. Each item can be either a simple value or an object with `value` and `label` properties. The values `MUST` have the same type as the containing property i.e. `"string"` or `"integer"`.

Metadata example:

```json
{
  "properties": {
    "severity": {
      "type": "integer",
      "format": "categorical",
      "categories": [
        { "value": 1, "label": "Low" },
        { "value": 2, "label": "Medium" },
        { "value": 3, "label": "High" }
      ]
    }
  }
}
```

Data example:

```csv
severity
1
2
3
1
```

#### `withOrder`

> [!NOTE]
> Supported types: **Categorical**

An optional boolean indicating that the categorical values in the column have natural order: the order of [`categories`](#categories). Sorting by the column then follows that order, as [Sorting](#sorting) sets out.

Metadata example:

```json
{
  "properties": {
    "severity": {
      "type": "integer",
      "format": "categorical",
      "withOrder": true,
      "categories": [
        { "value": 1, "label": "Low" },
        { "value": 2, "label": "Medium" },
        { "value": 3, "label": "High" }
      ]
    }
  }
}
```

Data example:

```csv
severity
1
2
3
1
```

#### `delimiter`

> [!NOTE]
> Supported types: **List**

An optional single character used to delimit items in a list column. It defaults to a comma (`,`).

Metadata example:

```json
{
  "properties": {
    "tags": {
      "type": "string",
      "format": "list",
      "delimiter": ";"
    }
  }
}
```

Data example:

```csv
tags
red;green;blue
alpha;beta
gamma;delta;epsilon
```

#### `itemType`

> [!NOTE]
> Supported types: **List**

An optional type for items in a list column. It `MUST` be one of: `string`, `integer`, `number`, `boolean`, `date`, `time`, `local-time`, `date-time`, `local-date-time`.

Metadata example:

```json
{
  "properties": {
    "measurements": {
      "type": "string",
      "format": "list",
      "itemType": "number"
    }
  }
}
```

Data example:

```csv
measurements
"1.5,2.3,4.7"
"10.2,15.8"
"3.14,2.71,1.41"
```

#### `minItems`

> [!NOTE]
> Supported types: **List**

An optional minimum number of items for the column. It `MUST` be a non-negative integer.

Metadata example:

```json
{
  "properties": {
    "tags": {
      "type": "string",
      "format": "list",
      "minItems": 1
    }
  }
}
```

Data example:

```csv
tags
"red,blue,green"
"small,compact"
"new,sale,featured"
```

#### `maxItems`

> [!NOTE]
> Supported types: **List**

An optional maximum number of items for the column. It `MUST` be a non-negative integer.

Metadata example:

```json
{
  "properties": {
    "tags": {
      "type": "string",
      "format": "list",
      "maxItems": 3
    }
  }
}
```

Data example:

```csv
tags
"red,blue,green"
"small,compact"
"new,sale,featured"
```

#### `temporalFormat`

> [!NOTE]
> Supported types: **Date**, **Time**, **LocalTime**, **DateTime**, **LocalDateTime**

An optional string specifying the pattern values are written in. A pattern is literal text and the conversion specifications below; it `MUST NOT` contain any other `%` sequence. A value `MUST` match the whole pattern: a value with text before or after the part the pattern matches is invalid.

| Specification | Meaning                                                               | Written as                     |
| ------------- | --------------------------------------------------------------------- | ------------------------------ |
| `%Y`          | Year                                                                  | `2024`                         |
| `%y`          | Year within a century: `00`–`69` is 2000–2069, `70`–`99` is 1970–1999 | `24`                           |
| `%m`          | Month, `01`–`12`                                                      | `01`                           |
| `%b`          | Abbreviated English month name                                        | `Jan`                          |
| `%B`          | Full English month name; reading also accepts `%b` names              | `January`                      |
| `%d`          | Day of the month, `01`–`31`                                           | `05`                           |
| `%j`          | Day of the year, `001`–`366`                                          | `005`                          |
| `%H`          | Hour, `00`–`23`                                                       | `14`                           |
| `%I`          | Hour on a 12-hour clock, `01`–`12`                                    | `02`                           |
| `%p`          | `AM` or `PM`                                                          | `PM`                           |
| `%M`          | Minute, `00`–`59`                                                     | `30`                           |
| `%S`          | Second, `00`–`59`                                                     | `00`                           |
| `%.f`         | Optional fractional seconds: `.` and one or more digits               | `.5` as `.500`, none when zero |
| `%z`          | UTC offset `+HHMM` or `-HHMM`; reading also accepts `+HH:MM`          | `+0200`                        |
| `%:z`         | UTC offset `+HH:MM` or `-HH:MM`; reading also accepts `+HHMM`         | `+02:00`                       |
| `%%`          | A literal `%`                                                         | `%`                            |

Reading a value, a numeric field matches with or without its leading zeros, so `1/5/2024` matches `%m/%d/%Y`, and month names and `%p` match regardless of case. Writing a value, an implementation `MUST` use the forms in the last column, with fractional seconds in 3, 6 or 9 digits.

Without a pattern, a value is in the default form its type states, which follows the grammar of [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339#section-5.6) section 5.6 for [Date](#date), [Time](#time), [LocalTime](#localtime), [DateTime](#datetime) and [LocalDateTime](#localdatetime) alike:

- A date is `YYYY-MM-DD`, with any year from `0000` to `9999`, and a day that exists in its month and year
- An hour is `00`–`23`, so `24:00:00` is invalid; a minute is `00`–`59`; a second is `00`–`59`, or `60` for a leap second. A leap second is valid only at `23:59:60` in UTC once the offset is applied (`23:59:60Z`, `15:59:60-08:00`), or at `23:59:60` as written in a type without an offset
- A date and a time are separated by `T` or `t`; a space or any other separator is invalid
- An offset is `Z` or `z`, or a sign, two digits of hours `00`–`23`, a colon and two digits of minutes `00`–`59`, so offsets lie from `-23:59` to `+23:59`; `+0200`, `+02` and `+24:00` are invalid

Writing a [DateTime](#datetime) or [Time](#time) value, an implementation `MUST` write it in UTC. Without a pattern, it `MUST` write the default form of the type, with fractional seconds only when they are not zero and with `Z` as the offset.

A pattern for [DateTime](#datetime) or [Time](#time) `MUST` contain one offset specification, `%z` or `%:z`. A pattern for [LocalDateTime](#localdatetime), [Date](#date) or [LocalTime](#localtime) `MUST NOT` contain one.

Metadata example:

```json
{
  "properties": {
    "collection_date": {
      "type": "string",
      "format": "date",
      "temporalFormat": "%m/%d/%Y"
    }
  }
}
```

Data example:

```csv
collection_date
01/15/2024
03/22/2024
12/31/2023
```

#### `<jsonSchema>`

> [!NOTE]
> Supported types: **Array**, **Object**, **GeoJSON**, **TopoJSON**

For `array` and `object` column types, all properties from [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12/schema) are supported to define the structure and validation rules.

For example, with an array column:

```json
{
  "properties": {
    "coordinates": {
      "type": "array",
      "items": {
        "type": "number"
      },
      "minItems": 2,
      "maxItems": 3
    }
  }
}
```

Data example:

```csv
coordinates
"[1.5, 2.3]"
"[10.2, 15.8, 20.5]"
"[3.14, 2.71]"
```

For example, with an object column:

```json
{
  "properties": {
    "metadata": {
      "type": "object",
      "properties": {
        "author": { "type": "string" },
        "version": { "type": "integer" }
      },
      "required": ["author"]
    }
  }
}
```

Data example:

```csv
metadata
"{""author"": ""Alice"", ""version"": 1}"
"{""author"": ""Bob"", ""version"": 2}"
"{""author"": ""Charlie""}"
```

## Validation

A Table Schema that is not valid is reported with metadata errors, and no table is checked against it.

A validator that reports the errors of a table `MUST` list them in this order:

1. Column errors, such as a [required](#required) column missing from the data, then the columns [`additionalProperties`](#additionalproperties) does not allow, in the order of the data
2. Cell errors, column by column in the order of [`properties`](#properties), and within a column by row
3. Row errors: [`primaryKey`](#primarykey) errors, then [`uniqueKeys`](#uniquekeys) errors

[`foreignKeys`](#foreignkeys) errors are dataset-level and follow these, as the [Dataset](../dataset#validation) specification sets out.

A cell that cannot be read as its column's type has a `cell/type` error and no other: its constraints are not checked. A cell that can be read is checked against every constraint of its column, and each one it fails is a separate error. A validator `MUST` report them in this keyword order:

1. [`enum`](#enum), [`const`](#const)
2. [`minimum`](#minimum), [`maximum`](#maximum), [`exclusiveMinimum`](#exclusiveminimum), [`exclusiveMaximum`](#exclusivemaximum), [`multipleOf`](#multipleof)
3. [`minLength`](#minlength), [`maxLength`](#maxlength), [`pattern`](#pattern)
4. [`minItems`](#minitems), [`maxItems`](#maxitems)
5. The JSON Schema keywords of an [Array](#array), [Object](#object), [GeoJSON](#geojson) or [TopoJSON](#topojson) column ([`<jsonSchema>`](#jsonschema))

A cell matches a boolean or category value exactly, character for character, without trimming or case folding, as [`trueValues`](#truevalues) and [Categorical](#categorical) set out.

## Sorting

An implementation that sorts a table by a column, such as in a query, `MUST` place null values last, in ascending and in descending order alike. A [Categorical](#categorical) column with [`withOrder`](#withorder) set to `true` sorts by the position of each value in [`categories`](#categories); every other column, including a Categorical one without `withOrder`, sorts by value.

## JSON Schema

A Table Schema `MAY` use any JSON Schema 2020-12 keyword that describes a single column, with the restrictions this specification states, and any annotation keyword such as `$comment` or `deprecated`. These keywords `MUST` be rejected, at the top level and on a column, since they do not describe one typed column per property: `allOf`, `anyOf`, `oneOf`, `not`, `if`, `then`, `else`, `$ref`, `$dynamicRef`, `$defs`, `patternProperties`, `propertyNames`, `dependentRequired`, `dependentSchemas`, `unevaluatedProperties`, `unevaluatedItems`. Inside an [Array](#array) or [Object](#object) column, which holds JSON values, every JSON Schema keyword is allowed.

The keywords [`enum`](#enum), [`const`](#const), [`default`](#default) and [`examples`](#examples) of a column that allows missing values `MAY` include `null`.

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

> [!TIP]
> Additional properties are allowed.

Properties not defined by this specification or by [JSON Schema](#json-schema) are allowed, as in JSON Schema, so that a descriptor written for a later version stays readable: an implementation `MUST` ignore them and `MAY` warn about them. Fairspec Table Schema has no extension mechanism: such properties carry no meaning.

## Comparison

> [!NOTE]
> This section is under development.
