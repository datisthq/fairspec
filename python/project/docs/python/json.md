---
title: Working with JSON tables in Python
label: JSON
path: /python/json/
order: 12
---

JSON file handling with automatic format detection and high-performance data operations.

## Installation

```bash
pip install fairspec
```

## Getting Started

The JSON plugin provides:

- `load_json_table` - Load JSON files into tables
- `save_json_table` - Save tables to JSON files
- `JsonPlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_json_table, Resource

table = load_json_table(Resource(data="table.json"))
# Standard JSON array of objects format
```

## Basic Usage

### Loading JSON Files

```python
from fairspec import load_json_table, Resource

# Load from local file
table = load_json_table(Resource(data="data.json"))

# Load from remote URL
table = load_json_table(Resource(data="https://example.com/data.json"))

# Load multiple files (concatenated)
table = load_json_table(Resource(data=["file1.json", "file2.json"]))
```

### Saving JSON Files

```python
from fairspec import save_json_table
from fairspec_metadata import JsonFileDialect

# Save with default options
save_json_table(table, path="output.json")

# Save with explicit format
save_json_table(table, path="output.json", fileDialect=JsonFileDialect())
```

## Standard Format

JSON tables use an array of objects format:

```json
[
  {"id": 1, "name": "Alice", "age": 30},
  {"id": 2, "name": "Bob", "age": 25}
]
```

## Advanced Features

### JSON Pointer Extraction

Extract data from nested objects using `jsonPointer`:

```python
from fairspec import load_json_table, Resource
from fairspec_metadata import JsonFileDialect

# Input: {"users": [{"id": 1, "name": "Alice"}]}
table = load_json_table(Resource(
    data="data.json",
    fileDialect=JsonFileDialect(jsonPointer="users"),
))
```

### Column Selection

Select specific columns using `columnNames`:

```python
from fairspec import load_json_table, Resource
from fairspec_metadata import JsonFileDialect

# Only load specific columns
table = load_json_table(Resource(
    data="data.json",
    fileDialect=JsonFileDialect(columnNames=["name", "age"]),
))
```

### Array Format Handling

Handle CSV-style array data with `rowType: "array"`:

```python
from fairspec import load_json_table, Resource
from fairspec_metadata import JsonFileDialect

# Input: [["id", "name"], [1, "Alice"], [2, "Bob"]]
table = load_json_table(Resource(
    data="data.json",
    fileDialect=JsonFileDialect(rowType="array"),
))
```

### Saving with JSON Pointer

Wrap data in a nested structure when saving:

```python
from fairspec import save_json_table
from fairspec_metadata import JsonFileDialect

# Output: {"users": [{"id": 1, "name": "Alice"}]}
save_json_table(table, path="output.json", fileDialect=JsonFileDialect(
    jsonPointer="users",
))
```
