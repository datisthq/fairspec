from __future__ import annotations

import sqlite3

from fairspec_dataset import get_is_local_path_exist


def connect_database(path: str, *, create: bool = False) -> sqlite3.Connection:
    path = path.removeprefix("sqlite://")

    if path == ":memory:" or path.startswith("file::memory"):
        raise Exception("In-memory databases are not supported")

    if not create:
        if not get_is_local_path_exist(path):
            raise Exception(f'Database file "{path}" does not exist')

    conn = sqlite3.connect(path)
    conn.row_factory = sqlite3.Row
    return conn
