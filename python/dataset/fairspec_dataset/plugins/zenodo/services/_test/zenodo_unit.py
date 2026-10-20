import json
from unittest.mock import MagicMock, patch

import pytest

from fairspec_dataset.plugins.zenodo.models.deposition import ZenodoDeposition

from ..zenodo import make_zenodo_api_request


class TestMakeZenodoApiRequest:
    def test_returns_the_response_parsed_by_the_model(self):
        body = {"id": 1, "links": {"html": "https://zenodo.org/deposit/1"}, "x": 1}
        with _mock_urlopen(body):
            deposition = make_zenodo_api_request(
                endpoint="/deposit/depositions/1", model=ZenodoDeposition
            )

        assert deposition.id == 1
        assert deposition.links.html == "https://zenodo.org/deposit/1"

    def test_rejects_a_response_that_does_not_match_the_model(self):
        with (
            _mock_urlopen({"id": 1, "links": {}}),
            pytest.raises(
                Exception,
                match=r"Zenodo API returned an unexpected response from "
                r"/deposit/depositions/1[\s\S]*links\.html",
            ),
        ):
            make_zenodo_api_request(
                endpoint="/deposit/depositions/1", model=ZenodoDeposition
            )


def _mock_urlopen(body: object):
    response = MagicMock()
    response.read.return_value = json.dumps(body).encode()
    response.__enter__.return_value = response
    return patch("urllib.request.urlopen", return_value=response)
