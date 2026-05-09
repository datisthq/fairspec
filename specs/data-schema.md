---
title: Fairspec Data Schema
label: Data Schema
path: /specs/data-schema/
order: 4
---

:::card{title="Specification" icon="file-text"}

| Authors      | Profile                                                                                                        |
| :----------- | :------------------------------------------------------------------------------------------------------------- |
| Evgeny Karev | [https://fairspec.org/profiles/latest/data-schema.json](https://fairspec.org/profiles/latest/data-schema.json) |

:::

Fairspec Data Schema is a simple [JSON](https://json.org)-based format that defines Data Schema to describe a class of structured data resources. It is fully compatible with the [JSON Schema](https://json-schema.org/) standard.

## Language

The key words `MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `RECOMMENDED`, `MAY`, and `OPTIONAL` in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## Descriptor

A Fairspec Data Schema is a [JSON](https://json.org/) resource that `MUST` be an object compatible with the [Data Schema](#data-schema) structure outlined below.

## Data Schema

A top-level descriptor object defining a schema of structured data resources. It `MIGHT` have the following properties (all optional unless otherwise stated):

### `$schema` [#profile]

> [!NOTE]
> Data publisher `SHOULD` provide this property with an exact version when sharing a standalone descriptor publicly.

[External Path](#external-path) to one of the officially published Fairspec Data Schema profiles or `https://json-schema.org/draft/2020-12/schema` with default value `https://fairspec.org/profiles/latest/data-schema.json`.

For example for version X.Y.Z of the profile:

```json
{
  "$schema": "https://fairspec.org/profiles/X.Y.Z/data-schema.json"
}
```

### `<jsonSchema>`

Dataset supports all the properties defined in [Json Schema Draft 2020-12](https://json-schema.org/draft/2020-12).
For example:

```json
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string"
    },
    "age": {
      "type": "integer"
    }
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

Fairspec Data Schema does not support extension.

## Comparison

> [!NOTE]
> This section is under development.
