---
title: Fairspec Dataset
label: Dataset
path: /specs/dataset/
order: 2
---

:::card{title="Specification" icon="file-text"}

| Authors      | Profile                                                                                                |
| :----------- | :----------------------------------------------------------------------------------------------------- |
| Evgeny Karev | [https://fairspec.org/profiles/latest/dataset.json](https://fairspec.org/profiles/latest/dataset.json) |

:::

Fairspec Dataset is a simple [JSON](https://json.org)-based format that allows to describe a single dataset and its resources. It is compatible with the [DataCite](https://schema.datacite.org/) standard for metadata and the [JSON Schema](https://json-schema.org/) standard for structured data.

## Language

The key words `MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `RECOMMENDED`, `MAY`, and `OPTIONAL` in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## Descriptor

> [!NOTE]
> If applicable, a data publisher `SHOULD` name the descriptor `dataset.json` and place it in the root of a dataset directory when sharing the dataset publicly.

A Fairspec Dataset is a [JSON](https://json.org/) resource that `MUST` be an object compatible with the [Dataset](#dataset) structure outlined below.

## Dataset

A top-level descriptor object describing an individual dataset. It `MIGHT` have the following properties (all optional unless otherwise stated):

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

A list of resources. It `MUST` be an array with search item `MUST` be a [Resource](#resource).

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
- Array of [Paths](#path) to files
- Inline JSON object
- Inline JSON array of objects

When multiple files are provided, they `MUST` all follow the same [Format](#format) and their contents `MUST` be physically concatenable in case of binary formats and logically concatenable in case of textual formats (i.e., combining them should produce a valid single file of that format).

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

An optional name for the resource. It `MUST` be a string consisting of alphanumeric characters and underscores. If provided, it can be used to reference resource within a dataset context. For example, a name of the resource is used in [Foreign Keys](../table#foreign-keys) specified in [Fairspec Schema](../table-schema).

For example:

```json
{
  "name": "measurements"
}
```

### `textual`

A boolean indicating whether the file is text-based. When `true`, the file `MUST` be `utf-8` encoded.

For example:

```json
{
  "data": "document.md",
  "textual": true
}
```

### `integrity`

The integrity check of the file. It `MUST` be a JSON object with the following properties:

**`type`**

The type of the integrity check. It `MUST` be one of the following values:

- `md5`
- `sha1`
- `sha256`
- `sha512`

**`hash`**

The hash of the file. It `MUST` be a string.

For example for a file with SHA-256 hash:

```json
{
  "integrity": {
    "type": "sha256",
    "hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  }
}
```

### `fileDialect`

It `MUST` be a [Path](#path) to a File Dialect or an object with the File Dialect. The File Dialect `MUST` be compatible with the [Fairspec File Dialect](../file-dialect) specification. If present, [Data](#data) `MUST` be compatible with the provided dialect.

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

It `MUST` be a [Path](#path) to or an object with [Fairspec Data Schema](../data-schema). If present, [Data](#data) `MUST` be a JSON document that is compatible with the provided schema.

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

## Common

Common properties shared by multiple entities in the descriptor.

### Path

It `MUST` be is a string representing a file location. It `MUST` be one of the following:

- [Internal Path](#internal-path)
- [External Path](#external-path)

### Internal Path

It `MUST` be a string representing a relative path, using Unix-style forward slashes (`/`) as path separators. The path is resolved relative to the location of the dataset descriptor file. Unix-style paths `MUST` be converted to the appropriate platform-specific format when accessing files (e.g., converting `/` to `\` on Windows).

Internal path `MUST` point to a file in the same directory as the descriptor file or in a subdirectory of it. Files outside of the descriptor directory are not supported.

The path `MUST NOT` contain any of the following:

- Absolute path indicators (starting with `/` or `~`)
- Directory traversal sequences (`..`)
- Windows-style backslashes (`\`) - only forward slashes (`/`) are allowed as separators
- Windows drive letters (`C:`, `D:`, etc.)
- URI schemes (`://`)

The path `MAY` contain unicode characters, spaces, and special characters in file and directory names.

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

Fairspec Dataset has a simple yet powerful extension mechanism based on the JSON Schema standard. An extension is a domain-specific Fairspec Dataset flavour that enriches the standard with additional metadata properties and validation rules.

### Creation

A custom JSON Schema can be provided as a `$schema` property in the dataset descriptor. The profile instructs to validate the descriptor using JSON Schema rules defined by the extension. The extension's schema `MUST` include base Fairspec Dataset schema in the root `allOf` property.

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
  "$schema": "http://json-schema.org/draft/2020-12/schema",
  "title": "Faispec Spectroscopy Profile",
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
