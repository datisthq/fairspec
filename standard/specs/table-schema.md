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

Fairspec Table Schema is a simple [JSON](https://json.org)-based format that defines Table Schema to describe a class of tabular data resources. Table Schema is structurally compatible with the [JSON Schema](https://json-schema.org/) standard. When expedient, this specification aims to be structurally compatible with the [Data Package](https://datapackage.org/standard/table-schema/) standard.

## Language

The key words `MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `RECOMMENDED`, `MAY`, and `OPTIONAL` in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## Descriptor

A Fairspec Table Schema is a [JSON](https://json.org/) resource that `MUST` be an object compatible with the [Table Schema](#table-schema) structure outlined below.

## Table Schema

A top-level descriptor object defining a schema of tabular data resources. It `MIGHT` have the following properties (all optional unless otherwise stated):

### `$schema` [#profile]

> [!NOTE]
> Data publisher `SHOULD` provide this property with an exact version when sharing a standalone descriptor publicly.

[External Path](#external-path) to one of the officially published Fairspec Table Schema profiles with default value `https://fairspec.org/profiles/latest/table-schema.json`.

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

An optional array of column names that form the table's primary key. The combination of values in these columns `MUST` uniquely identify each row. At least one column name `MUST` be specified if this property is present.

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

An optional array of unique key constraints. Each unique key is an array of column names whose combined values `MUST` be unique across all rows in the table. Each unique key array `MUST` contain at least one column name.

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

An optional array of foreign key constraints that define relationships between this table and other tables. Each foreign key specifies local `columns` and their `reference` to columns in another resource (identified by resource [`name`](../dataset#name) in the dataset context).

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
  ]
}
```

### `missingValues`

> [!NOTE]
> Table Schema level missing values works in addition to [Format](../dataset#format) level missing values or null sequences. For example, CSV Format treats no values between commas (`,,`) as `null` so there is no need to provide `''` as a Table Schema missing value in this case.

An optional list of values that represent missing or null data across all columns in the table. Each item can be either a simple value or an object with `value` and `label` properties for documentation purposes. The values type `MUST` be `"string"` or `"integer"`.

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

If a column allows missing values the type can include `null` (order insensitive):

- `["boolean", "null"]` - True/false values or missing values
- `["integer", "null"]` - Whole numbers or missing values
- `["number", "null"]` - Numeric values or missing values
- `["string", "null"]` - Text values or missing values
- `["array", "null"]` - Array/list values or missing values
- `["object", "null"]` - Object/dictionary values or missing values

Any other value of the type property indicates that the column type is [Unknown](#unknown).

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
      "default": "pending"
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

Metadata example:

```json
{
  "properties": {
    "severity": {
      "type": "integer",
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
- [`withOrder`](#categoriesordered)

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

A column for ISO 8601 date values. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"date"`.

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

A column for ISO 8601 time values. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"time"`.

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
14:30:00
09:45:30
18:00:00
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)
- [`temporalFormat`](#temporalformat)

#### DateTime

A column for ISO 8601 date with time values. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"date-time"`.

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

#### Duration

A column for ISO 8601 duration values. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"duration"`.

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
0101000000000000000000000000000000000024400000000000003E40
0102000000030000000000000000003E400000000000002440
0103000000010000000500000000000000000024400000000000003E40
```

Type properties:

- [`minLength`](#minlength)
- [`maxLength`](#maxlength)
- [`pattern`](#pattern)

#### Hex

A column for hexadecimal encoded data. It `MUST` have [`type`](#type) set to `"string"` and [`format`](#format) set to `"hex"`.

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

- [`<jsonSchema>`](#jsonSchema)

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

- [`<jsonSchema>`](#jsonSchema)

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

- [`<jsonSchema>`](#jsonSchema)

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

- [`<jsonSchema>`](#jsonSchema)

#### Unknown

A column for values of unknown type. It `MUST` have [`type`](#type) not supported by the types above.

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
> Supported types: **List**, **Base64**, **Hex**, **Email**, **URL**, **DateTime**, **Date**, **Time**, **Duration**, **WKT**, **WKB**, **GeoJSON**, **TopoJSON**

An optional format qualifier that specifies a more specific subtype of the base type.

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

An optional array of string values that `SHOULD` be interpreted as `true` when parsing data. It `MUST` be an array of strings.

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

An optional array of string values that `SHOULD` be interpreted as `false` when parsing data. It `MUST` be an array of strings.

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
> Supported types: **Integer**, **Number**, **Decimal**

An optional minimum value constraint (inclusive). The type `MUST` match the column type.

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
> Supported types: **Integer**, **Number**, **Decimal**

An optional maximum value constraint (inclusive). The type `MUST` match the column type.

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
> Supported types: **Integer**, **Number**, **Decimal**

An optional minimum value constraint (exclusive). The type `MUST` match the column type.

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
> Supported types: **Integer**, **Number**, **Decimal**

An optional maximum value constraint (exclusive). The type `MUST` match the column type.

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
> Supported types: **Integer**, **Number**, **Decimal**

An optional constraint that values `MUST` be a multiple of this number. For integers, it `MUST` be a positive integer. For numbers, it `MUST` be a positive number.

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

An optional single character used as the decimal separator in the data.

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

An optional single character used as the thousands separator in the data. It `MUST` be a string of length 1.

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

An optional boolean indicating whether numeric values may include non-numeric text that should be stripped during parsing.

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
```

#### `minLength`

> [!NOTE]
> Supported types: **Decimal**, **String**, **List**, **Base64**, **Hex**, **Email**, **URL**, **DateTime**, **Date**, **Time**, **Duration**, **WKT**, **WKB**

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
> Supported types: **Decimal**, **String**, **List**, **Base64**, **Hex**, **Email**, **URL**, **DateTime**, **Date**, **Time**, **Duration**, **WKT**, **WKB**

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
> Supported types: **Decimal**, **String**, **List**, **Base64**, **Hex**, **Email**, **URL**, **DateTime**, **Date**, **Time**, **Duration**, **WKT**, **WKB**

An optional regular expression pattern that values `MUST` match. It `MUST` be a valid regex string.

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

An optional boolean indicating that the categorical values in the column have natural order.

Metadata example:

```json
{
  "properties": {
    "severity": {
      "type": "integer",
      "categoriesOrdered": true,
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

An optional single character used to delimit items in a list column.

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

An optional type for items in a list column. It `MUST` be one of: `string`, `integer`, `number`, `boolean`, `date-time`, `date`, `time`.

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
> Supported types: **DateTime**, **Date**, **Time**

An optional string specifying the temporal format pattern as per the [Strftime ](https://pubs.opengroup.org/onlinepubs/009696799/functions/strftime.html) specification.

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

Fairspec Table Schema does not support extension.

## Comparison

> [!NOTE]
> This section is under development.
