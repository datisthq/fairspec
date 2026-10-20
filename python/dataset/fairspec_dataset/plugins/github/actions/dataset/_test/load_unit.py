import pytest

from ..load import load_dataset_from_github


@pytest.mark.vcr
class TestLoadDatasetFromGithub:
    def test_loads_a_dataset(self):
        dataset = load_dataset_from_github("https://github.com/roll/data")

        assert dataset["titles"] == [{"title": "roll/testdata"}]
        assert dataset["resources"]
        for resource in dataset["resources"]:
            assert resource["data"].startswith(
                "https://raw.githubusercontent.com/roll/testdata/"
            )
