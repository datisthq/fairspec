import pytest

from ..policy import Error, get_host_policy, set_host_policy


@pytest.fixture(autouse=True)
def reset_host_policy(monkeypatch):
    monkeypatch.delenv("FAIRSPEC_HOST_POLICY", raising=False)
    yield
    set_host_policy(None)


class TestGetHostPolicy:
    def test_defaults_to_any(self):
        assert get_host_policy() == "any"

    def test_reads_env(self, monkeypatch):
        monkeypatch.setenv("FAIRSPEC_HOST_POLICY", "public")
        assert get_host_policy() == "public"

    def test_empty_env_defaults_to_any(self, monkeypatch):
        monkeypatch.setenv("FAIRSPEC_HOST_POLICY", "")
        assert get_host_policy() == "any"

    def test_rejects_invalid_env(self, monkeypatch):
        monkeypatch.setenv("FAIRSPEC_HOST_POLICY", "private")
        with pytest.raises(Error, match="Invalid FAIRSPEC_HOST_POLICY: private"):
            get_host_policy()

    def test_set_policy_wins_over_env(self, monkeypatch):
        monkeypatch.setenv("FAIRSPEC_HOST_POLICY", "any")
        set_host_policy("public")
        assert get_host_policy() == "public"

    def test_set_none_falls_back_to_env(self, monkeypatch):
        set_host_policy("public")
        set_host_policy(None)
        assert get_host_policy() == "any"
