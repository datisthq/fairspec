---
title: Working with Datasets in Rust
label: Dataset
path: /rust/library/dataset/
order: 1
---

Dataset operations for managing collections of tabular resources with metadata and schemas — from Rust.

## Available Functions

The dataset API provides utilities for working with datasets:

- `load_dataset` - Load a dataset descriptor from a local path or remote URL
- `infer_dataset` - Automatically fill in missing metadata for every resource
- `save_dataset` - Save a dataset (and its resources) via plugins
- `validate_dataset` - Validate a dataset descriptor and all its resources
- `query_dataset` - Run a SQL query across the dataset's tables
- `validate_dataset_foreign_keys` - Check foreign-key relationships across resources
- `render_dataset_as` - Render a dataset through the render plugins

## What is a Dataset?

A dataset is a collection of related data resources (tables) with:

- Metadata describing the dataset (title, description, license, etc.)
- Resource definitions for each table (path, dialect, schema)
- Table Schemas defining the structure of each resource
- Relationships and foreign keys between resources

Datasets are represented in Rust by the `Dataset` struct and serialised as JSON descriptors (often named `dataset.json`) following the Fairspec specification. Its metadata follows DataCite, so a title is an entry in `titles` and a license one in `rightsList`:

```rust
use fairspec::fairspec_metadata::Dataset;
use serde_json::json;

let dataset: Dataset = serde_json::from_value(json!({
    "titles": [{"title": "Sales Database Export"}],
    "rightsList": [{"rights": "CC-BY-4.0"}],
    "resources": [
        {"name": "users", "data": "users.csv"},
        {"name": "orders", "data": "orders.csv"},
    ],
}))?;
```

## Loading a Dataset

Load a dataset descriptor from a local path or remote URL:

```rust
use fairspec::load_dataset;

// Load from local file
let descriptor = load_dataset("dataset.json")?;

// Load from a remote URL
let descriptor = load_dataset("https://example.com/dataset.json")?;
```

`load_dataset` dispatches to the appropriate plugin based on the source (folder, zip, GitHub, Zenodo, CKAN, …) and returns `Option<Descriptor>` — a `serde_json::Map` — or `None` if no plugin recognises the source.

To get a typed `Dataset`, deserialise the descriptor:

```rust
use fairspec::fairspec_metadata::Dataset;
use fairspec::load_dataset;

let descriptor = load_dataset("dataset.json")?.ok_or("unsupported source")?;
let dataset: Dataset = serde_json::from_value(descriptor.into())?;
```

For format-specific loaders, use (from `fairspec::fairspec_dataset`):

- `load_dataset_from_folder(path)`
- `load_dataset_from_zip(path)`
- `load_dataset_from_github(url, api_key)`
- `load_dataset_from_zenodo(url, api_key)`
- `load_dataset_from_ckan(url, api_key)`

## Inferring a Dataset

Automatically fill in missing metadata for every resource in a dataset:

```rust
use fairspec::fairspec_metadata::Dataset;
use fairspec::fairspec_table::InferTableSchemaOptions;
use fairspec::infer_dataset;
use serde_json::json;

let dataset: Dataset = serde_json::from_value(json!({
    "resources": [{"data": "users.csv"}, {"data": "orders.csv"}],
}))?;

let dataset = infer_dataset(&dataset, &InferTableSchemaOptions::default())?;
```

The inference process automatically fills in:

- `name` — generated from the file name
- `fileDialect` — detected from the file content (CSV delimiter, JSON pointer, etc.)
- `tableSchema` — inferred from the data (column types, required fields)
- `dataSchema` — inferred for JSON resources
- `integrity` — computed hash of the file content

Once inferred, the dataset can be serialised to a descriptor:

```rust
println!("{}", serde_json::to_string_pretty(&dataset)?);
```

Example output:

```json
{
  "resources": [
    {
      "name": "users",
      "data": "users.csv",
      "fileDialect": { "format": "csv", "delimiter": "," },
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

Save a dataset to a folder, zip, or other target via plugins:

```rust
use fairspec::fairspec_dataset::SaveDatasetOptions;
use fairspec::save_dataset;

// Save to a local folder
save_dataset(&dataset, &SaveDatasetOptions {
    target: "./local-dataset".into(),
    with_remote: false,
})?;

// Save to a zip archive
save_dataset(&dataset, &SaveDatasetOptions {
    target: "./dataset.zip".into(),
    with_remote: false,
})?;
```

`save_dataset` takes `SaveDatasetOptions`:

- `target` — output path (folder or zip file)
- `with_remote` — also include remote resources (when `true`, they are downloaded; when `false`, references are preserved)

For format-specific saving, use (from `fairspec::fairspec_dataset`):

- `save_dataset_to_folder(&dataset, folder_path, with_remote)`
- `save_dataset_to_zip(&dataset, archive_path, with_remote)`
- `save_dataset_to_github(&dataset, api_key, repo, org)`
- `save_dataset_to_zenodo(&dataset, api_key, sandbox)`
- `save_dataset_to_ckan(&dataset, api_key, ckan_url, owner_org, dataset_name)`

## Validating a Dataset

Validate a dataset descriptor and all its resources:

```rust
use fairspec::{ValidateTableOptions, validate_dataset};

// Validate from a path (loads + validates)
let report = validate_dataset("dataset.json", &ValidateTableOptions::default())?;

// Validate a Dataset directly
let report = validate_dataset(dataset, &ValidateTableOptions::default())?;

if report.valid {
    println!("Dataset is valid");
} else {
    for error in &report.errors {
        println!("{}", serde_json::to_string(error)?);
    }
}
```

`validate_dataset` takes a path or URL, a `Descriptor` or a `Dataset`, and checks:

- **Descriptor validity** — valid JSON conforming to the Fairspec specification
- **Resource existence** — all referenced files can be loaded
- **Schema validation** — each resource validates against its Table Schema
- **Referential integrity** — foreign-key relationships are valid
- **Format compliance** — resources match their declared dialects

The returned `Report` has two fields:

- `valid: bool` — whether validation passed
- `errors: Vec<FairspecError>` — the list of errors; serialised, each carries a `type` and context fields like `resourceName`, `rowNumber`, `columnName`

Example error inspection:

```rust
use fairspec::{ValidateTableOptions, validate_dataset};

let report = validate_dataset("dataset.json", &ValidateTableOptions::default())?;

for error in &report.errors {
    let error = serde_json::to_value(error)?;
    println!("[{}] {}", error["type"], error["resourceName"]);
}
```

## Querying a Dataset

`query_dataset(source, sql, options)` runs a SQL query across a dataset's tables using Polars' SQL engine. Each table resource is exposed under its name (an unnamed one under the name inferred from its file), so one query can join several resources. Only the resources the query mentions are loaded.

```rust
use fairspec::fairspec_table::LoadTableOptions;
use fairspec::query_dataset;

let result = query_dataset(
    "dataset.json",
    "SELECT users.name, COUNT(*) AS orders FROM orders JOIN users ON orders.user_id = users.id GROUP BY users.name",
    &LoadTableOptions::default(),
)?;

println!("{}", result.collect()?);
```

`source` is a path, a `Descriptor` or a `Dataset`. To query a single table as `self`, load it and use `query_table`.

## Validating Foreign Keys

Validate only the foreign-key relationships between resources (skipping per-resource schema checks):

```rust
use fairspec::{ValidateTableOptions, validate_dataset_foreign_keys};

let report = validate_dataset_foreign_keys(&dataset, &ValidateTableOptions::default())?;
```

This is useful when you have already validated each resource independently and only want to confirm cross-resource referential integrity.

## Rendering a Dataset

`render_dataset_as` asks the render plugins for the requested format:

```rust
use fairspec::fairspec_metadata::RenderDatasetOptions;
use fairspec::render_dataset_as;

let markdown = render_dataset_as(&dataset, &RenderDatasetOptions {
    format: "markdown".into(),
    ..Default::default()
})?;
```

No Rust plugin renders yet, so the result is `None` for every format.

## Working with Resources

A `Dataset` exposes its resources via the `resources` field. Iterate to access each resource's metadata:

```rust
for resource in dataset.resources.iter().flatten() {
    println!("{:?} {:?}", resource.name, resource.data);
}
```

To load a single resource as a table, pass it to `load_table` (see [Tables](/rust/library/table/)):

```rust
use fairspec::fairspec_table::LoadTableOptions;
use fairspec::load_table;

let resources = dataset.resources.unwrap_or_default();
let users = load_table(&resources[0], &LoadTableOptions::default())?;
```

To validate a single resource without re-validating the whole dataset, use `validate_resource` (see [Tables](/rust/library/table/) for the table-level operations and [Files](/rust/library/file/) for file-level operations).

## Common Workflows

### Create a Dataset from Files

```rust
use fairspec::fairspec_metadata::Dataset;
use fairspec::fairspec_table::InferTableSchemaOptions;
use fairspec::{ValidateTableOptions, infer_dataset, validate_dataset};
use serde_json::json;

let dataset: Dataset = serde_json::from_value(json!({
    "titles": [{"title": "Sales Database Export"}],
    "rightsList": [{"rights": "CC-BY-4.0"}],
    "resources": [
        {"data": "customers.csv"},
        {"data": "orders.csv"},
        {"data": "products.csv"},
    ],
}))?;

let dataset = infer_dataset(&dataset, &InferTableSchemaOptions::default())?;
let report = validate_dataset(dataset, &ValidateTableOptions::default())?;

assert!(report.valid, "{:?}", report.errors);
```

### Clone a Remote Dataset

```rust
use fairspec::fairspec_dataset::SaveDatasetOptions;
use fairspec::fairspec_metadata::Dataset;
use fairspec::{ValidateTableOptions, load_dataset, save_dataset, validate_dataset};

let descriptor = load_dataset("https://example.com/dataset.json")?.ok_or("unsupported source")?;
let dataset: Dataset = serde_json::from_value(descriptor.into())?;

save_dataset(&dataset, &SaveDatasetOptions {
    target: "./local-data".into(),
    with_remote: true,
})?;

let report = validate_dataset("./local-data/dataset.json", &ValidateTableOptions::default())?;
assert!(report.valid);
```

### Dataset Quality Assurance in CI

```rust
use fairspec::{ValidateTableOptions, validate_dataset};

let report = validate_dataset("dataset.json", &ValidateTableOptions::default())?;

if report.valid {
    println!("Dataset validation passed");
} else {
    for error in &report.errors {
        println!("  {}", serde_json::to_string(error)?);
    }
    std::process::exit(1);
}
```

## Examples

### Create a Multi-Table Dataset with Foreign Keys

```rust
use fairspec::fairspec_metadata::Dataset;
use fairspec::{ValidateTableOptions, validate_dataset};
use serde_json::json;

let dataset: Dataset = serde_json::from_value(json!({
    "titles": [{"title": "Sales Database Export"}],
    "rightsList": [{"rights": "CC-BY-4.0"}],
    "resources": [
        {
            "name": "customers",
            "data": "customers.csv",
            "tableSchema": {
                "properties": {
                    "id": {"type": "integer"},
                    "name": {"type": "string"},
                },
                "primaryKey": ["id"],
            },
        },
        {
            "name": "orders",
            "data": "orders.csv",
            "tableSchema": {
                "properties": {
                    "order_id": {"type": "integer"},
                    "customer_id": {"type": "integer"},
                },
                "primaryKey": ["order_id"],
                "foreignKeys": [{
                    "columns": ["customer_id"],
                    "reference": {"resource": "customers", "columns": ["id"]},
                }],
            },
        },
    ],
}))?;

let report = validate_dataset(dataset, &ValidateTableOptions::default())?;
assert!(report.valid, "{:?}", report.errors);
```

### Download and Validate a Public Dataset

```rust
use fairspec::fairspec_dataset::SaveDatasetOptions;
use fairspec::fairspec_metadata::Dataset;
use fairspec::{ValidateTableOptions, load_dataset, save_dataset, validate_dataset};

let descriptor = load_dataset("https://data.example.org/climate/dataset.json")?
    .ok_or("unsupported source")?;
let dataset: Dataset = serde_json::from_value(descriptor.into())?;

save_dataset(&dataset, &SaveDatasetOptions {
    target: "./climate-data".into(),
    with_remote: true,
})?;

let report = validate_dataset("./climate-data/dataset.json", &ValidateTableOptions::default())?;
println!("Valid: {}", report.valid);
let names: Vec<_> = dataset.resources.iter().flatten().map(|r| &r.name).collect();
println!("Resources: {names:?}");
```

### Interactive Data Exploration

```rust
use fairspec::fairspec_metadata::Dataset;
use fairspec::fairspec_table::LoadTableOptions;
use fairspec::{load_dataset, load_table};

let descriptor = load_dataset("dataset.json")?.ok_or("unsupported source")?;
let dataset: Dataset = serde_json::from_value(descriptor.into())?;
let resources = dataset.resources.unwrap_or_default();

let names: Vec<_> = resources.iter().map(|r| &r.name).collect();
println!("{names:?}");

let users_resource = resources
    .iter()
    .find(|r| r.name.as_deref() == Some("users"))
    .ok_or("no users resource")?;
let users = load_table(users_resource, &LoadTableOptions::default())?.ok_or("unsupported format")?;

println!("{}", users.limit(5).collect()?);
```
