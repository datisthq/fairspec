---
title: Overview
label: Overview
path: /overview/
order: 10
icon: book-open
---

> [!TIP]
> The [FAIR principles](https://en.wikipedia.org/wiki/FAIR_data) provide guidelines to improve the **Findability, Accessibility, Interoperability, and Reusability** of digital assets.

**Fairspec** is a lightweight data exchange format designed to make scientific and research data FAIR (findable, accessible, interoperable, and reusable). Built on widely-adopted open standards, Fairspec provides a simple yet powerful way to describe, share, and discover datasets.

## Specifications

Fairspec consists of four complementary JSON-based specifications:

**Fairspec Catalog** - A replication format for syncing collections of datasets using JSON Lines. Each entry points to a dataset with its location and last updated timestamp, enabling efficient streaming and synchronization.

**Fairspec Dataset** - Describes individual datasets and their resources using DataCite-compatible metadata. Supports multiple data formats (CSV, JSON, Parquet, XLSX, and more) with rich metadata including integrity checks, format specifications, and schema definitions.

**Fairspec Dialect** - Defines file format dialects and their properties. Specifies how different file formats (CSV, TSV, JSON, JSONL, XLSX, ODS, Parquet, Arrow, SQLite) should be interpreted, including delimiters, headers, encoding, and format-specific settings.

**Fairspec Data Schema** - A simple JSON-based format that defines data schemas to describe structured data resources. Fully compatible with JSON Schema Draft 2020-12, it provides a way to validate and document the structure of JSON data.

**Fairspec Table Schema** - A table schema specification structurally compatible with JSON Schema but adapted for tabular data. Defines column types, constraints, relationships, and validation rules with support for primary keys, foreign keys, and complex data types.

## Features

- **Standards-based**: Compatible with DataCite Metadata Schema 4.6 and JSON Schema Draft 2020-12
- **Format-agnostic**: Works with CSV, TSV, JSON, JSONL, Parquet, Arrow, XLSX, ODS, and SQLite
- **Extensible**: Custom profiles can add domain-specific properties while maintaining compatibility
- **Software-first**: Available with Python and TypeScript implementations out-of-the-box

Fairspec enables researchers and data publishers to describe their data with sufficient metadata to ensure it can be properly discovered, cited, and reused by others in the scientific community.
