import json
from unittest.mock import MagicMock, patch

import pytest

from fairspec_dataset.plugins.ckan.models.schema import CkanSchema

from ..ckan import make_ckan_api_request

CKAN_URL = "https://ckan.example.com/dataset/test"


class TestMakeCkanApiRequest:
    def test_returns_the_result_parsed_by_the_model(self):
        result = {"fields": [{"id": "name", "type": "text"}], "records": []}
        with _mock_urlopen({"success": True, "result": result}):
            schema = make_ckan_api_request(
                ckan_url=CKAN_URL,
                action="datastore_search",
                payload={"resource_id": "1"},
                model=CkanSchema,
            )

        assert len(schema.fields) == 1
        assert schema.fields[0].id == "name"
        assert schema.fields[0].type == "text"

    def test_rejects_a_result_that_does_not_match_the_model(self):
        with (
            _mock_urlopen({"success": True, "result": {"fields": [{"id": "name"}]}}),
            pytest.raises(
                Exception,
                match=r"CKAN API returned an unexpected response from "
                r"datastore_search[\s\S]*fields\.0\.type",
            ),
        ):
            make_ckan_api_request(
                ckan_url=CKAN_URL,
                action="datastore_search",
                payload={"resource_id": "1"},
                model=CkanSchema,
            )

    def test_reports_the_error_of_an_unsuccessful_response(self):
        error = {"message": "Not found", "__type": "Not Found Error"}
        with (
            _mock_urlopen({"success": False, "error": error}),
            pytest.raises(Exception, match="CKAN API error: Not found"),
        ):
            make_ckan_api_request(
                ckan_url=CKAN_URL,
                action="package_show",
                payload={"id": "test"},
                model=CkanSchema,
            )


def _mock_urlopen(body: object):
    response = MagicMock()
    response.read.return_value = json.dumps(body).encode()
    response.__enter__.return_value = response
    return patch("urllib.request.urlopen", return_value=response)
