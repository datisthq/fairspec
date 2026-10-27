from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.actions.resource.to_frictionless import (
    convert_resource_to_frictionless,
)
from fairspec_metadata.actions.rights.to_frictionless import (
    convert_rights_to_frictionless,
)

if TYPE_CHECKING:
    from fairspec_metadata.models.descriptor import Descriptor

ROLES = {
    "ContactPerson": "maintainer",
    "DataCurator": "wrangler",
    "Distributor": "publisher",
}


def convert_dataset_to_frictionless(dataset: Descriptor) -> Descriptor:
    """Convert a denormalized Fairspec Dataset to a Frictionless Data Package descriptor

    Its resources' `tableSchema` and `fileDialect` must already be objects, not paths
    """
    frictionless_package: Descriptor = {
        "$schema": "https://datapackage.org/profiles/2.0/datapackage.json",
    }

    if dataset.get("doi"):
        frictionless_package["id"] = f"https://doi.org/{dataset['doi']}"

    title = (dataset.get("titles") or [{}])[0].get("title")
    if title:
        frictionless_package["title"] = title

    description = (dataset.get("descriptions") or [{}])[0].get("description")
    if description:
        frictionless_package["description"] = description

    related_identifiers: list[Descriptor] = dataset.get("relatedIdentifiers") or []
    homepage = next(
        (
            identifier
            for identifier in related_identifiers
            if identifier.get("relationType") == "IsDocumentedBy"
        ),
        None,
    )
    if homepage:
        frictionless_package["homepage"] = homepage["relatedIdentifier"]

    if dataset.get("version"):
        frictionless_package["version"] = dataset["version"]

    created = next(
        (
            date
            for date in dataset.get("dates") or []
            if date.get("dateType") == "Created"
        ),
        None,
    )
    if created:
        frictionless_package["created"] = created["date"]

    if dataset.get("subjects"):
        frictionless_package["keywords"] = [
            subject["subject"] for subject in dataset["subjects"]
        ]

    if dataset.get("rightsList"):
        frictionless_package["licenses"] = convert_rights_to_frictionless(
            dataset["rightsList"]
        )

    contributors = [
        *(
            _convert_person(creator, "author")
            for creator in dataset.get("creators") or []
        ),
        *(
            _convert_person(
                contributor, ROLES.get(contributor["contributorType"], "contributor")
            )
            for contributor in dataset.get("contributors") or []
        ),
    ]
    if contributors:
        frictionless_package["contributors"] = contributors

    sources = [
        {
            "title": identifier["relatedIdentifier"],
            "path": identifier["relatedIdentifier"],
        }
        for identifier in related_identifiers
        if identifier.get("relationType") == "IsDerivedFrom"
    ]
    if sources:
        frictionless_package["sources"] = sources

    frictionless_package["resources"] = [
        convert_resource_to_frictionless(resource, resource_number=index + 1)
        for index, resource in enumerate(dataset.get("resources") or [])
    ]

    return frictionless_package


def _convert_person(person: Descriptor, role: str) -> Descriptor:
    contributor: Descriptor = {"title": person["name"]}
    if person.get("givenName"):
        contributor["givenName"] = person["givenName"]
    if person.get("familyName"):
        contributor["familyName"] = person["familyName"]
    organization = (person.get("affiliation") or [{}])[0].get("name")
    if organization:
        contributor["organization"] = organization
    contributor["roles"] = [role]
    return contributor
