---
title: Working with Inline Data tables in Python
label: Inline Data
path: /python/inline/
order: 19
---

Inline data handling for tables embedded directly in resource definitions.

## Installation

```bash
pip install fairspec
```

## Getting Started

The Inline plugin provides:

- `load_inline_table` - Load tables from inline data
- `InlinePlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_inline_table, Resource

table = load_inline_table(Resource(data=[
    {"id": 1, "name": "Alice"},
    {"id": 2, "name": "Bob"},
]))
```

## Basic Usage

### Object Format Data

The most common format is an array of objects:

```python
from fairspec import load_inline_table, Resource

table = load_inline_table(Resource(data=[
    {"id": 1, "name": "english", "native": "English"},
    {"id": 2, "name": "chinese", "native": "中文"},
    {"id": 3, "name": "spanish", "native": "Español"},
]))
```

### Array Format Data

You can also use array-of-arrays format with the first row as headers:

```python
from fairspec import load_inline_table, Resource

table = load_inline_table(Resource(data=[
    ["id", "name", "native"],
    [1, "english", "English"],
    [2, "chinese", "中文"],
    [3, "spanish", "Español"],
]))
```

## Advanced Features

### With Table Schema

Provide a Table Schema for type validation and conversion:

```python
from fairspec import load_inline_table, Resource
from fairspec_metadata import TableSchema, IntegerColumnProperty, StringColumnProperty, BooleanColumnProperty

table = load_inline_table(Resource(
    data=[
        {"id": 1, "name": "english", "active": True},
        {"id": 2, "name": "chinese", "active": False},
    ],
    tableSchema=TableSchema(properties={
        "id": IntegerColumnProperty(),
        "name": StringColumnProperty(),
        "active": BooleanColumnProperty(),
    }),
))
```

### Mixed with File Data

Inline data can be used alongside file-based resources in datasets:

```python
from fairspec import load_inline_table, load_csv_table, Resource

# Load inline reference data
languages = load_inline_table(Resource(
    name="languages",
    data=[
        {"id": 1, "name": "english"},
        {"id": 2, "name": "chinese"},
    ],
))

# Load main data from file
users = load_csv_table(Resource(name="users", data="users.csv"))
```

### Resource Metadata

You can include metadata with inline data resources:

```python
from fairspec import load_inline_table, Resource
from fairspec_metadata import TableSchema, StringColumnProperty

table = load_inline_table(Resource(
    name="countries",
    title="Country Reference Data",
    description="ISO country codes and names",
    data=[
        {"code": "US", "name": "United States"},
        {"code": "CN", "name": "China"},
        {"code": "ES", "name": "Spain"},
    ],
    tableSchema=TableSchema(properties={
        "code": StringColumnProperty(),
        "name": StringColumnProperty(),
    }),
))
```
