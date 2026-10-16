# Fairspec Python

This guide will help you get started with Fairspec Python. If you are new to the core framework's technologies, please take a look at the [Fairspec standard](https://fairspec.org/) and [Polars DataFrames](https://pola.rs/) documentation.

## Runtimes

> [!TIP]
> - It is possible to use Fairspec Python in [Jupyter Notebooks](/python/jupyter)!

Fairspec Python requires:

- **Python 3.12+**

## Installation

The framework can be installed as one package:

```bash
pip install fairspec
```

You can cherry-pick from individual packages:

```bash
pip install fairspec-metadata fairspec-table
```

## Type Hints

Fairspec Python is built with type safety in mind. It uses Python type hints to provide type definitions for all packages and to enforce type safety throughout the framework. It's highly recommended to use a type-aware editor such as VS Code with Pylance or PyCharm to work with the project.

## Examples

Loading a Dataset from Zenodo merging system Zenodo metadata into a user dataset and validating its metadata:

```python
from fairspec import load_dataset

dataset = load_dataset("https://zenodo.org/records/10053903")

print(dataset)
# {
#   "id": "https://doi.org/10.5281/zenodo.10053903",
#   ...
# }
```

Validating an in-memory dataset descriptor:

```python
from fairspec import validate_dataset

report = validate_dataset({"resources": "bad"})

print(report.valid)
# False
print(report.errors)
# [
#   {
#     "type": "metadata",
#     "message": "must have type array",
#     "jsonPointer": "/resources",
#   }
# ]
```

Loading a dataset from a remote descriptor and saving it locally as a zip archive, and then using it as a local dataset:

```python
from fairspec import (
    Dataset,
    load_dataset,
    load_dataset_from_zip,
    save_dataset_to_zip,
    get_temp_file_path,
)

archive_path = get_temp_file_path()
source_dataset = Dataset.model_validate(
    load_dataset(
        "https://raw.githubusercontent.com/fairspec/fairspec-typescript/refs/heads/main/metadata/actions/dataset/fixtures/dataset.json",
    )
)

save_dataset_to_zip(source_dataset, archive_path=archive_path)
target_dataset = load_dataset_from_zip(archive_path)
print(target_dataset)
```

Reading a CSV table:

```python
from fairspec import load_table, Resource
from fairspec_metadata import CsvFileDialect

table = load_table(Resource(data="data.csv"))

# Load with custom format
table = load_table(Resource(
    data="data.csv",
    fileDialect=CsvFileDialect(
        delimiter=";",
        headerRows=[1],
    ),
))
```

## Reference

Note that `fairspec` and `fairspec-library` packages re-export most of the functionality.
