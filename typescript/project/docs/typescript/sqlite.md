---
title: Working with SQLite in TypeScript
label: SQLite
path: /typescript/library/sqlite/
order: 13
---

SQLite database file handling with table loading and saving capabilities.

## Installation

```bash
npm install fairspec
```

## Getting Started

The SQLite plugin provides:

- `loadSqliteTable` - Load tables from SQLite databases
- `saveSqliteTable` - Save tables to SQLite databases
- `SqlitePlugin` - Plugin for framework integration

For example:

```typescript
import { loadSqliteTable } from "fairspec"

const table = await loadSqliteTable({
  data: "database.db",
  fileDialect: {
    format: "sqlite",
    tableName: "users",
  },
})
// column types will be automatically inferred from database schema
```

Loaders return a nodejs-polars `LazyDataFrame`: call `.collect()` to get a `DataFrame`. Savers take any `LazyDataFrame`, so pass `df.lazy()` to save a `DataFrame`.

## Basic Usage

### Loading SQLite Tables

```typescript
import { loadSqliteTable } from "fairspec"

// Load a table from SQLite database
const products = await loadSqliteTable({
  data: "data.db",
  fileDialect: {
    format: "sqlite",
    tableName: "products",
  },
})

// Load from a specific path
const orders = await loadSqliteTable({
  data: "/path/to/database.db",
  fileDialect: {
    format: "sqlite",
    tableName: "orders",
  },
})
```

### Saving SQLite Tables

```typescript
import { saveSqliteTable } from "fairspec"

// Save table to SQLite database
await saveSqliteTable(table, {
  path: "output.db",
  fileDialect: {
    format: "sqlite",
    tableName: "results",
  },
})

// Overwrite existing table
await saveSqliteTable(table, {
  path: "output.db",
  fileDialect: {
    format: "sqlite",
    tableName: "results",
  },
  overwrite: true,
})
```

## Advanced Features

### Schema Inference

Table schemas are automatically inferred from SQLite table definitions:

```typescript
import { loadSqliteTable } from "fairspec"

// Field types are automatically detected from database schema
const table = await loadSqliteTable({
  data: "shop.db",
  fileDialect: {
    format: "sqlite",
    tableName: "products",
  },
})
// Types like INTEGER, TEXT, REAL are mapped to appropriate Table Schema types
```

### Creating New Tables

When saving, the plugin automatically creates the table structure:

```typescript
import { saveSqliteTable } from "fairspec"

// Creates a new database file with the specified table
await saveSqliteTable(table, {
  path: "new-database.db",
  fileDialect: {
    format: "sqlite",
    tableName: "my_data",
  },
})
```

### Working with Table Schema

You can provide a custom Table Schema when saving:

```typescript
import { saveSqliteTable } from "fairspec"

await saveSqliteTable(table, {
  path: "output.db",
  fileDialect: {
    format: "sqlite",
    tableName: "customers",
  },
  tableSchema: {
    properties: {
      id: { type: "integer" },
      name: { type: "string" },
      email: { type: "string" },
    },
  },
})
```
