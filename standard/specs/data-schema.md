---
title: Fairspec Data Schema
label: Data Schema
path: /specs/data-schema/
order: 4
---

<table>
  <tr>
    <th>Authors</th>
    <td>Evgeny Karev</td>
  </tr>
  <tr>
    <th>Profile</th>
    <td>
      <a href="https://fairspec.org/profiles/latest/data-schema.json">https://fairspec.org/profiles/latest/data-schema.json</a>
    </td>
  </tr>
</table>

Fairspec Data Schema is a simple [JSON](https://json.org)-based format that defines Data Schema to describe a class of structured data resources. It is fully compatible with the [JSON Schema](https://json-schema.org/) standard.

## Language

The key words `MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `RECOMMENDED`, `MAY`, and `OPTIONAL` in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## Descriptor

A Fairspec Data Schema is a [JSON](https://json.org/) resource that `MUST` be an object compatible with the [Data Schema](#data-schema) structure outlined below.

## Data Schema

A top-level descriptor object defining a schema of structured data resources. This includes a JSON file whose top-level value is an object, read without a [`jsonPointer`](../file-dialect#jsonpointer): such a file is not a table, so a Table Schema does not apply to it. It `MAY` have the following properties (all optional unless otherwise stated):

### `$schema` [#profile]

> [!NOTE]
> Data publisher `SHOULD` provide this property with an exact version when sharing a standalone descriptor publicly.

[External Path](#external-path) to one of the officially published Fairspec Data Schema profiles or `https://json-schema.org/draft/2020-12/schema` with default value `https://fairspec.org/profiles/latest/data-schema.json`. Every one of them is the JSON Schema 2020-12 dialect: a Data Schema whose `$schema` names any other dialect, such as `http://json-schema.org/draft-07/schema#`, `MUST` be reported as a metadata error at `$schema`.

For example for version X.Y.Z of the profile:

```json
{
  "$schema": "https://fairspec.org/profiles/X.Y.Z/data-schema.json"
}
```

### `<jsonSchema>`

Data Schema supports all the properties defined in [Json Schema Draft 2020-12](https://json-schema.org/draft/2020-12), with these rules:

- A subschema `MAY` be a boolean schema, `true` or `false`, as JSON Schema 2020-12 allows; the Data Schema itself is an object
- `format` is asserted, not only annotated: a Data Schema is validated as with the [Format-Assertion vocabulary](https://json-schema.org/draft/2020-12/json-schema-validation#section-7.2.2) of JSON Schema 2020-12 enabled, so a value that does not match a `format` the implementation knows is an error
- A `$ref` `MUST` resolve within the Data Schema itself, such as `#/$defs/item`. An implementation `MUST NOT` fetch a remote reference, and a `$ref` that does not resolve within the document `MUST` be reported as a metadata error at the `$ref`

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
  "$schema": "https://json-schema.org/draft/2020-12/schema"
}
```

## Extension

> [!TIP]
> Additional properties are allowed.

Properties not defined by this specification are allowed, as in JSON Schema, so that a descriptor written for a later version stays readable: an implementation `MUST` ignore them and `MAY` warn about them. Fairspec Data Schema has no extension mechanism: such properties carry no meaning.

## Comparison

> [!NOTE]
> This section is under development.
