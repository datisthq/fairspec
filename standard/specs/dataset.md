---
title: Fairspec Dataset
label: Dataset
path: /specs/dataset/
order: 2
---

<table>
  <tr>
    <th>Authors</th>
    <td>Evgeny Karev</td>
  </tr>
  <tr>
    <th>Profile</th>
    <td>
      <a href="https://fairspec.org/profiles/latest/dataset.json">https://fairspec.org/profiles/latest/dataset.json</a>
    </td>
  </tr>
</table>

Fairspec Dataset is a simple [JSON](https://json.org)-based format that allows to describe a single dataset and its resources. It is compatible with the [DataCite](https://schema.datacite.org/) standard for metadata and the [JSON Schema](https://json-schema.org/) standard for structured data.

## Language

The key words `MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `RECOMMENDED`, `MAY`, and `OPTIONAL` in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## Descriptor

> [!NOTE]
> If applicable, a data publisher `SHOULD` name the descriptor `dataset.json` and place it in the root of a dataset directory when sharing the dataset publicly.

A Fairspec Dataset is a [JSON](https://json.org/) resource that `MUST` be an object compatible with the [Dataset](#dataset) structure outlined below.

## Dataset

A top-level descriptor object describing an individual dataset. It `MAY` have the following properties (all optional unless otherwise stated):

### `$schema` [#profile]

> [!NOTE]
> Data publishers `SHOULD` provide this property with an exact version when sharing a standalone descriptor publicly.

[External Path](#external-path) to one of the officially published Fairspec Dataset profiles or to a Fairspec Dataset [Extension](#extension) profile with default value `https://fairspec.org/profiles/latest/dataset.json`.

For example for version X.Y.Z of the profile:

```json
{
  "$schema": "https://fairspec.org/profiles/X.Y.Z/dataset.json"
}
```

### `resources`

A list of resources. It `MUST` be an array where each item `MUST` be a [Resource](#resource).

For example for a single resource:

```json
{
  "resources": [
    {
      "data": "https://example.com/file.csv"
    }
  ]
}
```

For multiple resources:

```json
{
  "resources": [
    {
      "data": "https://example.com/file1.csv",
      "format": {
        "name": "csv",
        "delimiter": ";"
      },
      "integrity": {
        "type": "sha256",
        "hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
      }
    },
    {
      "data": "https://example.com/file2.json"
    }
  ]
}
```

### `<datacite>` [#dataset-datacite]

Dataset supports all the properties defined in [DataCite Metadata Schema 4.6](https://datacite-metadata-schema.readthedocs.io/en/4.6/).

For example for a dataset with DOI and some other properties:

```json
{
  "doi": "10.1234/5678",
  "title": "My Dataset",
  "creators": [
    {
      "name": "John Doe",
      "nameType": "Personal"
    }
  ]
}
```

## Resource

A resource within a dataset. It `MUST` have the following properties (all optional unless otherwise stated):

### `data`

Data or content of the resource. It `MUST` be in one of the following:

- [Path](#path) to a file
- Array of at least one [Path](#path) to files
- Inline JSON object
- Inline JSON array of objects

A resource without `data` describes no content. It `MAY` carry metadata, but a [Table Schema](#tableschema), [Data Schema](#dataschema) or [File Dialect](#filedialect) on it has nothing to apply to, so validating such a resource reports a `resource/type` error.

When multiple paths are provided, the resource is multipart: the files are the ordered parts of one resource. Every part `MUST` be a complete file of the same format, and the [File Dialect](#filedialect) and [Table Schema](#tableschema), if present, apply to every part. The order of the paths is significant.

For tabular formats, the records of the resource are the records of each part, taken in order. If the dialect declares header rows, every part `MUST` carry its own; such rows describe their part and are not records of the resource. The header of every part `MUST` give the same column names, in the same order, as the header of the first part; a part whose header differs is an error that names the part. A part with no bytes at all has no header, so under a dialect with header rows it is an error too. The check holds when the dialect's [`columnNames`](../file-dialect#columnnames) names the columns: the names come from `columnNames`, but each part's own header `MUST` still match the first part's. In [JSON](../file-dialect#json) and [JSONL](../file-dialect#jsonl) parts, array rows compare their header rows as CSV does, while object rows have no header row and are not compared.

For other formats, the content of the resource is the content of each part, taken in order.

For example, for a single internal file:

```json
{
  "data": "file.csv"
}
```

For multiple external files:

```json
{
  "data": ["https://example.com/file1.csv", "https://example.com/file2.csv"]
}
```

For a JSON object:

```json
{
  "data": {
    "name": "John Doe",
    "age": 30
  }
}
```

For a JSON array of objects:

```json
{
  "data": [
    {
      "name": "John Doe",
      "age": 30
    },
    {
      "name": "Jane Doe",
      "age": 25
    }
  ]
}
```

### `name`

An optional name for the resource. It `MUST` be a non-empty string of ASCII letters, digits and underscores (`[A-Za-z0-9_]+`), of any length, and it `MAY` start with a digit. It `MUST` be unique among the resources of the dataset; names that differ only in case are distinct. If provided, it can be used to reference resource within a dataset context. For example, a name of the resource is used in [Foreign Keys](../table-schema#foreignkeys) specified in [Fairspec Table Schema](../table-schema).

For example:

```json
{
  "name": "measurements"
}
```

### `textual`

A boolean indicating whether the file is text-based. When `true`, the file `MUST` be `utf-8` encoded. For a multipart resource, every part `MUST` be `utf-8` encoded. A UTF-8 byte order mark at the start of a file is valid UTF-8.

For example:

```json
{
  "data": "document.md",
  "textual": true
}
```

### `integrity`

The integrity check of the resource's content. It `MUST` be a JSON object with exactly the following properties, both of which are `REQUIRED`:

**`type`**

The type of the integrity check. It `MUST` be one of the following values:

- `md5`
- `sha1`
- `sha256`
- `sha512`

**`hash`**

The hash of the resource's content. It `MUST` be the digest in lowercase hexadecimal, of the length its `type` produces: 32 characters for `md5`, 40 for `sha1`, 64 for `sha256` and 128 for `sha512`. Hashes are compared exactly. A hash with an uppercase digit or of another length is a metadata error at `integrity/hash`, and the content is not hashed.

For example for a file with SHA-256 hash:

```json
{
  "integrity": {
    "type": "sha256",
    "hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  }
}
```

A resource with inline [Data](#data) has no file bytes to hash, so it `MUST NOT` have `integrity`.

For a multipart resource, the hash `MUST` be computed over the concatenation of the parts' bytes, taken in the order the paths appear in [Data](#data). It covers every byte of every part, including any header rows, and reordering the parts changes the hash.

### `fileDialect`

It `MUST` be a [Path](#path) to a File Dialect or an object with the File Dialect. The File Dialect `MUST` be compatible with the [Fairspec File Dialect](../file-dialect) specification. If present, [Data](#data) `MUST` be compatible with the provided dialect.

Without a dialect, an implementation `MAY` infer the format and its [properties](../file-dialect#format-properties) on a best-effort basis, for example from the file extension or the content; how it does so is implementation-specific. A resource that every implementation must read the same way `SHOULD` declare its dialect's `format`.

For example, for a file with CSV format:

```json
{
  "fileDialect": {
    "format": "csv",
    "delimiter": ";"
  }
}
```

### `dataSchema`

It `MUST` be a [Path](#path) to or an object with [Fairspec Data Schema](../data-schema). If present, [Data](#data) `MUST` be a JSON document that is compatible with the provided schema. A resource with a Data Schema `MUST NOT` have multipart data: a JSON document cannot be split into parts.

A JSON file whose top-level value is an object, read without a [`jsonPointer`](../file-dialect#jsonpointer), is not a table: it is described by a Data Schema, and a [Table Schema](#tableschema) does not apply to it.

For example as an external path:

```json
{
  "dataSchema": "https://example.com/schema.json"
}
```

For example as an object:

```json
{
  "dataSchema": {
    "type": "object",
    "properties": {
      "name": {
        "type": "string"
      },
      "age": {
        "type": "integer"
      }
    },
    "required": ["name", "age"]
  }
}
```

### `tableSchema`

It `MUST` be a [Path](#path) to or an object with [Fairspec Table Schema](../table-schema). If present, [Data](#data) `MUST` be a table that is compatible with the provided schema.

For example as an external path:

```json
{
  "tableSchema": "https://example.com/schema.json"
}
```

For example as an object:

```json
{
  "tableSchema": {
    "required": ["name", "age"],
    "properties": {
      "name": {
        "type": "string"
      },
      "age": {
        "type": "integer"
      }
    }
  }
}
```

### `<datacite>` [#resource-datacite]

Resource supports all the properties defined in [DataCite Metadata Schema 4.6](https://datacite-metadata-schema.readthedocs.io/en/4.6/).

For example for a resource with geolocation:

```json
{
  "geoLocations": [
    {
      "geoLocationPoint": {
        "pointLongitude": 12.34,
        "pointLatitude": 56.78
      },
      "geoLocationBox": {
        "westBoundLongitude": 12.34,
        "eastBoundLongitude": 56.78,
        "southBoundLatitude": 12.34,
        "northBoundLatitude": 56.78
      }
    }
  ]
}
```

## Validation

A validator `MUST` check the descriptor first. When the descriptor has metadata errors, including those of a [File Dialect](#filedialect), [Data Schema](#dataschema) or [Table Schema](#tableschema) it holds, the validator reports them and checks no data.

Otherwise a validator that reports the errors of a dataset `MUST` list the errors of its resources first, each resource's table errors in the order the [Table Schema](../table-schema#validation) specification sets. Dataset-level errors follow them: [foreign key](../table-schema#foreignkeys) violations, and foreign keys that reference a resource the dataset does not contain.

## Common

Common properties shared by multiple entities in the descriptor.

### Path

It `MUST` be a string representing a file location. It `MUST` be one of the following:

- [Internal Path](#internal-path)
- [External Path](#external-path)

### Internal Path

It `MUST` be a string representing a relative path, using Unix-style forward slashes (`/`) as path separators. The path is resolved relative to the location of the dataset descriptor file. Unix-style paths `MUST` be converted to the appropriate platform-specific format when accessing files (e.g., converting `/` to `\` on Windows).

Internal path `MUST` point to a file in the same directory as the descriptor file or in a subdirectory of it. Files outside of the descriptor directory are not supported.

The path `MUST NOT` contain any of the following:

- Absolute path indicators (starting with `/` or `~`)
- A `..` component, which leaves the directory; a name that merely starts with two dots, such as `..notes.csv`, is allowed
- Windows-style backslashes (`\`) - only forward slashes (`/`) are allowed as separators
- A colon (`:`) in a first component that starts with a letter, which would read as a Windows drive (`C:`) or a URI scheme (`http:`, `file:`); a colon elsewhere, as in `10:30.csv`, is allowed

The path `MAY` contain unicode characters, spaces, and special characters in file and directory names.

An internal path is a file path, not a URL: it is used as written, and a percent sign is a literal character, so `a%20b.csv` names a file whose name holds `%20`, not a space.

For example:

```json
{
  "data": "measurements.csv"
}
```

For example with subdirectories:

```json
{
  "data": "data/experiments/results-2024.json"
}
```

For example with unicode and special characters:

```json
{
  "data": "données/résultats (final).csv"
}
```

### External Path

It `MUST` be a string representing an HTTP or HTTPS URL to a remote file. As in [RFC 3986](https://datatracker.ietf.org/doc/html/rfc3986#section-3.1), the scheme is case-insensitive, so `HTTPS://example.com/data.csv` is an external path.

For example:

```json
{
  "data": "https://example.com/datasets/measurements.csv"
}
```

## Extension

> [!TIP]
> Additional properties are allowed.

Fairspec Dataset has a simple yet powerful extension mechanism based on the JSON Schema standard. An extension is a domain-specific Fairspec Dataset flavour that enriches the standard with additional metadata properties and validation rules.

### Creation

A custom JSON Schema can be provided as a `$schema` property in the dataset descriptor. The profile instructs to validate the descriptor using JSON Schema rules defined by the extension. The extension's schema `MUST` include base Fairspec Dataset schema in the root `allOf` property, as a `{"$ref": "https://fairspec.org/profiles/<version>/dataset.json"}` entry.

A validator `MUST` check a descriptor in two steps: against the profile its `$schema` declares, then against the Fairspec version the validator implements. The second step `MUST` keep every property the core does not define, since an extension adds properties on top of the core.

Using JSON Schema features with custom profiles allows you to:

- Add new domain-specific properties
- Require existing properties to meet specific requirements
- Define expected resource types and their schemas
- Combine existing profiles as part of a high-level extension

### Example

For example, a Spectroscopy Fairspec extension that requires spectral metadata:

```json
{
  "$schema": "https://spectroscopy.org/profiles/1.0.0/dataset.json",
  "resources": [
    {
      "data": "spectrum.csv",
      "spectralRange": {
        "min": 400,
        "max": 4000,
        "unit": "cm-1"
      }
    }
  ]
}
```

The extension profile would include the base Fairspec Dataset schema and add domain-specific requirements:

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Fairspec Spectroscopy Profile",
  "allOf": [
    { "$ref": "https://fairspec.org/profiles/1.0.0/dataset.json" },
    { "$ref": "#/$defs/spectroscopyMixin" }
  ],
  "$defs": {
    "spectroscopyMixin": {
      "type": "object",
      "properties": {
        "resources": {
          "type": "array",
          "items": {
            "properties": {
              "spectralRange": {
                "type": "object",
                "required": ["min", "max", "unit"],
                "properties": {
                  "min": { "type": "number" },
                  "max": { "type": "number" },
                  "unit": { "type": "string" }
                }
              }
            }
          }
        }
      }
    }
  }
}
```

## Comparison

> [!NOTE]
> This section is under development.
