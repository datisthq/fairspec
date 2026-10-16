from __future__ import annotations

import re
from typing import TYPE_CHECKING

from fairspec_metadata import get_file_name
from fairspec_metadata import DateType, DescriptionType
from fairspec_metadata import DataciteDate
from fairspec_metadata import DataciteDescription
from fairspec_metadata import Integrity, IntegrityType
from fairspec_metadata import Resource

from fairspec_dataset.plugins.ckan.actions.table_schema.from_ckan import (
    convert_table_schema_from_ckan,
)

if TYPE_CHECKING:
    from fairspec_dataset.plugins.ckan.models.resource import CkanResource


def convert_resource_from_ckan(ckan_resource: CkanResource) -> Resource:
    name = _convert_name(ckan_resource.name) if ckan_resource.name else None

    descriptions = (
        [
            DataciteDescription(
                description=ckan_resource.description,
                descriptionType=DescriptionType.Abstract,
            )
        ]
        if ckan_resource.description
        else None
    )

    sizes = [f"{ckan_resource.size} bytes"] if ckan_resource.size else None

    integrity = (
        Integrity(type=IntegrityType.md5, hash=ckan_resource.hash)
        if ckan_resource.hash
        else None
    )

    dates: list[DataciteDate] = []
    if ckan_resource.created:
        dates.append(DataciteDate(date=ckan_resource.created, dateType=DateType.Created))
    if ckan_resource.last_modified:
        dates.append(
            DataciteDate(date=ckan_resource.last_modified, dateType=DateType.Updated)
        )

    table_schema = (
        convert_table_schema_from_ckan(ckan_resource.schema_)
        if ckan_resource.schema_
        else None
    )

    return Resource(
        data=ckan_resource.url or "",
        name=name,
        descriptions=descriptions,
        sizes=sizes,
        integrity=integrity,
        dates=dates if dates else None,
        tableSchema=table_schema,
        unstable_customMetadata={
            "ckanKey": get_file_name(ckan_resource.url or ""),
            "ckanUrl": ckan_resource.url or "",
            "ckanId": ckan_resource.id,
        },
    )


def _convert_name(name: str) -> str:
    result = re.sub(r"[\s.()/\\,\-]+", "_", name).lower()
    result = re.sub(r"[^a-z0-9_]", "", result)
    result = re.sub(r"^(\d)", r"_\1", result)
    return result[:100]
