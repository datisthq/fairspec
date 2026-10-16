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

from fairspec_dataset.plugins.github.actions.resource.from_github import (
    convert_resource_from_github,
)

if TYPE_CHECKING:
    from fairspec_dataset.plugins.github.models.repository import GithubRepository


def convert_dataset_from_github(repository: GithubRepository) -> Dataset:
    titles = [Title(title=repository.full_name)] if repository.full_name else None

    descriptions = (
        [
            DataciteDescription(
                description=repository.description,
                descriptionType=DescriptionType.Abstract,
            )
        ]
        if repository.description
        else None
    )

    rights_list = None
    license_info = repository.license
    if license_info:
        rights_list = [
            Rights(
                rights=license_info.name or "",
                rightsUri=license_info.url,
                rightsIdentifier=license_info.spdx_id or license_info.key,
                rightsIdentifierScheme="SPDX",
            )
        ]

    creators = None
    contributors = None
    owner = repository.owner
    if owner:
        if owner.type == "Organization":
            contributors = [
                Contributor(
                    name=owner.login or "",
                    nameType=CreatorNameType.Organizational,
                    contributorType=ContributorType.HostingInstitution,
                )
            ]
        else:
            creators = [
                Creator(
                    name=owner.login or "",
                    nameType=CreatorNameType.Personal,
                )
            ]

    files = repository.files or []
    resource_list = []
    if files:
        default_branch = repository.default_branch or "main"
        resource_list = [
            convert_resource_from_github(f, default_branch=default_branch)
            for f in files
            if not (f.path or "").startswith(".") and f.type == "blob"
        ]

    topics = repository.topics or []
    subjects = [Subject(subject=topic) for topic in topics] if topics else None

    dates = (
        [DataciteDate(date=repository.created_at, dateType=DateType.Created)]
        if repository.created_at
        else None
    )

    return Dataset(
        titles=titles,
        descriptions=descriptions,
        rightsList=rights_list,
        creators=creators,
        contributors=contributors,
        resources=resource_list,
        subjects=subjects,
        dates=dates,
    )
