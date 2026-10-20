import json
from unittest.mock import MagicMock, patch

import pytest

from fairspec_dataset.plugins.github.models.tree import GithubTree

from ..github import make_github_api_request


class TestMakeGithubApiRequest:
    def test_returns_the_response_parsed_by_the_model(self):
        entry = {
            "path": "data",
            "mode": "040000",
            "type": "tree",
            "sha": "abc",
            "url": "https://api.github.com/repos/o/r/git/trees/abc",
        }
        with _mock_urlopen({"sha": "def", "tree": [entry], "truncated": False}):
            tree = make_github_api_request(
                endpoint="/repos/o/r/git/trees/main", model=GithubTree
            )

        assert len(tree.tree) == 1
        assert tree.tree[0].path == "data"
        assert tree.tree[0].size is None

    def test_rejects_a_response_that_does_not_match_the_model(self):
        with (
            _mock_urlopen({"message": "Not Found"}),
            pytest.raises(
                Exception,
                match=r"Github API returned an unexpected response from "
                r"/repos/o/r/git/trees/main[\s\S]*tree",
            ),
        ):
            make_github_api_request(
                endpoint="/repos/o/r/git/trees/main", model=GithubTree
            )


def _mock_urlopen(body: object):
    response = MagicMock()
    response.read.return_value = json.dumps(body).encode()
    response.__enter__.return_value = response
    return patch("urllib.request.urlopen", return_value=response)
