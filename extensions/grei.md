---
title: Fairspec GREI
label: GREI
path: /extensions/grei/
order: 4
---

:::card{title="Specification" icon="file-text"}

| Authors      | Profile                                                                                                          |
| :----------- | :--------------------------------------------------------------------------------------------------------------- |
| Evgeny Karev | [https://fairspec.org/profiles/latest/grei/dataset.json](https://fairspec.org/profiles/latest/grei/dataset.json) |

:::

Fairspec GREI is a [Fairspec Dataset](../dataset) extension that requires the use of the [GREI](https://datascience.nih.gov/data-ecosystem/generalist-repository-ecosystem-initiative)'s required properties.

## Language

The key words `MUST`, `MUST NOT`, `REQUIRED`, `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, `RECOMMENDED`, `MAY`, and `OPTIONAL` in this document are to be interpreted as described in [RFC 2119](https://www.ietf.org/rfc/rfc2119.txt).

## Descriptor

A Fairspec Dataset is a [JSON](https://json.org/) resource that `MUST` be an object compatible with the [Dataset](#dataset) structure outlined below.

## Dataset

A top-level descriptor object describing an individual dataset. It `MAY` have the [Fairspec Dataset](../../specs/dataset) properties and `MUST` have the following properties:

- `$schema` = `https://fairspec.org/profiles/<version>/grei/dataset.json`
- `doi`
- `creators`
- `titles`
- `publisher`
- `publicationYear`
- `resourceType`
- `subjects`
- `dates`
- `relatedIdentifiers`
- `version`
- `rightsList`
- `descriptions`
- `fundingReferences`
