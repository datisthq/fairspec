from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_dataset.plugins.ckan.actions.resource.to_ckan import (
    convert_resource_to_ckan,
)
from fairspec_dataset.plugins.ckan.models.dataset import CkanDataset
from fairspec_dataset.plugins.ckan.models.tag import CkanTag

if TYPE_CHECKING:
    from fairspec_metadata import Dataset


def convert_dataset_to_ckan(dataset: Dataset) -> CkanDataset:
    title = None
    titles = dataset.titles or []
    if titles and titles[0].title:
        title = titles[0].title

    notes = None
    descriptions = dataset.descriptions or []
    if descriptions and descriptions[0].description:
        notes = descriptions[0].description

    version = dataset.version

    license_id = None
    license_title = None
    license_url = None
    rights_list = dataset.rightsList or []
    if rights_list:
        rights = rights_list[0]
        license_id = rights.rightsIdentifier
        license_title = rights.rights
        license_url = rights.rightsUri

    author = None
    creators = dataset.creators or []
    if creators and creators[0].name:
        author = creators[0].name

    maintainer = None
    contributors = dataset.contributors or []
    if contributors:
        maintainer_obj = next(
            (c for c in contributors if c.contributorType == "ContactPerson"),
            None,
        )
        if maintainer_obj and maintainer_obj.name:
            maintainer = maintainer_obj.name

    resource_list = None
    resources = dataset.resources or []
    if resources:
        resource_list = [
            r
            for r in (convert_resource_to_ckan(res) for res in resources)
            if r is not None
        ]
    else:
        resource_list = []

    tags = None
    subjects = dataset.subjects or []
    if subjects:
        tags = [CkanTag(name=s.subject, display_name=s.subject) for s in subjects]
    else:
        tags = []

    metadata_created = None
    metadata_modified = None
    dates = dataset.dates or []
    created_date = next((d for d in dates if d.dateType == "Created"), None)
    if created_date:
        metadata_created = created_date.date
    updated_date = next((d for d in dates if d.dateType == "Updated"), None)
    if updated_date:
        metadata_modified = updated_date.date

    return CkanDataset(
        title=title,
        notes=notes,
        version=version,
        license_id=license_id,
        license_title=license_title,
        license_url=license_url,
        author=author,
        maintainer=maintainer,
        resources=resource_list,
        tags=tags,
        metadata_created=metadata_created,
        metadata_modified=metadata_modified,
    )
