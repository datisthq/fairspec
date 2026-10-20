import pytest

from ..load import load_dataset_from_ckan


@pytest.mark.vcr
class TestLoadDatasetFromCkan:
    def test_loads_a_dataset(self):
        dataset = load_dataset_from_ckan(
            "https://data.gov.ie/dataset/referendum-on-the-thirty-seventh-amendment-of-the-constitution-bill-2018"
        )

        assert dataset["titles"]
        assert dataset["resources"]
        assert any("tableSchema" in resource for resource in dataset["resources"])
