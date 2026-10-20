import json
import os
from unittest.mock import MagicMock, patch

from fairspec_metadata import CsvFileDialect, Dataset, Resource, Title

from ..save import save_dataset_to_ckan

FIXTURES = os.path.join(os.path.dirname(__file__), "fixtures")


class TestSaveDatasetToCkan:
    def test_saves_a_dataset(self):
        package = _read_response("package_create.json")
        resources = [_create_resource("resource-1"), _create_resource("resource-2")]
        bodies = [{"success": True, "result": body} for body in [package, *resources]]

        with _mock_urlopen(bodies) as urlopen:
            result = save_dataset_to_ckan(
                _create_dataset(),
                api_key="test-api-key",
                ckan_url="https://ckan.example.com",
                owner_org="test-org",
                dataset_name="test-dataset",
            )

        assert result == {
            "path": resources[1]["url"],
            "dataset_url": "https://ckan.example.com/dataset/test-dataset",
        }
        requests = [call.args[0] for call in urlopen.call_args_list]
        assert [request.full_url for request in requests] == [
            "https://ckan.example.com/api/3/action/package_create",
            "https://ckan.example.com/api/3/action/resource_create",
            "https://ckan.example.com/api/3/action/resource_create",
        ]
        assert b'filename="data.csv"' in requests[1].data
        assert b'filename="dataset.json"' in requests[2].data

    def test_saved_descriptor_is_found_by_load(self):
        package = _read_response("package_create.json")
        resources = [_create_resource("resource-1"), _create_resource("resource-2")]
        bodies = [{"success": True, "result": body} for body in [package, *resources]]

        with _mock_urlopen(bodies) as urlopen:
            save_dataset_to_ckan(
                _create_dataset(),
                api_key="test-api-key",
                ckan_url="https://ckan.example.com",
                owner_org="test-org",
                dataset_name="test-dataset",
            )

        descriptor_request = urlopen.call_args_list[2].args[0]
        assert b'name="name"\r\n\r\ndataset.json' in descriptor_request.data


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


def _create_resource(resource_id: str) -> dict:
    return {
        **_read_response("resource_create.json"),
        "id": resource_id,
        "url": f"https://ckan.example.com/dataset/test-dataset/resource/{resource_id}/download/data",
    }


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
