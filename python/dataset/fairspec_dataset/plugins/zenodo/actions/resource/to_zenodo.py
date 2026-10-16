from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def convert_resource_to_zenodo(resource: Descriptor) -> dict:
    zenodo_file: dict = {}

    if resource.get("name"):
        zenodo_file["key"] = resource["name"]

    integrity = resource.get("integrity", {})
    if isinstance(integrity, dict) and integrity.get("type") == "md5":
        zenodo_file["checksum"] = f"md5:{integrity['hash']}"

    return zenodo_file
