from __future__ import annotations

import os
import re
import urllib.parse


def safe_relpath(path: str, start: str | None = None) -> str:
    try:
        return os.path.relpath(path) if start is None else os.path.relpath(path, start)
    except ValueError:
        return os.path.abspath(path)


def get_file_protocol(path: str) -> str:
    try:
        parsed = urllib.parse.urlparse(path)
        protocol = parsed.scheme
        if len(protocol) < 2:
            return "file"
        return protocol
    except Exception:
        return "file"


def get_is_remote_path(path: str) -> bool:
    return get_file_protocol(path) != "file"


def get_file_name(path: str) -> str | None:
    if get_is_remote_path(path):
        pathname = urllib.parse.urlparse(path).path
        file_name = pathname.split("/")[-1]
        return file_name if file_name and "." in file_name else None

    resolved = os.path.abspath(path)
    file_name = os.path.basename(resolved)
    return file_name if file_name and "." in file_name else None


def get_file_extension(path: str) -> str | None:
    file_name = get_file_name(path)
    if not file_name:
        return None
    extension = file_name.split(".")[-1]
    if file_name == f".{extension}":
        return None
    return extension


def get_file_basename(path: str) -> str | None:
    file_name = get_file_name(path)
    extension = get_file_extension(path)
    if extension and file_name:
        return file_name.replace(f".{extension}", "", 1)
    return file_name


def get_file_name_slug(path: str) -> str | None:
    basename = get_file_basename(path)
    if not basename:
        return None
    return _slugify(basename)


def _slugify(text: str) -> str:
    text = re.sub(r"([a-z])([A-Z])", r"\1_\2", text)
    text = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", text)
    slug = re.sub(r"[^a-z0-9]+", "_", text.lower()).strip("_")
    slug = re.sub(r"_+", "_", slug)
    return re.sub(r"[^a-zA-Z0-9_]", "", slug)
