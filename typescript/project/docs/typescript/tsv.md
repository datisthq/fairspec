---
title: Working with TSV in TypeScript
label: TSV
path: /typescript/library/tsv/
order: 6
---

Tab-separated values (TSV) file handling with automatic format detection.

## Installation

```bash
npm install fairspec
```

## Getting Started

The TSV format is handled by the CSV plugin, which provides:

- `loadCsvTable` - Load TSV files into tables
- `saveCsvTable` - Save tables to TSV files
- `CsvPlugin` - Plugin for framework integration

For example:

```typescript
import { loadCsvTable } from "fairspec"

const table = await loadCsvTable({ data: "table.tsv" })
// the column types will be automatically inferred
```

Loaders return a nodejs-polars `LazyDataFrame`: call `.collect()` to get a `DataFrame`. Savers take any `LazyDataFrame`, so pass `df.lazy()` to save a `DataFrame`.

## Basic Usage

### Loading TSV Files

```typescript
import { loadCsvTable } from "fairspec"

// Load a simple TSV file
const table = await loadCsvTable({ data: "data.tsv" })

// Load with explicit format
const table = await loadCsvTable({
  data: "data.tsv",
  fileDialect: {
    format: "tsv",
    headerRows: [1],
  },
})

// Load multiple TSV files (concatenated)
const table = await loadCsvTable({
  data: ["part1.tsv", "part2.tsv", "part3.tsv"],
})
```

### Saving TSV Files

```typescript
import { saveCsvTable } from "fairspec"

// Save with default options
await saveCsvTable(table, {
  path: "output.tsv",
  fileDialect: { format: "tsv" },
})

// Save with line terminator option
await saveCsvTable(table, {
  path: "output.tsv",
  fileDialect: {
    format: "tsv",
    lineTerminator: "\r\n",
  },
})
```

## Advanced Features

### Multi-Header Row Processing

```typescript
import { loadCsvTable } from "fairspec"

// TSV with multiple header rows
const table = await loadCsvTable({
  data: "multi-header.tsv",
  fileDialect: {
    format: "tsv",
    headerRows: [1, 2],
    headerJoin: "_",
  },
})
```

### Comment Handling

```typescript
import { loadCsvTable } from "fairspec"

// TSV with comment lines
const table = await loadCsvTable({
  data: "with-comments.tsv",
  fileDialect: {
    format: "tsv",
    commentPrefix: "#",
    headerRows: [1],
  },
})

// Or specify specific comment row numbers
const table = await loadCsvTable({
  data: "with-comments.tsv",
  fileDialect: {
    format: "tsv",
    commentRows: [1, 2],
    headerRows: [3],
  },
})
```

### Remote File Loading

```typescript
import { loadCsvTable } from "fairspec"

// Load from URL
const table = await loadCsvTable({
  data: "https://example.com/data.tsv",
})

// Load multiple remote files
const table = await loadCsvTable({
  data: [
    "https://api.example.com/data-2023.tsv",
    "https://api.example.com/data-2024.tsv",
  ],
})
```
