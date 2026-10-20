import json
import os
from unittest.mock import MagicMock, patch

from fairspec_metadata import CsvFileDialect, Dataset, Resource, Title

from ..save import save_dataset_to_github

FIXTURES = os.path.join(os.path.dirname(__file__), "fixtures")


class TestSaveDatasetToGithub:
    def test_saves_a_dataset(self):
        repository = _read_response("repository.json")
        content = _read_response("content.json")

        with _mock_urlopen([repository, content, content]) as urlopen:
            result = save_dataset_to_github(
                _create_dataset(), api_key="test-api-key", repo="test-repo"
            )

        assert result == {
            "path": "https://raw.githubusercontent.com/test-user/test-repo/refs/heads/main/dataset.json",
            "repo_url": "https://github.com/test-user/test-repo",
        }
        requests = [call.args[0] for call in urlopen.call_args_list]
        assert [request.full_url for request in requests] == [
            "https://api.github.com/user/repos",
            "https://api.github.com/repos/test-user/test-repo/contents/data.csv",
            "https://api.github.com/repos/test-user/test-repo/contents/dataset.json",
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
