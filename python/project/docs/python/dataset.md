---
title: Working with Datasets in Python
label: Dataset
path: /python/dataset/
order: 1
---
Dataset operations for managing collections of tabular resources with metadata and schemas — from Python.

## Available Functions

The dataset API provides utilities for working with datasets:

- `load_dataset` - Load a dataset descriptor from a local path or remote URL
- `infer_dataset` - Automatically fill in missing metadata for every resource
- `save_dataset` - Save a dataset (and its resources) via plugins
- `validate_dataset` - Validate a dataset descriptor and all its resources
- `validate_dataset_foreign_keys` - Check foreign-key relationships across resources
- `render_dataset_as` - Render a dataset as Markdown, JSON, or other formats

## What is a Dataset?

A dataset is a collection of related data resources (tables) with:

- Metadata describing the dataset (title, description, license, etc.)
- Resource definitions for each table (path, dialect, schema)
- Table Schemas defining the structure of each resource
- Relationships and foreign keys between resources

Datasets are represented in Python by the `Dataset` model and serialised as JSON descriptors (often named `dataset.json`) following the Fairspec specification.

```python
from fairspec import Dataset, Resource

dataset = Dataset(
    name="sales-data",
    title="Sales Database Export",
    license="CC-BY-4.0",
    resources=[
        Resource(name="users", data="users.csv"),
        Resource(name="orders", data="orders.csv"),
    ],
)
```

## Loading a Dataset

Load a dataset descriptor from a local path or remote URL:

```python
from fairspec import load_dataset

# Load from local file
descriptor = load_dataset("dataset.json")

# Load from a remote URL
descriptor = load_dataset("https://example.com/dataset.json")
```

`load_dataset` dispatches to the appropriate plugin based on the source (folder, zip, GitHub, Zenodo, CKAN, …) and returns a `Descriptor` (a dict-like JSON object) or `None` if no plugin recognises the source.

To get a typed `Dataset` model, validate the descriptor:

```python
from fairspec import Dataset, load_dataset

descriptor = load_dataset("dataset.json")
dataset = Dataset.model_validate(descriptor)
```

For format-specific loaders, use:

- `load_dataset_from_folder(path)`
- `load_dataset_from_zip(path)`
- `load_dataset_from_github(url)`
- `load_dataset_from_zenodo(url)`
- `load_dataset_from_ckan(url)`

## Inferring a Dataset

Automatically fill in missing metadata for every resource in a dataset:

```python
from fairspec import Dataset, Resource, infer_dataset

dataset = Dataset(resources=[
    Resource(data="users.csv"),
    Resource(data="orders.csv"),
])

dataset = infer_dataset(dataset)
```

The inference process automatically fills in:

- `name` — generated from the file name
- `fileDialect` — detected from the file content (CSV delimiter, JSON pointer, etc.)
- `tableSchema` — inferred from the data (column types, required fields)
- `dataSchema` — inferred for JSON resources
- `integrity` — computed hash of the file content

Once inferred, the dataset can be serialised to a descriptor:

```python
import json

print(json.dumps(dataset.model_dump(exclude_none=True), indent=2))
```

Example output:

```json
{
  "resources": [
    {
      "name": "users",
      "data": "users.csv",
      "fileDialect": { "name": "csv", "delimiter": "," },
      "tableSchema": {
        "properties": {
          "id": { "type": "integer" },
          "name": { "type": "string" },
          "email": { "type": "string" }
        },
        "required": ["id", "name", "email"]
      }
    }
  ]
}
```

## Saving a Dataset

Save a dataset to a folder, zip, GitHub repository, or other target via plugins:

```python
from fairspec import save_dataset

# Save to a local folder (downloads all remote resources)
save_dataset(dataset, target="./local-dataset")

# Save to a zip archive
save_dataset(dataset, target="./dataset.zip")
```

`save_dataset` takes `SaveDatasetOptions`:

- `target` — output path (folder, zip file, or remote URL)
- `with_remote` — also include remote resources (when `True`, they are downloaded; when `False`, references are preserved)

For format-specific saving, use:

- `save_dataset_to_folder(dataset, target=...)`
- `save_dataset_to_zip(dataset, target=...)`
- `save_dataset_to_github(dataset, target=...)`
- `save_dataset_to_zenodo(dataset, target=...)`
- `save_dataset_to_ckan(dataset, target=...)`

## Validating a Dataset

Validate a dataset descriptor and all its resources:

```python
from fairspec import validate_dataset

# Validate from a path (loads + validates)
report = validate_dataset("dataset.json")

# Validate a Dataset object directly
report = validate_dataset(dataset)

if report.valid:
    print("Dataset is valid")
else:
    for error in report.errors:
        print(error)
```

`validate_dataset` checks:

- **Descriptor validity** — valid JSON conforming to the Fairspec specification
- **Resource existence** — all referenced files can be loaded
- **Schema validation** — each resource validates against its Table Schema
- **Referential integrity** — foreign-key relationships are valid
- **Format compliance** — resources match their declared dialects

The returned `Report` has two fields:

- `valid: bool` — whether validation passed
- `errors: list[FairspecError]` — the list of errors (each carrying a `type`, `message`, and context fields like `resourceName`, `rowNumber`, `propertyName`)

Example error inspection:

```python
report = validate_dataset("dataset.json")

for error in report.errors:
    print(f"[{error.type}] {error.resourceName}: {error.message}")
```

## Validating Foreign Keys

Validate only the foreign-key relationships between resources (skipping per-resource schema checks):

```python
from fairspec import validate_dataset_foreign_keys

report = validate_dataset_foreign_keys(dataset)
```

This is useful when you have already validated each resource independently and only want to confirm cross-resource referential integrity.

## Rendering a Dataset

Render a dataset as Markdown, JSON, or other formats:

```python
from fairspec import RenderDatasetOptions, render_dataset_as

markdown = render_dataset_as(dataset, RenderDatasetOptions(format="markdown"))
```

Output formats depend on the available render plugins. The result is a string or `None` if no plugin handles the requested format.

## Working with Resources

A `Dataset` exposes its resources via the `resources` attribute. Iterate to access each resource's metadata:

```python
dataset = Dataset.model_validate(load_dataset("dataset.json"))

for resource in dataset.resources or []:
    print(resource.name, resource.data)
```

To load a single resource as a table, pass it to `load_table` (see [Tables](/python/table/)):

```python
from fairspec import load_table

users = load_table(dataset.resources[0])
```

To validate a single resource without re-validating the whole dataset, use `validate_resource` (see [Tables](/python/table/) for the table-level operations and [Files](/python/file/) for file-level operations).

## Common Workflows

### Create a Dataset from Files

```python
from fairspec import Dataset, Resource, infer_dataset, validate_dataset

dataset = Dataset(
    name="sales-data",
    title="Sales Database Export",
    license="CC-BY-4.0",
    resources=[
        Resource(data="customers.csv"),
        Resource(data="orders.csv"),
        Resource(data="products.csv"),
    ],
)

dataset = infer_dataset(dataset)
report = validate_dataset(dataset)

assert report.valid, report.errors
```

### Clone a Remote Dataset

```python
from fairspec import load_dataset, save_dataset, validate_dataset, Dataset

descriptor = load_dataset("https://example.com/dataset.json")
dataset = Dataset.model_validate(descriptor)

save_dataset(dataset, target="./local-data", with_remote=True)

report = validate_dataset("./local-data/dataset.json")
assert report.valid
```

### Dataset Quality Assurance in CI

```python
import sys
from fairspec import validate_dataset

report = validate_dataset("dataset.json")

if report.valid:
    print("Dataset validation passed")
else:
    for error in report.errors:
        print(f"  [{error.type}] {error.resourceName}: {error.message}")
    sys.exit(1)
```

## Examples

### Create a Multi-Table Dataset with Foreign Keys

```python
from fairspec import (
    Dataset,
    ForeignKey,
    ForeignKeyReference,
    IntegerColumnProperty,
    Resource,
    StringColumnProperty,
    TableSchema,
    infer_dataset,
    validate_dataset,
)

dataset = Dataset(
    name="sales-data",
    title="Sales Database Export",
    license="CC-BY-4.0",
    resources=[
        Resource(
            name="customers",
            data="customers.csv",
            tableSchema=TableSchema(
                properties={
                    "id": IntegerColumnProperty(),
                    "name": StringColumnProperty(),
                },
                primaryKey=["id"],
            ),
        ),
        Resource(
            name="orders",
            data="orders.csv",
            tableSchema=TableSchema(
                properties={
                    "order_id": IntegerColumnProperty(),
                    "customer_id": IntegerColumnProperty(),
                },
                primaryKey=["order_id"],
                foreignKeys=[
                    ForeignKey(
                        columns=["customer_id"],
                        reference=ForeignKeyReference(
                            resource="customers",
                            columns=["id"],
                        ),
                    ),
                ],
            ),
        ),
    ],
)

report = validate_dataset(dataset)
assert report.valid, report.errors
```

### Download and Validate a Public Dataset

```python
from fairspec import Dataset, load_dataset, save_dataset, validate_dataset

descriptor = load_dataset("https://data.example.org/climate/dataset.json")
dataset = Dataset.model_validate(descriptor)

save_dataset(dataset, target="./climate-data", with_remote=True)

report = validate_dataset("./climate-data/dataset.json")
print(f"Valid: {report.valid}")
print(f"Resources: {[r.name for r in dataset.resources or []]}")
```

### Interactive Data Exploration

```python
from fairspec import Dataset, load_dataset, load_table

dataset = Dataset.model_validate(load_dataset("dataset.json"))

names = [r.name for r in dataset.resources or []]
print(names)

users_resource = next(r for r in dataset.resources or [] if r.name == "users")
users = load_table(users_resource)

print(users.head(5).collect())
```
