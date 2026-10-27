---
title: Working with Datasets in Terminal
label: Dataset
path: /rust/terminal/dataset/
order: 1
---

Dataset operations for managing collections of tabular resources with metadata and schemas.

## Available Commands

The `fairspec dataset` command provides utilities for working with datasets:

- `infer` - Automatically infer a dataset descriptor from data files
- `copy` - Copy datasets to a local folder
- `validate` - Validate dataset descriptors and their resources
- `list` - List resources in a dataset
- `query` - Query the dataset's tables with SQL
- `script` - Interactive SQL session with loaded dataset

## What is a Dataset?

A dataset is a collection of related data resources (tables) with:

- Metadata describing the dataset (title, description, license, etc.)
- Resource definitions for each table (path, format, schema)
- Table Schemas defining the structure of each resource
- Relationships and foreign keys between resources

Datasets use JSON descriptor files (often named `dataset.json`) following the Fairspec specification.

## Infer Dataset

Automatically generate a dataset descriptor from data files:

```bash
# Infer from single file
fairspec dataset infer data.csv

# Infer from multiple files
fairspec dataset infer users.csv products.csv orders.csv

# Infer with remote files
fairspec dataset infer https://example.com/data1.csv data2.csv

# Save to descriptor file
fairspec dataset infer *.csv --json > dataset.json
```

### Inference Process

The infer command automatically:

1. Detects format for each file (CSV, JSON, Excel, etc.)
2. Infers Table Schema for each resource
3. Generates resource names from file names
4. Creates a complete dataset descriptor

### Options

- `--debug` - Show debug information
- `--json` - Output as JSON

### Format Options

Format detection and schema inference can be customized:

- `--delimiter <char>` - CSV delimiter
- `--header-rows <numbers>` - Header row indices (JSON array)
- `--sample-rows <number>` - Sample size for schema inference
- `--confidence <number>` - Confidence threshold for type detection
- `--column-types <json>` - Override types for specific columns
- `--keep-strings` - Keep original string types
- `--comma-decimal` - Treat comma as decimal separator
- `--month-first` - Parse dates as month-first

### Generated Descriptor

Example generated dataset descriptor:

```json
{
  "resources": [
    {
      "data": "users.csv",
      "name": "users",
      "textual": true,
      "integrity": {
        "type": "sha256",
        "hash": "efb0784b89fb0318b356c374a9c45313bccea3bd9b5511b86fe23f98a9014570"
      },
      "fileDialect": {
        "format": "csv",
        "delimiter": ",",
        "lineTerminator": "\n",
        "headerRows": [1]
      },
      "tableSchema": {
        "properties": {
          "id": {
            "type": "integer"
          },
          "name": {
            "type": "string"
          },
          "email": {
            "type": "string",
            "format": "email"
          },
          "created_at": {
            "type": "string",
            "format": "date"
          }
        }
      }
    },
    {
      "data": "orders.csv",
      "name": "orders",
      "textual": true,
      "integrity": {
        "type": "sha256",
        "hash": "e1783d3bd2735800751ae59fc3d0b582cd216177f829f7870c36fa9c67b323b5"
      },
      "fileDialect": {
        "format": "csv",
        "delimiter": ",",
        "lineTerminator": "\n",
        "headerRows": [1]
      },
      "tableSchema": {
        "properties": {
          "order_id": {
            "type": "integer"
          },
          "user_id": {
            "type": "integer"
          },
          "amount": {
            "type": "number"
          },
          "status": {
            "type": "string"
          }
        }
      }
    }
  ]
}
```

## Copy Dataset

Copy a dataset and all its resources to a local folder:

```bash
# Copy dataset to local folder
fairspec dataset copy dataset.json --to-path ./local-dataset

# Copy remote dataset
fairspec dataset copy https://example.com/dataset.json --to-path ./dataset

# Silent mode for automation
fairspec dataset copy dataset.json --to-path ./output --silent
```

### Copy Behavior

The copy command:

- Downloads all remote resources
- Preserves directory structure
- Updates resource paths in the descriptor to point to local files
- Creates the target directory if it doesn't exist
- Saves the updated descriptor to the target location

### Options

- `--to-path <path>` (required) - Target directory path
- `--silent` - Suppress output messages
- `--debug` - Show debug information
- `--json` - Output as JSON

### Example

Given a dataset with remote resources:

```json
{
  "resources": [
    {
      "name": "users",
      "data": "https://example.com/data/users.csv"
    },
    {
      "name": "products",
      "data": "https://example.com/data/products.csv"
    }
  ]
}
```

After copying:

```bash
fairspec dataset copy dataset.json --to-path ./local
```

Results in:

```text
./local/
  dataset.json   # Updated descriptor
  users.csv          # Downloaded resource
  products.csv       # Downloaded resource
```

## Validate Dataset

Validate a dataset descriptor and all its resources:

```bash
# Validate local dataset
fairspec dataset validate dataset.json

# Validate remote dataset
fairspec dataset validate https://example.com/dataset.json

# Output validation report as JSON
fairspec dataset validate dataset.json --json
```

### Validation Checks

The validate command checks:

- **Descriptor validity** - Valid JSON and conforms to the Fairspec specification
- **Resource existence** - All referenced resources can be loaded
- **Schema validation** - Each resource validates against its Table Schema
- **Referential integrity** - Foreign key relationships are valid
- **Format compliance** - Resources match their declared formats

### Validation Report

Returns a validation report with:

- `valid` - Boolean indicating if validation passed
- `errors` - Array of validation errors (if any)

Example validation errors:

```json
{
  "valid": false,
  "errors": [
    {
      "type": "cell/type",
      "resourceName": "orders",
      "columnName": "amount",
      "rowNumber": 3,
      "cell": "abc",
      "columnType": "number"
    },
    {
      "type": "foreignKey",
      "resourceName": "orders",
      "foreignKey": {
        "columns": ["user_id"],
        "reference": {
          "resource": "users",
          "columns": ["id"]
        }
      },
      "cells": ["3"]
    }
  ]
}
```

### Options

- `--debug` - Show debug information
- `--json` - Output as JSON

## List Resources

List all resources in a dataset:

```bash
# List resources
fairspec dataset list dataset.json

# List from remote dataset
fairspec dataset list https://example.com/dataset.json

# Output as JSON array
fairspec dataset list dataset.json --json
```

### Output

Returns an array of resource names in the dataset, printed as JSON with or without `--json`:

```json
["users", "products", "orders", "transactions"]
```

### Options

- `--debug` - Show debug information
- `--json` - Output as JSON

## Query Dataset

Run a SQL query across every table resource in a dataset. Each resource is a table named after
the resource, so a query can join several of them:

```bash
# Join two resources
fairspec dataset query dataset.json \
  "SELECT users.name, COUNT(*) AS orders FROM orders JOIN users ON orders.user_id = users.id GROUP BY users.name"

# Query a remote dataset
fairspec dataset query https://example.com/dataset.json "SELECT * FROM products LIMIT 10"

# Output as JSON
fairspec dataset query dataset.json "SELECT COUNT(*) AS n FROM users" --json
```

### Table Names

- A resource is queried under its `name`; one without a name takes the name `dataset list`
  shows for it, inferred from its file name
- Only the resources the query mentions are loaded, so a large resource the query does not use
  costs nothing
- A resource that is not a table, such as a JSON object, cannot be queried

### Options

- `--debug` - Show debug information
- `--json` - Output as JSON

## Interactive Scripting

Start an interactive session with a loaded dataset. Each line runs as SQL against the dataset, with a table per resource name, and an empty line exits:

```bash
# Load dataset and start a session
fairspec dataset script dataset.json

# Script remote dataset
fairspec dataset script https://example.com/dataset.json
```

The Python and TypeScript CLIs open a language REPL here; the Rust session takes SQL only, like `dataset query` one line at a time.

### Example Session

```text
⚠ Each line runs as SQL against the dataset, a table per resource name; an empty line exits
> SELECT * FROM users WHERE active
shape: (1, 2)
...
> SELECT users.name, COUNT(*) FROM orders JOIN users ON orders.user_id = users.id GROUP BY users.name
...
>
```

## Common Workflows

### Create Dataset from Files

```bash
# 1. Infer dataset from multiple files
fairspec dataset infer data/*.csv --json > dataset.json

# 2. Manually edit dataset.json to add:
#    - Title and description
#    - License information
#    - Foreign key relationships
#    - Additional metadata

# 3. Validate the dataset
fairspec dataset validate dataset.json

# 4. List resources to confirm
fairspec dataset list dataset.json
```

### Clone Remote Dataset

```bash
# 1. Copy remote dataset locally
fairspec dataset copy https://example.com/dataset.json --to-path ./local-data

# 2. Validate local copy
fairspec dataset validate ./local-data/dataset.json

# 3. List resources
fairspec dataset list ./local-data/dataset.json
```

### Dataset Quality Assurance

```bash
# 1. Validate the dataset
fairspec dataset validate dataset.json

# 2. If validation fails, check individual resources
fairspec table validate --dataset dataset.json --resource users

# 3. Inspect resource schemas
fairspec table infer-schema --dataset dataset.json --resource users

```

### Dataset Evolution

```bash
# 1. Start with existing dataset
fairspec dataset validate old-dataset.json

# 2. Add new data files
fairspec dataset infer old-data/*.csv new-data/*.csv --json > dataset.json

# 3. Merge metadata from old descriptor
# (manual step - copy title, license, etc.)

# 4. Validate updated dataset
fairspec dataset validate dataset.json

# 5. Verify all resources
fairspec dataset list dataset.json
```

### Automation and CI/CD

```bash
#!/bin/bash

# Validate dataset in CI pipeline
if fairspec dataset validate dataset.json --json | jq -e '.valid'; then
  echo "✓ Dataset validation passed"
  exit 0
else
  echo "✗ Dataset validation failed"
  fairspec dataset validate dataset.json
  exit 1
fi
```

## Output Formats

### Text Output (default)

Human-readable output with colors and formatting:

```bash
fairspec dataset list dataset.json
```

Output:

```json
["users", "products", "orders"]
```

### JSON Output

Machine-readable JSON for automation and scripting:

```bash
fairspec dataset validate dataset.json --json
```

### Silent Mode

Suppress all output except errors (for copy command):

```bash
fairspec dataset copy dataset.json --to-path ./output --silent
```

Use exit code to check success:

```bash
if fairspec dataset copy dataset.json --to-path ./output --silent; then
  echo "Success"
else
  echo "Failed"
fi
```

## Examples

### Create Multi-Table Dataset

```bash
# Prepare your data files
# - customers.csv
# - orders.csv
# - products.csv

# Infer the dataset
fairspec dataset infer customers.csv orders.csv products.csv --json > dataset.json

# Enhance the descriptor
cat > dataset.json << 'EOF'
{
  "name": "sales-data",
  "title": "Sales Database Export",
  "description": "Customer orders and product catalog",
  "license": "CC-BY-4.0",
  "resources": [
    {
      "name": "customers",
      "data": "customers.csv",
      "tableSchema": { "properties": { ... } }
    },
    {
      "name": "orders",
      "data": "orders.csv",
      "tableSchema": {
        "properties": { ... },
        "foreignKeys": [
          {
            "columns": ["customer_id"],
            "reference": {
              "resource": "customers",
              "columns": ["id"]
            }
          }
        ]
      }
    }
  ]
}
EOF

# Validate
fairspec dataset validate dataset.json
```

### Download and Validate Public Dataset

```bash
# Copy public dataset
fairspec dataset copy https://data.example.org/climate/dataset.json \
  --to-path ./climate-data

# Validate local copy
fairspec dataset validate ./climate-data/dataset.json

# List available resources
fairspec dataset list ./climate-data/dataset.json

# Explore specific resource
fairspec table describe --dataset ./climate-data/dataset.json \
  --resource temperature
```

### Dataset Testing

```bash
# test-dataset.sh

echo "Testing dataset integrity..."

# 1. Validate descriptor
if ! fairspec dataset validate dataset.json --silent; then
  echo "✗ Dataset validation failed"
  fairspec dataset validate dataset.json
  exit 1
fi

# 2. Check all resources exist
for resource in $(fairspec dataset list dataset.json --json | jq -r '.[]'); do
  echo "Checking resource: $resource"
  if ! fairspec table describe --dataset dataset.json --resource "$resource" --silent; then
    echo "✗ Resource $resource could not be loaded"
    exit 1
  fi
done

echo "✓ All tests passed"
```

### Interactive Data Exploration

```bash
# Start interactive session
fairspec dataset script dataset.json

# In the session, each line is SQL over the resources:
# - List a resource: SELECT * FROM users LIMIT 5
# - Filter: SELECT * FROM users WHERE active
# - Join: SELECT * FROM orders JOIN users ON orders.user_id = users.id
```

## Working with Resources

All dataset commands integrate with table commands through the `--dataset` and `--resource` options:

```bash
# Load resource from dataset
fairspec table describe --dataset dataset.json --resource users

# Query resource
fairspec table query --dataset dataset.json --resource orders \
  "SELECT * FROM self WHERE status = 'shipped'"

# Validate resource
fairspec table validate --dataset dataset.json --resource products

# Infer resource schema
fairspec table infer-schema --dataset dataset.json --resource users
```

This approach allows you to:

- Work with resources without specifying paths or formats
- Use embedded Table Schemas automatically
- Maintain consistency across your dataset
- Simplify command-line usage
