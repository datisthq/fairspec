from unittest.mock import patch

import pytest
from fairspec_library import get_host_policy, set_host_policy

from ..server import start_server


@pytest.fixture(autouse=True)
def reset_host_policy(monkeypatch):
    monkeypatch.delenv("FAIRSPEC_HOST_POLICY", raising=False)
    yield
    set_host_policy(None)


class TestStartServer:
    def test_should_block_non_public_hosts_by_default(self):
        with patch("fairspec_mcp_server.server.server.run") as run:
            start_server()

        run.assert_called_once_with("stdio")
        assert get_host_policy() == "public"

    def test_should_keep_host_policy_from_env(self, monkeypatch):
        monkeypatch.setenv("FAIRSPEC_HOST_POLICY", "any")
        with patch("fairspec_mcp_server.server.server.run"):
            start_server()

        assert get_host_policy() == "any"
