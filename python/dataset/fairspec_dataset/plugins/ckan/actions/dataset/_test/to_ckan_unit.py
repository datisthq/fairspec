from __future__ import annotations

import json
import os

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
from fairspec_metadata import Integrity, IntegrityType
from fairspec_metadata import CsvFileDialect
from fairspec_metadata import Resource

from fairspec_dataset.plugins.ckan.models.dataset import CkanDataset
from ..from_ckan import convert_dataset_from_ckan
from ..to_ckan import convert_dataset_to_ckan


def _load_fixture() -> CkanDataset:
    path = os.path.join(os.path.dirname(__file__), "fixtures", "ckan-dataset.json")
    with open(path) as f:
        return CkanDataset(**json.load(f))


class TestConvertDatasetToCkan:
    def test_converts_fairspec_dataset_to_ckan_dataset(self):
        dataset = Dataset(
            titles=[Title(title="Test Package")],
            descriptions=[
                DataciteDescription(
                    description="This is a test package",
                    descriptionType=DescriptionType.Abstract,
                )
            ],
            version="1.0.0",
            rightsList=[
                Rights(
                    rights="Creative Commons Attribution",
                    rightsUri="http://www.opendefinition.org/licenses/cc-by",
                    rightsIdentifier="cc-by",
                )
            ],
            creators=[Creator(name="Test Author", nameType=CreatorNameType.Personal)],
            contributors=[
                Contributor(
                    name="Test Maintainer",
                    nameType=CreatorNameType.Personal,
                    contributorType=ContributorType.ContactPerson,
                )
            ],
            subjects=[
                Subject(subject="test"),
                Subject(subject="sample"),
                Subject(subject="data"),
            ],
            dates=[
                DataciteDate(date="2023-01-01T00:00:00Z", dateType=DateType.Created),
                DataciteDate(date="2023-01-02T00:00:00Z", dateType=DateType.Updated),
            ],
            resources=[
                Resource(
                    name="test_resource",
                    data="https://example.com/data.csv",
                    fileDialect=CsvFileDialect(),
                    descriptions=[
                        DataciteDescription(
                            description="Test resource",
                            descriptionType=DescriptionType.Abstract,
                        )
                    ],
                    integrity=Integrity(type=IntegrityType.md5, hash="1234567890abcdef"),
                )
            ],
        )

        result = convert_dataset_to_ckan(dataset)

        assert result.title == "Test Package"
        assert result.notes == "This is a test package"
        assert result.version == "1.0.0"

        assert result.license_id == "cc-by"
        assert result.license_title == "Creative Commons Attribution"
        assert result.license_url == "http://www.opendefinition.org/licenses/cc-by"

        assert result.author == "Test Author"
        assert result.maintainer == "Test Maintainer"

        assert result.tags is not None
        assert len(result.tags) == 3
        for i, subject in enumerate(["test", "sample", "data"]):
            assert result.tags[i].name == subject
            assert result.tags[i].display_name == subject

        assert result.metadata_created == "2023-01-01T00:00:00Z"
        assert result.metadata_modified == "2023-01-02T00:00:00Z"

        assert result.resources is not None
        assert len(result.resources) == 1
        assert result.resources[0].name == "test_resource"
        assert result.resources[0].description == "Test resource"
        assert result.resources[0].hash == "1234567890abcdef"

    def test_handles_empty_resources_array(self):
        dataset = Dataset(resources=[])

        result = convert_dataset_to_ckan(dataset)

        assert result.resources == []

    def test_handles_undefined_optional_properties(self):
        dataset = Dataset(resources=[])

        result = convert_dataset_to_ckan(dataset)

        assert result.title is None
        assert result.notes is None
        assert result.version is None
        assert result.metadata_created is None
        assert result.metadata_modified is None
        assert result.license_id is None
        assert result.license_title is None
        assert result.license_url is None
        assert result.author is None
        assert result.maintainer is None
        assert result.tags == []
        assert result.resources == []

    def test_round_trip_ckan_to_dataset_to_ckan(self):
        original = _load_fixture()

        dataset = convert_dataset_from_ckan(original)
        result = convert_dataset_to_ckan(dataset)

        assert result.title == original.title
        assert result.notes == original.notes
        assert result.version == original.version

        assert result.license_id == original.license_id
        assert result.license_title == original.license_title
        assert result.license_url == original.license_url

        assert result.author == original.author
        assert result.maintainer == original.maintainer

        assert result.metadata_created == original.metadata_created
        assert result.metadata_modified == original.metadata_modified

        assert result.resources is not None
        assert len(result.resources) > 0

        assert result.tags is not None
        assert original.tags is not None
        assert len(result.tags) == len(original.tags)
        for original_tag in original.tags:
            matching = [t for t in result.tags if t.name == original_tag.name]
            assert len(matching) > 0
