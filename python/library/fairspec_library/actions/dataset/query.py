from __future__ import annotations

import re
from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import Dataset, Descriptor, assert_dataset, infer_resource_name
from fairspec_table import query_tables
from fairspec_table.models.table import LoadTableOptions

from fairspec_library.actions.dataset.load import load_dataset
from fairspec_library.actions.table.load import load_table

if TYPE_CHECKING:
    from fairspec_table import Table


def query_dataset(
    source: Dataset | Descriptor | str,
    query: str,
    **options: Unpack[LoadTableOptions],
) -> Table:
    """Run a SQL query across a dataset, each table resource exposed under its name.

    Only the resources the query names are loaded.
    """
    if isinstance(source, str):
        descriptor = load_dataset(source)
        if descriptor is None:
            raise ValueError(f"No plugin can load the dataset: {source}")
        dataset = Dataset.model_validate(descriptor)
    elif isinstance(source, dict):
        dataset = assert_dataset(source)
    else:
        dataset = source

    words = set(re.findall(r"[a-z0-9_]+", query.lower()))
    resources = [
        (
            resource,
            resource.name or infer_resource_name(resource, resource_number=index + 1),
        )
        for index, resource in enumerate(dataset.resources or [])
    ]
    resources = [
        (resource, name) for resource, name in resources if name.lower() in words
    ]

    tables: dict[str, Table] = {}
    for resource, name in resources:
        if sum(1 for _, other in resources if other == name) > 1:
            raise ValueError(f'More than one resource is named "{name}"')

        # A word in the query can be a column or a literal, so a non-table is skipped
        table = load_table(resource, **options)
        if table is not None:
            tables[name] = table

    return query_tables(tables, query)
