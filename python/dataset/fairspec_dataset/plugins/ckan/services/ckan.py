from __future__ import annotations

import json
import urllib.parse
import urllib.request
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def make_ckan_api_request(
    *,
    ckan_url: str,
    action: str,
    payload: Descriptor,
    upload: tuple[str, bytes] | None = None,
    api_key: str | None = None,
) -> dict:
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

    with urllib.request.urlopen(request) as response:
        data = json.loads(response.read().decode())

    if not data.get("success"):
        raise Exception(f"CKAN API error: {data.get('error')}")

    return data["result"]
