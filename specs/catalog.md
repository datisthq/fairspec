---
title: Fairspec Catalog
label: Catalog
path: /specs/catalog/
order: 1
---

:::card{title="Specification" icon="file-text"}

| Authors      | Profile                                                                                                |
| :----------- | :----------------------------------------------------------------------------------------------------- |
| Evgeny Karev | [https://fairspec.org/profiles/latest/catalog.json](https://fairspec.org/profiles/latest/catalog.json) |

:::

Fairspec Catalog is a simple [JSON Lines](https://jsonlines.org/)-based replication format that allows to sync a catalog of Fairspec Datasets. Dynamic search, sorting and similar capabilities are no goals of this specification.

## Language

The key words `MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `RECOMMENDED`, `MAY`, and `OPTIONAL` in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## Descriptor

A Fairspec Catalog is a [JSON Lines](https://jsonlines.org/) resource with each line represents a [Fairspec Dataset](../dataset) and its last updated time. The dataset locations `MUST` be unique within a catalog and the datasets `MUST` be sorted by the last updated time in descending order.

## Catalog

A top-level descriptor `MUST` be an array of [Dataset](#dataset) objects.

For example:

```jsonl
{"loc": "https://example.com/dataset1.json", "upd": "2023-10-01T00:00:00Z"}
{"loc": "https://example.com/dataset3.json", "upd": "2023-09-01T00:00:00Z"}
{"loc": "https://example.com/dataset2.json", "upd": "2023-08-01T00:00:00Z"}
```

## Dataset

A catalog entry pointing to a [Fairspec Dataset](../dataset). It `MUST` have the following properties (all required):

### `loc` [required] [#loc]

URI to the [Fairspec Dataset](../dataset) descriptor. The property `MUST` be [JSON Schema URI](https://json-schema.org/understanding-json-schema/reference/type#resource-identifiers).

For example:

```json
{
  "loc": "https://example.com/dataset.json"
}
```

### `upd` [required] [#upd]

The last updated time of the dataset. The property `MUST` be [JSON Schema date-time](https://json-schema.org/understanding-json-schema/reference/type#dates-and-times) (this format requires a timezone component).

For example:

```json
{
  "upd": "2023-10-01T00:00:00Z"
}
```

## Extension

> [!WARNING]
> Additional properties are NOT allowed.

Fairspec Catalog does not support extension.

## Streaming

Fairspec Catalog is designed to be used in a streaming manner. Because datasets are sorted by their last updated time in descending order, clients `SHOULD` read the catalog line-by-line and terminate reading once they encounter a dataset with an timestamp that has already been seen. This allows efficient synchronization without processing the entire catalog file.

## Comparison

> [!NOTE]
> This section is under development.
