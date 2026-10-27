from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def convert_rights_to_frictionless(rights_list: list[Descriptor]) -> list[Descriptor]:
    licenses: list[Descriptor] = []
    for rights in rights_list:
        license: Descriptor = {}
        if rights.get("rightsIdentifier"):
            license["name"] = rights["rightsIdentifier"]
        if rights.get("rightsUri"):
            license["path"] = rights["rightsUri"]
        if rights.get("rights"):
            license["title"] = rights["rights"]
        if license.get("name") or license.get("path"):
            licenses.append(license)
    return licenses
