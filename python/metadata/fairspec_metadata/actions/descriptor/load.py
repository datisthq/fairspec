from __future__ import annotations

import json
import urllib.request

from .parse import parse_descriptor
from fairspec_metadata.actions.path.general import (
    get_file_protocol,
    get_is_remote_path,
)
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.settings import USER_AGENT


def load_descriptor(
    path: str,
    *,
    only_remote: bool = False,
) -> Descriptor:
    is_remote = get_is_remote_path(path)

    if not is_remote and only_remote:
        raise Error("Cannot load descriptor for security reasons")

    if is_remote:
        return _load_remote_descriptor(path)
    return _load_local_descriptor(path)


class Error(Exception):
    pass


def _load_local_descriptor(path: str) -> Descriptor:
    with open(path, encoding="utf-8") as file:
        text = file.read()
    return parse_descriptor(text)


def _load_remote_descriptor(path: str) -> Descriptor:
    protocol = get_file_protocol(path)
    if protocol not in ("http", "https"):
        raise Error(f"Unsupported remote protocol: {protocol}")

    request = urllib.request.Request(path, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request) as response:  # noqa: S310
        descriptor: Descriptor = json.loads(response.read())

    return descriptor
