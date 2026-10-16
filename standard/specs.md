---
title: Specifications
label: Specifications
path: /specs/
order: 20
icon: file-text
---

The Fairspec standard consists of five complementary JSON-based specifications. Each one describes a different layer of data publishing — from individual files up to collections of datasets.

- [Catalog](/specs/catalog/) — A replication format for syncing collections of datasets using JSON Lines.
- [Dataset](/specs/dataset/) — Describes individual datasets and their resources using DataCite-compatible metadata.
- [Data Schema](/specs/data-schema/) — A simple JSON-based format for describing the structure of JSON data, compatible with JSON Schema Draft 2020-12.
- [File Dialect](/specs/file-dialect/) — Defines how different file formats (CSV, TSV, JSON, JSONL, XLSX, ODS, Parquet, Arrow, SQLite) should be interpreted.
- [Table Schema](/specs/table-schema/) — A schema specification adapted for tabular data, including columns, constraints, and primary/foreign keys.

All specifications use the same descriptor pattern: a JSON document referencing a published profile via the `$schema` property. Profiles are versioned and hosted at `https://fairspec.org/profiles/<version>/<name>.json`.
