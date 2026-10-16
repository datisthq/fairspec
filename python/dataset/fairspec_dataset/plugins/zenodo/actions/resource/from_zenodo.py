from __future__ import annotations

import re
from typing import TYPE_CHECKING

from fairspec_metadata import get_file_name_slug
from fairspec_metadata import Integrity, IntegrityType
from fairspec_metadata import Resource

if TYPE_CHECKING:
    from fairspec_dataset.plugins.zenodo.models.file import ZenodoFile


def convert_resource_from_zenodo(zenodo_file: ZenodoFile) -> Resource:
    links = zenodo_file.links
    path = _convert_path(links.self or "" if links else "")

    return Resource(
        data=path,
        name=get_file_name_slug(zenodo_file.key or "") or zenodo_file.id or "",
        integrity=Integrity(
            type=IntegrityType.md5,
            hash=(zenodo_file.checksum or "").replace("md5:", ""),
        ),
        unstable_customMetadata={
            "zenodoKey": zenodo_file.key,
            "zenodoUrl": path,
        },
    )


def _convert_path(link: str) -> str:
    result = link.replace("/api/", "/")
    result = re.sub(r"/content$", "", result)
    return result
