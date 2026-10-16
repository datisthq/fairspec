---
title: Working with JSONL tables in Python
label: JSONL
path: /python/jsonl/
order: 13
---

JSONL (JSON Lines) file handling with automatic format detection and high-performance data operations.

## Installation

```bash
pip install fairspec
```

## Getting Started

The JSONL format is handled by the JSON plugin, which provides:

- `load_json_table` - Load JSONL files into tables
- `save_json_table` - Save tables to JSONL files
- `JsonPlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_json_table, Resource

table = load_json_table(Resource(data="table.jsonl"))
# Newline-delimited JSON objects
```

## Basic Usage

### Loading JSONL Files

```python
from fairspec import load_json_table, Resource
from fairspec_metadata import JsonFileDialect

# Load from local file
table = load_json_table(Resource(data="data.jsonl"))

# Load with explicit format
table = load_json_table(Resource(
    data="data.jsonl",
    fileDialect=JsonFileDialect(format="jsonl"),
))

# Load multiple files (concatenated)
table = load_json_table(Resource(data=["part1.jsonl", "part2.jsonl"]))
```

### Saving JSONL Files

```python
from fairspec import save_json_table
from fairspec_metadata import JsonFileDialect

# Save as JSONL
save_json_table(table, path="output.jsonl", fileDialect=JsonFileDialect(format="jsonl"))
```

## Standard Format

JSONL uses newline-delimited JSON objects:

```jsonl
{"id": 1, "name": "Alice", "age": 30}
{"id": 2, "name": "Bob", "age": 25}
{"id": 3, "name": "Charlie", "age": 35}
```

## Advanced Features

### Column Selection

Select specific columns using `columnNames`:

```python
from fairspec import load_json_table, Resource
from fairspec_metadata import JsonFileDialect

# Only load specific columns
table = load_json_table(Resource(
    data="data.jsonl",
    fileDialect=JsonFileDialect(format="jsonl", columnNames=["name", "age"]),
))
```

### Array Format Handling

Handle CSV-style array data with `rowType: "array"`:

```python
from fairspec import load_json_table, Resource
from fairspec_metadata import JsonFileDialect

# Input JSONL with arrays:
# ["id", "name"]
# [1, "Alice"]
# [2, "Bob"]

table = load_json_table(Resource(
    data="data.jsonl",
    fileDialect=JsonFileDialect(format="jsonl", rowType="array"),
))
```

### Remote File Loading

```python
from fairspec import load_json_table, Resource

# Load from URL
table = load_json_table(Resource(data="https://example.com/data.jsonl"))

# Load multiple remote files
table = load_json_table(Resource(data=[
    "https://api.example.com/logs-2023.jsonl",
    "https://api.example.com/logs-2024.jsonl",
]))
```
