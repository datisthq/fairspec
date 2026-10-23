from __future__ import annotations

import json
import urllib.parse
import urllib.request
from typing import TYPE_CHECKING

from fairspec_metadata import fetch_remote
from fairspec_metadata.models.base import FairspecModel
from pydantic import ValidationError

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def make_ckan_api_request[M: FairspecModel](
    *,
    ckan_url: str,
    action: str,
    payload: Descriptor,
    upload: tuple[str, bytes] | None = None,
    api_key: str | None = None,
    model: type[M],
) -> M:
    parsed = urllib.parse.urlparse(ckan_url)
    url = f"{parsed.scheme}://{parsed.netloc}/api/3/action/{action}"

    headers: dict[str, str] = {}
    if api_key:
        headers["Authorization"] = api_key

    if upload:
        file_name, file_data = upload
        boundary = "----FormBoundary7MA4YWxkTrZu0gW"
        headers["Content-Type"] = f"multipart/form-data; boundary={boundary}"

        body = b""
        for key, value in payload.items():
            body += f"--{boundary}\r\n".encode()
            body += f'Content-Disposition: form-data; name="{key}"\r\n\r\n'.encode()
            body += f"{value}\r\n".encode()

        body += f"--{boundary}\r\n".encode()
        body += f'Content-Disposition: form-data; name="upload"; filename="{file_name}"\r\n'.encode()
        body += b"Content-Type: application/octet-stream\r\n\r\n"
        body += file_data
        body += f"\r\n--{boundary}--\r\n".encode()
    else:
        headers["Content-Type"] = "application/json"
        body = json.dumps(payload).encode()

    request = urllib.request.Request(url, data=body, headers=headers, method="POST")

    with fetch_remote(request) as response:
        data = json.loads(response.read().decode())

    try:
        envelope = _CkanResponse.model_validate(data)
    except ValidationError as error:
        raise _create_response_error(action, error) from error

    if not envelope.success:
        message = envelope.error.get("message") if envelope.error else None
        raise Exception(f"CKAN API error: {message or json.dumps(envelope.error)}")

    try:
        return model.model_validate(envelope.result)
    except ValidationError as error:
        raise _create_response_error(action, error) from error


class _CkanResponse(FairspecModel):
    success: bool
    result: object = None
    error: dict[str, object] | None = None


def _create_response_error(action: str, error: ValidationError) -> Exception:
    return Exception(f"CKAN API returned an unexpected response from {action}:\n{error}")
