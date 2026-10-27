from __future__ import annotations

import re
from typing import TYPE_CHECKING

from fairspec_metadata.actions.frictionless.mapping import (
    create_frictionless_recorder,
    nest_frictionless_recorder,
    skip_frictionless_record,
)
from fairspec_metadata.actions.resource.from_frictionless import (
    convert_resource_from_frictionless,
)
from fairspec_metadata.actions.rights.from_frictionless import (
    convert_rights_from_frictionless,
)

if TYPE_CHECKING:
    from fairspec_metadata.actions.frictionless.mapping import FrictionlessMapping
    from fairspec_metadata.models.descriptor import Descriptor
    from fairspec_metadata.models.frictionless.package import (
        FrictionlessContributor,
        FrictionlessPackage,
    )

CONTRIBUTOR_TYPES = {
    "maintainer": "ContactPerson",
    "wrangler": "DataCurator",
    "publisher": "Distributor",
}


def convert_dataset_from_frictionless(
    frictionless_package: FrictionlessPackage,
    *,
    mapping: FrictionlessMapping | None = None,
) -> Descriptor:
    """Convert a Frictionless Data Package to a Fairspec Dataset descriptor

    A resource's `schema` or `dialect` path stays a path
    """
    record = (
        create_frictionless_recorder(mapping)
        if mapping is not None
        else skip_frictionless_record
    )
    dataset: Descriptor = {}

    doi = _convert_doi(frictionless_package.id)
    if doi:
        dataset["doi"] = doi
        record("/doi", "/id")

    creators: list[Descriptor] = []
    contributors: list[Descriptor] = []
    for index, contributor in enumerate(frictionless_package.contributors or []):
        person = _convert_person(contributor)
        if not person:
            continue

        roles = contributor.roles
        if roles is None:
            roles = [contributor.role] if contributor.role else []
        if "author" in roles or "creator" in roles:
            record(f"/creators/{len(creators)}", f"/contributors/{index}")
            creators.append(person)
        else:
            role = next((role for role in roles if role in CONTRIBUTOR_TYPES), None)
            record(f"/contributors/{len(contributors)}", f"/contributors/{index}")
            contributors.append(
                {
                    **person,
                    "contributorType": CONTRIBUTOR_TYPES[role] if role else "Other",
                }
            )

    if creators:
        dataset["creators"] = creators

    if frictionless_package.title:
        dataset["titles"] = [{"title": frictionless_package.title}]
        record("/titles", "/title")

    if frictionless_package.keywords:
        dataset["subjects"] = [
            {"subject": subject} for subject in frictionless_package.keywords
        ]
        for index in range(len(frictionless_package.keywords)):
            record(f"/subjects/{index}", f"/keywords/{index}")

    if contributors:
        dataset["contributors"] = contributors

    if frictionless_package.created:
        dataset["dates"] = [{"date": frictionless_package.created, "dateType": "Created"}]
        record("/dates", "/created")

    related_identifiers: list[Descriptor] = []
    if frictionless_package.homepage:
        record(f"/relatedIdentifiers/{len(related_identifiers)}", "/homepage")
        related_identifiers.append(
            {
                "relatedIdentifier": frictionless_package.homepage,
                "relatedIdentifierType": "URL",
                "relationType": "IsDocumentedBy",
            }
        )
    for index, source in enumerate(frictionless_package.sources or []):
        if not source.path or not re.match(r"https?://", source.path):
            continue
        record(f"/relatedIdentifiers/{len(related_identifiers)}", f"/sources/{index}")
        related_identifiers.append(
            {
                "relatedIdentifier": source.path,
                "relatedIdentifierType": "URL",
                "relationType": "IsDerivedFrom",
            }
        )
    if related_identifiers:
        dataset["relatedIdentifiers"] = related_identifiers

    if frictionless_package.version:
        dataset["version"] = frictionless_package.version
        record("/version", "/version")

    if frictionless_package.licenses:
        dataset["rightsList"] = convert_rights_from_frictionless(
            frictionless_package.licenses,
            record=nest_frictionless_recorder(record, "/rightsList", "/licenses"),
        )

    if frictionless_package.description:
        dataset["descriptions"] = [
            {
                "description": frictionless_package.description,
                "descriptionType": "Abstract",
            }
        ]
        record("/descriptions", "/description")

    resources: list[Descriptor] = []
    for index, resource in enumerate(frictionless_package.resources):
        prefix = f"/resources/{index}"
        record(prefix, prefix, verbatim=True)
        converted = convert_resource_from_frictionless(
            resource, record=nest_frictionless_recorder(record, prefix, prefix)
        )
        name = converted.get("name")
        if mapping is not None and resource.name and isinstance(name, str):
            mapping.resource_names[name] = resource.name
        resources.append(converted)
    dataset["resources"] = resources
    record("/resources", "/resources", verbatim=True)

    return dataset


def _convert_person(contributor: FrictionlessContributor) -> Descriptor | None:
    name = contributor.title
    if name is None:
        parts = [part for part in (contributor.givenName, contributor.familyName) if part]
        name = " ".join(parts) or None
    if not name:
        return None

    person: Descriptor = {"name": name}
    if contributor.givenName or contributor.familyName:
        person["nameType"] = "Personal"
    if contributor.givenName:
        person["givenName"] = contributor.givenName
    if contributor.familyName:
        person["familyName"] = contributor.familyName
    if contributor.organization:
        person["affiliation"] = [{"name": contributor.organization}]
    return person


def _convert_doi(id: str | None) -> str | None:
    if id is None:
        return None
    doi = re.sub(r"^https?://(dx\.)?doi\.org/", "", id, count=1)
    return doi if re.fullmatch(r"10[.][0-9]{4,9}/\S+", doi) else None
