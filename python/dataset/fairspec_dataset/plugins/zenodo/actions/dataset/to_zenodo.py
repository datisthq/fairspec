from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata import Dataset


def convert_dataset_to_zenodo(dataset: Dataset) -> dict:
    metadata: dict = {"upload_type": "dataset"}

    titles = dataset.titles or []
    if titles and titles[0].title:
        metadata["title"] = titles[0].title

    descriptions = dataset.descriptions or []
    if descriptions and descriptions[0].description:
        metadata["description"] = descriptions[0].description
    elif titles and titles[0].title:
        metadata["description"] = titles[0].title
    else:
        metadata["description"] = "Dataset created with fairspec"

    if dataset.version:
        metadata["version"] = dataset.version

    rights_list = dataset.rightsList or []
    if rights_list:
        rights = rights_list[0]
        if rights.rights:
            metadata["license"] = rights.rights

    creators = dataset.creators or []
    if creators:
        metadata["creators"] = []
        for creator in creators:
            zenodo_creator: dict = {"name": creator.name}
            affiliations = creator.affiliation or []
            if affiliations and affiliations[0].name:
                zenodo_creator["affiliation"] = affiliations[0].name
            metadata["creators"].append(zenodo_creator)
    else:
        metadata["creators"] = [
            {"name": "Unknown Author", "affiliation": "Unknown Affiliation"}
        ]

    subjects = dataset.subjects or []
    if subjects:
        metadata["keywords"] = [s.subject for s in subjects]

    dates = dataset.dates or []
    if dates:
        issued = next((d for d in dates if d.dateType == "Issued"), None)
        if issued:
            metadata["publication_date"] = issued.date

    if dataset.doi:
        metadata["doi"] = dataset.doi

    return {"metadata": metadata}
