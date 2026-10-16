from __future__ import annotations

import json
from pathlib import Path

from fairspec_metadata.models.profile import (
    Profile,
    ProfileRegistry,
    ProfileRegistryItem,
    ProfileType,
)
from fairspec_metadata.settings import FAIRSPEC_VERSION

_PROFILES_DIR = Path(__file__).parent.parent.parent / "profiles"


def _load_profile(profile_type: ProfileType) -> Profile:
    with open(_PROFILES_DIR / f"{profile_type.value}.json", encoding="utf-8") as file:
        return json.load(file)


_bundled_profiles: dict[ProfileType, Profile] = {
    profile_type: _load_profile(profile_type) for profile_type in ProfileType
}

# The bundle is a snapshot of FAIRSPEC_VERSION, so it must answer to that version's URL as
# well as "latest" -- every save_* action stamps the versioned one, and a miss goes remote.
profile_registry: ProfileRegistry = [
    ProfileRegistryItem(
        type=profile_type,
        path=f"https://fairspec.org/profiles/{version}/{profile_type.value}.json",
        version=version,
        profile=profile,
    )
    for profile_type, profile in _bundled_profiles.items()
    for version in ("latest", FAIRSPEC_VERSION)
]
