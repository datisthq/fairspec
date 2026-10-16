from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import (
    ContributorType,
    CreatorNameType,
    DateType,
    DescriptionType,
)
from fairspec_metadata import Contributor
from fairspec_metadata import Creator
from fairspec_metadata import DataciteDate
from fairspec_metadata import DataciteDescription
from fairspec_metadata import Rights
from fairspec_metadata import Subject
from fairspec_metadata import Title
from fairspec_metadata import Dataset

from fairspec_dataset.plugins.ckan.actions.resource.from_ckan import (
    convert_resource_from_ckan,
)

if TYPE_CHECKING:
    from fairspec_dataset.plugins.ckan.models.dataset import CkanDataset


def convert_dataset_from_ckan(ckan_dataset: CkanDataset) -> Dataset:
    titles = [Title(title=ckan_dataset.title)] if ckan_dataset.title else None

    descriptions = (
        [
            DataciteDescription(
                description=ckan_dataset.notes, descriptionType=DescriptionType.Abstract
            )
        ]
        if ckan_dataset.notes
        else None
    )

    version = ckan_dataset.version

    resources = ckan_dataset.resources or []
    resource_list = (
        [convert_resource_from_ckan(r) for r in resources] if resources else []
    )

    rights_list = None
    if ckan_dataset.license_id or ckan_dataset.license_title:
        rights_list = [
            Rights(
                rights=ckan_dataset.license_title or ckan_dataset.license_id or "",
                rightsUri=ckan_dataset.license_url,
                rightsIdentifier=ckan_dataset.license_id,
            )
        ]

    creators = (
        [Creator(name=ckan_dataset.author, nameType=CreatorNameType.Personal)]
        if ckan_dataset.author
        else None
    )

    contributors = None
    if ckan_dataset.maintainer:
        contributors = [
            Contributor(
                name=ckan_dataset.maintainer,
                nameType=CreatorNameType.Personal,
                contributorType=ContributorType.ContactPerson,
            )
        ]

    tags = ckan_dataset.tags or []
    subjects = [Subject(subject=tag.name or "") for tag in tags] if tags else None

    dates: list[DataciteDate] = []
    if ckan_dataset.metadata_created:
        dates.append(
            DataciteDate(date=ckan_dataset.metadata_created, dateType=DateType.Created)
        )
    if ckan_dataset.metadata_modified:
        dates.append(
            DataciteDate(date=ckan_dataset.metadata_modified, dateType=DateType.Updated)
        )

    return Dataset(
        titles=titles,
        descriptions=descriptions,
        version=version,
        resources=resource_list,
        rightsList=rights_list,
        creators=creators,
        contributors=contributors,
        subjects=subjects,
        dates=dates if dates else None,
    )
