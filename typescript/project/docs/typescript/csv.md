---
title: Working with CSV in TypeScript
label: CSV
path: /typescript/library/csv/
order: 5
---

Comprehensive CSV file handling with automatic format detection and advanced header processing.

## Installation

```bash
npm install fairspec
```

## Getting Started

The CSV plugin provides these capabilities:

- `loadCsvTable` - Load CSV/TSV files into tables
- `saveCsvTable` - Save tables to CSV/TSV files
- `CsvPlugin` - Plugin for framework integration

For example:

```typescript
import { loadCsvTable } from "fairspec"

const table = await loadCsvTable({ data: "table.csv" })
// the column types will be automatically inferred
// or you can provide a Table Schema
```

Loaders return a nodejs-polars `LazyDataFrame`: call `.collect()` to get a `DataFrame`. Savers take any `LazyDataFrame`, so pass `df.lazy()` to save a `DataFrame`.

## Basic Usage

### Loading CSV Files

```typescript
import { loadCsvTable } from "fairspec"

// Load a simple CSV file
const local = await loadCsvTable({ data: "data.csv" })

// Load with custom format
const custom = await loadCsvTable({
  data: "data.csv",
  fileDialect: {
    format: "csv",
    delimiter: ";",
    headerRows: [1],
  },
})

// Load multiple CSV files (concatenated)
const combined = await loadCsvTable({
  data: ["part1.csv", "part2.csv", "part3.csv"],
})
```

Every part carries its own header rows, and header and comment rows count the rows of each part. A part whose header gives other column names than part 1's, or a 0-byte part under a dialect with header rows, fails the load with a `FairspecException` whose report holds a `file/header` error naming the part; validation reports that error. With `columnNames` the names come from the dialect, but each part's own header is still compared. A 0-byte part without header rows adds no rows.

### Saving CSV Files

```typescript
import { saveCsvTable } from "fairspec"

// Save with default options
await saveCsvTable(table, { path: "output.csv" })

// Save with custom format
await saveCsvTable(table, {
  path: "output.csv",
  fileDialect: {
    format: "csv",
    delimiter: "\t",
    quoteChar: "'",
  },
})

// Save as TSV
await saveCsvTable(table, {
  path: "output.tsv",
  fileDialect: { format: "tsv" },
})
```

### Automatic Format Detection

```typescript
import { loadCsvTable } from "fairspec"

// Format is automatically detected when not specified
const detected = await loadCsvTable({ data: "unknown-dialect.csv" })
// The CSV plugin will automatically infer delimiter, quote characters, etc.

// You can also explicitly specify the format if detection isn't accurate
const explicit = await loadCsvTable({
  data: "data.csv",
  fileDialect: {
    format: "csv",
    delimiter: ",",
    quoteChar: '"',
    headerRows: [1],
  },
})
```

## Advanced Features

### Multi-Header Row Processing

```typescript
import { loadCsvTable } from "fairspec"

// CSV with multiple header rows:
// Year,2023,2023,2024,2024
// Quarter,Q1,Q2,Q1,Q2
// Revenue,100,120,110,130

const table = await loadCsvTable({
  data: "multi-header.csv",
  fileDialect: {
    format: "csv",
    headerRows: [1, 2],
    headerJoin: "_",
  },
})
// Resulting columns: ["Year_Quarter", "2023_Q1", "2023_Q2", "2024_Q1", "2024_Q2"]
```

### Comment Row Handling

```typescript
import { loadCsvTable } from "fairspec"

// CSV with comment rows:
// # This is a comment
// # Generated on 2024-01-01
// Name,Age,City
// John,25,NYC

const byCommentRows = await loadCsvTable({
  data: "with-comments.csv",
  fileDialect: {
    format: "csv",
    commentRows: [1, 2],
    headerRows: [3],
  },
})

// Or use commentPrefix to skip lines starting with a specific character
const byCommentPrefix = await loadCsvTable({
  data: "with-comments.csv",
  fileDialect: {
    format: "csv",
    commentPrefix: "#",
    headerRows: [1],
  },
})
```

### Remote File Loading

```typescript
import { loadCsvTable } from "fairspec"

// Load from URL
const remote = await loadCsvTable({
  data: "https://example.com/data.csv",
})

// Load multiple remote files
const remoteParts = await loadCsvTable({
  data: [
    "https://api.example.com/data-2023.csv",
    "https://api.example.com/data-2024.csv",
  ],
})
```
