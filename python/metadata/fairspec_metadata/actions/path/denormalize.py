from __future__ import annotations

import os
import urllib.parse

from .general import get_is_remote_path


def denormalize_path(path: str, *, basepath: str | None = None) -> str:
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
        normalized_basepath = urllib.parse.urlparse(basepath or "").geturl()
        if not path.startswith(normalized_basepath):
            raise Error(path, basepath)
        return path.removeprefix(f"{normalized_basepath}/")

    normalized_path = os.path.abspath(path)
    normalized_basepath = os.path.abspath(basepath or "")
    if not normalized_path.startswith(normalized_basepath):
        raise Error(path, basepath)

    relative = os.path.relpath(normalized_path, normalized_basepath)
    return relative.replace(os.sep, "/")


class Error(Exception):
    def __init__(self, path: str, basepath: str | None):
        super().__init__(f"Path {path} is not a subpath of {basepath}")
