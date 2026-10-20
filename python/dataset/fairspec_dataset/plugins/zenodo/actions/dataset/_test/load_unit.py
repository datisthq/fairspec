import pytest

from ..load import load_dataset_from_zenodo


@pytest.mark.vcr
class TestLoadDatasetFromZenodo:
    def test_loads_a_dataset(self):
        dataset = load_dataset_from_zenodo("https://zenodo.org/records/15525711")

        assert dataset["titles"]
        assert dataset["resources"]
        assert dataset["rightsList"] == [{"rights": "cc-by-4.0"}]
        assert dataset["creators"][0] == {
            "name": "Sukhija, Vinay Jagdish",
            "nameType": "Personal",
        }
