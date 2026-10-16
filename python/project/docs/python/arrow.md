---
title: Working with Arrow in Python
label: Arrow
path: /python/arrow/
order: 16
---

Apache Arrow IPC file handling with high-performance columnar data processing.

## Installation

```bash
pip install fairspec
```

## Getting Started

The Arrow plugin provides:

- `load_arrow_table` - Load Arrow IPC files into tables
- `save_arrow_table` - Save tables to Arrow IPC files
- `ArrowPlugin` - Plugin for framework integration

For example:

```python
from fairspec import load_arrow_table, Resource

table = load_arrow_table(Resource(data="table.arrow"))
# High-performance columnar format
```

## Basic Usage

### Loading Arrow Files

```python
from fairspec import load_arrow_table, Resource

# Load from local file
table = load_arrow_table(Resource(data="data.arrow"))

# Load from remote URL
table = load_arrow_table(Resource(data="https://example.com/data.arrow"))

# Load multiple files (concatenated)
table = load_arrow_table(Resource(data=["file1.arrow", "file2.arrow"]))
```

### Saving Arrow Files

```python
from fairspec import save_arrow_table

# Save with default options
save_arrow_table(table, path="output.arrow")
```

## Advanced Features

### Remote File Loading

```python
from fairspec import load_arrow_table, Resource

# Load from URL
table = load_arrow_table(Resource(data="https://example.com/data.arrow"))

# Load multiple remote files
table = load_arrow_table(Resource(data=[
    "https://api.example.com/data-2023.arrow",
    "https://api.example.com/data-2024.arrow",
]))
```
