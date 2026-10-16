import json
import os
from unittest.mock import patch

import pytest

from fairspec_metadata.settings import USER_AGENT

from ..load import Error, load_descriptor


FIXTURES_DIR = os.path.join(os.path.dirname(__file__), "fixtures")


class TestLoadDescriptor:
    def test_load_local_descriptor(self):
        path = os.path.join(FIXTURES_DIR, "schema.json")
        descriptor = load_descriptor(path)
        assert descriptor["fields"][0]["name"] == "id"
        assert descriptor["fields"][1]["name"] == "name"

    def test_load_remote_descriptor(self):
        expected = {"name": "test"}
        response_bytes = json.dumps(expected).encode()

        with patch(
            "fairspec_metadata.actions.descriptor.load.urllib.request.urlopen"
        ) as mock:
            mock.return_value.__enter__ = lambda s: s
            mock.return_value.__exit__ = lambda s, *a: None
            mock.return_value.read.return_value = response_bytes
            descriptor = load_descriptor("https://example.com/test.json")

        assert descriptor == expected

    def test_load_remote_descriptor_bad_protocol(self):
        with pytest.raises(Error, match="Unsupported remote protocol: ftp"):
            load_descriptor("ftp://example.com/file.json")

    def test_only_remote_rejects_local(self):
        with pytest.raises(Error, match="security"):
            load_descriptor("local.json", only_remote=True)

    def test_load_remote_descriptor_sends_user_agent(self):
        with patch(
            "fairspec_metadata.actions.descriptor.load.urllib.request.urlopen"
        ) as mock:
            mock.return_value.__enter__ = lambda s: s
            mock.return_value.__exit__ = lambda s, *a: None
            mock.return_value.read.return_value = b"{}"
            load_descriptor("https://example.com/test.json")

        request = mock.call_args.args[0]
        assert request.get_header("User-agent") == USER_AGENT
