from unittest.mock import patch

import pytest

from fairspec_metadata.actions.json_schema.load import load_json_schema
from fairspec_metadata.models.profile import ProfileType
from fairspec_metadata.settings import FAIRSPEC_VERSION

from ..load import load_profile
from ..registry import profile_registry


@pytest.fixture(autouse=True)
def clear_schema_cache():
    load_json_schema.cache_clear()
    yield
    load_json_schema.cache_clear()


# Patching urlopen is the assertion: a registry miss is only visible as a network call.
def offline():
    return patch(
        "fairspec_metadata.actions.descriptor.load.urllib.request.urlopen",
        side_effect=OSError("offline"),
    )


class TestLoadProfileFromRegistry:
    @pytest.mark.parametrize("version", ["latest", FAIRSPEC_VERSION])
    def test_resolves_bundled_version_offline(self, version):
        bundled = next(
            item.profile
            for item in profile_registry
            if item.path == f"https://fairspec.org/profiles/{version}/dataset.json"
        )

        with offline() as mock:
            profile = load_profile(
                f"https://fairspec.org/profiles/{version}/dataset.json",
                profile_type=ProfileType.dataset,
            )

        assert profile is bundled
        mock.assert_not_called()

    def test_goes_remote_for_unbundled_version(self):
        with offline() as mock, pytest.raises(OSError, match="offline"):
            load_profile(
                "https://fairspec.org/profiles/0.4.0/dataset.json",
                profile_type=ProfileType.dataset,
            )

        mock.assert_called_once()
