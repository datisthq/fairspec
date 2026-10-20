import json
import os
from unittest.mock import MagicMock, patch

from fairspec_metadata import CsvFileDialect, Dataset, Resource, Title

from ..save import save_dataset_to_zenodo

FIXTURES = os.path.join(os.path.dirname(__file__), "fixtures")


class TestSaveDatasetToZenodo:
    def test_saves_a_dataset(self):
        deposition = _read_response("deposition.json")
        file = _read_response("file.json")

        with _mock_urlopen([deposition, file, file]) as urlopen:
            result = save_dataset_to_zenodo(
                _create_dataset(), api_key="test-api-key", sandbox=True
            )

        assert result == {
            "path": "https://sandbox.zenodo.org/records/12345/files/dataset.json",
            "dataset_url": "https://sandbox.zenodo.org/uploads/12345",
        }
        requests = [call.args[0] for call in urlopen.call_args_list]
        assert [request.full_url.split("?")[0] for request in requests] == [
            "https://sandbox.zenodo.org/api/deposit/depositions",
            "https://sandbox.zenodo.org/api/deposit/depositions/12345/files",
            "https://sandbox.zenodo.org/api/deposit/depositions/12345/files",
        ]


def _create_dataset() -> Dataset:
    return Dataset(
        titles=[Title(title="Test Package")],
        resources=[
            Resource(
                name="test_resource",
                data=os.path.join(FIXTURES, "data.csv"),
                fileDialect=CsvFileDialect(),
            )
        ],
    )


def _read_response(name: str) -> dict:
    with open(os.path.join(FIXTURES, "responses", name), encoding="utf-8") as file:
        return json.load(file)


def _mock_urlopen(bodies: list[object]):
    responses = []
    for body in bodies:
        response = MagicMock()
        response.read.return_value = json.dumps(body).encode()
        response.__enter__.return_value = response
        responses.append(response)
    return patch("urllib.request.urlopen", side_effect=responses)
