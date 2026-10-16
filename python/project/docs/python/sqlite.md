---
title: Working with SQLite in Python
label: SQLite
path: /python/sqlite/
order: 18
---

SQLite database file handling with table loading and saving capabilities.

## Installation

```bash
pip install fairspec
```

## Getting Started

The SQLite plugin provides:

- `load_sqlite_table` - Load tables from SQLite databases
- `save_sqlite_table` - Save tables to SQLite databases
- `SqlitePlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_sqlite_table, Resource
from fairspec_metadata import SqliteFileDialect

table = load_sqlite_table(Resource(
    data="database.db",
    fileDialect=SqliteFileDialect(tableName="users"),
))
# column types will be automatically inferred from database schema
```

## Basic Usage

### Loading SQLite Tables

```python
from fairspec import load_sqlite_table, Resource
from fairspec_metadata import SqliteFileDialect

# Load a table from SQLite database
table = load_sqlite_table(Resource(
    data="data.db",
    fileDialect=SqliteFileDialect(tableName="products"),
))

# Load from a specific path
table = load_sqlite_table(Resource(
    data="/path/to/database.db",
    fileDialect=SqliteFileDialect(tableName="orders"),
))
```

### Saving SQLite Tables

```python
from fairspec import save_sqlite_table
from fairspec_metadata import SqliteFileDialect

# Save table to SQLite database
save_sqlite_table(table, path="output.db", fileDialect=SqliteFileDialect(
    tableName="results",
))

# Overwrite existing table
save_sqlite_table(table, path="output.db", fileDialect=SqliteFileDialect(
    tableName="results",
), overwrite=True)
```

## Advanced Features

### Schema Inference

Table schemas are automatically inferred from SQLite table definitions:

```python
from fairspec import load_sqlite_table, Resource
from fairspec_metadata import SqliteFileDialect

# Field types are automatically detected from database schema
table = load_sqlite_table(Resource(
    data="shop.db",
    fileDialect=SqliteFileDialect(tableName="products"),
))
# Types like INTEGER, TEXT, REAL are mapped to appropriate Table Schema types
```

### Creating New Tables

When saving, the plugin automatically creates the table structure:

```python
from fairspec import save_sqlite_table
from fairspec_metadata import SqliteFileDialect

# Creates a new database file with the specified table
save_sqlite_table(table, path="new-database.db", fileDialect=SqliteFileDialect(
    tableName="my_data",
))
```

### Working with Table Schema

You can provide a custom Table Schema when saving:

```python
from fairspec import save_sqlite_table
from fairspec_metadata import SqliteFileDialect, TableSchema, IntegerColumnProperty, StringColumnProperty

save_sqlite_table(table, path="output.db", fileDialect=SqliteFileDialect(
    tableName="customers",
), tableSchema=TableSchema(properties={
    "id": IntegerColumnProperty(),
    "name": StringColumnProperty(),
    "email": StringColumnProperty(),
}))
```
