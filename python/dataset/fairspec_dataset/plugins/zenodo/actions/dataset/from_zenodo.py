from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import CreatorNameType, DateType, DescriptionType
from fairspec_metadata import Creator, CreatorAffiliation
from fairspec_metadata import DataciteDate
from fairspec_metadata import DataciteDescription
from fairspec_metadata import Rights
from fairspec_metadata import Subject
from fairspec_metadata import Title
from fairspec_metadata import Dataset

from fairspec_dataset.plugins.zenodo.actions.resource.from_zenodo import (
    convert_resource_from_zenodo,
)

if TYPE_CHECKING:
    from fairspec_dataset.plugins.zenodo.models.record import ZenodoRecord


def convert_dataset_from_zenodo(zenodo_record: ZenodoRecord) -> Dataset:
    metadata = zenodo_record.metadata

    titles = [Title(title=metadata.title)] if metadata and metadata.title else None

    descriptions = (
        [
            DataciteDescription(
                description=metadata.description,
                descriptionType=DescriptionType.Abstract,
            )
        ]
        if metadata and metadata.description
        else None
    )

    creators_raw = metadata.creators or [] if metadata else []
    creators = None
    if creators_raw:
        creators = []
        for creator_data in creators_raw:
            affiliation = (
                [CreatorAffiliation(name=creator_data.affiliation)]
                if creator_data.affiliation
                else None
            )
            creators.append(
                Creator(
                    name=creator_data.name or "",
                    nameType=CreatorNameType.Personal,
                    affiliation=affiliation,
                )
            )

    keywords = metadata.keywords or [] if metadata else []
    subjects = [Subject(subject=kw) for kw in keywords] if keywords else None

    dates = (
        [DataciteDate(date=metadata.publication_date, dateType=DateType.Issued)]
        if metadata and metadata.publication_date
        else None
    )

    rights_list = (
        [Rights(rights=metadata.license)] if metadata and metadata.license else None
    )

    doi = metadata.doi if metadata else None
    version = metadata.version if metadata else None

    files = zenodo_record.files or []
    resource_list = [convert_resource_from_zenodo(f) for f in files] if files else []

    return Dataset(
        titles=titles,
        descriptions=descriptions,
        creators=creators,
        subjects=subjects,
        dates=dates,
        rightsList=rights_list,
        doi=doi,
        version=version,
        resources=resource_list,
    )
