from __future__ import annotations

import json
import os
import re

from fairspec_dataset.plugins.ckan.models.dataset import CkanDataset
from ..from_ckan import convert_dataset_from_ckan


def _load_fixture() -> CkanDataset:
    path = os.path.join(os.path.dirname(__file__), "fixtures", "ckan-dataset.json")
    with open(path) as f:
        return CkanDataset(**json.load(f))


class TestConvertDatasetFromCkan:
    def test_converts_ckan_dataset_to_fairspec_dataset(self):
        ckan_dataset = _load_fixture()

        result = convert_dataset_from_ckan(ckan_dataset)

        assert result.titles is not None
        assert len(result.titles) == 1
        assert result.titles[0].title == ckan_dataset.title

        assert result.descriptions is not None
        assert len(result.descriptions) == 1
        assert result.descriptions[0].description == ckan_dataset.notes
        assert result.descriptions[0].descriptionType == "Abstract"

        assert result.version == ckan_dataset.version

        assert result.dates is not None
        assert len(result.dates) == 2
        created_date = next(d for d in result.dates if d.dateType == "Created")
        assert created_date.date == ckan_dataset.metadata_created
        updated_date = next(d for d in result.dates if d.dateType == "Updated")
        assert updated_date.date == ckan_dataset.metadata_modified

        assert result.rightsList is not None
        assert len(result.rightsList) == 1
        rights = result.rightsList[0]
        assert rights.rights == ckan_dataset.license_title
        assert rights.rightsUri == ckan_dataset.license_url
        assert rights.rightsIdentifier == ckan_dataset.license_id

        assert result.creators is not None
        assert len(result.creators) == 1
        assert result.creators[0].name == ckan_dataset.author
        assert result.creators[0].nameType == "Personal"

        assert result.contributors is not None
        assert len(result.contributors) == 1
        assert result.contributors[0].name == ckan_dataset.maintainer
        assert result.contributors[0].nameType == "Personal"
        assert result.contributors[0].contributorType == "ContactPerson"

        assert result.subjects is not None
        assert ckan_dataset.tags is not None
        assert len(result.subjects) == len(ckan_dataset.tags)
        assert [s.subject for s in result.subjects] == [
            tag.name for tag in ckan_dataset.tags
        ]

        assert result.resources is not None
        assert ckan_dataset.resources is not None
        assert len(result.resources) == len(ckan_dataset.resources)

        first_ckan_resource = ckan_dataset.resources[0]
        first_resource = result.resources[0]
        assert first_resource.data == first_ckan_resource.url
        assert first_resource.name is not None
        assert re.match(r"^sample[-_]linked[-_]csv$", first_resource.name)
        assert first_resource.descriptions is not None
        assert (
            first_resource.descriptions[0].description == first_ckan_resource.description
        )

    def test_handles_empty_resources_array(self):
        ckan_dataset = _load_fixture().model_copy(update={"resources": []})

        result = convert_dataset_from_ckan(ckan_dataset)

        assert result.resources == []

    def test_handles_undefined_optional_properties(self):
        ckan_dataset = CkanDataset(resources=[], tags=[], id="test", name="test")

        result = convert_dataset_from_ckan(ckan_dataset)

        assert result.titles is None
        assert result.descriptions is None
        assert result.version is None
        assert result.dates is None
        assert result.rightsList is None
        assert result.creators is None
        assert result.contributors is None
        assert result.subjects is None
        assert result.resources == []
