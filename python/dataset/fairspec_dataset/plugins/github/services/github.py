from __future__ import annotations

import json
import urllib.request
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def make_github_api_request(
    *,
    endpoint: str,
    method: str = "GET",
    payload: Descriptor | None = None,
    api_key: str | None = None,
) -> dict:
    base_url = "https://api.github.com"
    url = f"{base_url}{endpoint}"

    headers: dict[str, str] = {}
    if api_key:
        headers["Authorization"] = f"Bearer {api_key}"

    data: bytes | None = None
    if payload:
        headers["Content-Type"] = "application/json"
        data = json.dumps(payload).encode()

    request = urllib.request.Request(url, data=data, headers=headers, method=method)

    with urllib.request.urlopen(request) as response:
        return json.loads(response.read().decode())
