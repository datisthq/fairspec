from __future__ import annotations

import json
import urllib.request
from typing import TYPE_CHECKING

from fairspec_metadata import fetch_remote
from fairspec_metadata.models.base import FairspecModel
from pydantic import ValidationError

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def make_github_api_request[M: FairspecModel](
    *,
    endpoint: str,
    method: str = "GET",
    payload: Descriptor | None = None,
    api_key: str | None = None,
    model: type[M],
) -> M:
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

    with fetch_remote(request) as response:
        data = json.loads(response.read().decode())

    try:
        return model.model_validate(data)
    except ValidationError as error:
        raise Exception(
            f"Github API returned an unexpected response from {endpoint}:\n{error}"
        ) from error
