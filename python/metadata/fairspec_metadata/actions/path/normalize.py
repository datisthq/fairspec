from __future__ import annotations

import os
import posixpath
import urllib.parse

from .general import get_is_remote_path, safe_relpath


def normalize_path(path: str, *, basepath: str | None = None) -> str:
    is_path_remote = get_is_remote_path(path)
    is_basepath_remote = get_is_remote_path(basepath or "")

    if is_path_remote:
        parsed = urllib.parse.urlparse(path)
        return urllib.parse.urlunparse(
            (
                parsed.scheme,
                parsed.netloc,
                parsed.path,
                parsed.params,
                parsed.query,
                parsed.fragment,
            )
        )

    if is_basepath_remote:
        joined = f"{basepath}/{path}"
        parsed = urllib.parse.urlparse(joined)
        normalized_url_path = posixpath.normpath(parsed.path)
        normalized_url = urllib.parse.urlunparse(
            (
                parsed.scheme,
                parsed.netloc,
                normalized_url_path,
                parsed.params,
                parsed.query,
                parsed.fragment,
            )
        )
        if not normalized_url.startswith(basepath or ""):
            raise Error(path, basepath)
        return normalized_url

    normalized_path = os.path.join(basepath, path) if basepath else path
    relative_path = os.path.relpath(normalized_path, basepath or "")
    if relative_path.startswith(".."):
        raise Error(path, basepath)

    return safe_relpath(os.path.abspath(normalized_path))


class Error(Exception):
    def __init__(self, path: str, basepath: str | None):
        super().__init__(f"Path {path} is not a subpath of {basepath}")
