from __future__ import annotations

import json
import urllib.parse
import urllib.request


def make_zenodo_api_request(
    *,
    endpoint: str,
    method: str = "GET",
    payload: dict | None = None,
    upload: tuple[str, bytes] | None = None,
    api_key: str | None = None,
    sandbox: bool = False,
) -> dict:
    base_url = "https://sandbox.zenodo.org/api" if sandbox else "https://zenodo.org/api"
    url = f"{base_url}{endpoint}"

    if api_key:
        separator = "&" if "?" in url else "?"
        url = f"{url}{separator}access_token={api_key}"

    headers: dict[str, str] = {}
    data: bytes | None = None

    if upload:
        file_name, file_data = upload
        boundary = "----FormBoundary7MA4YWxkTrZu0gW"
        headers["Content-Type"] = f"multipart/form-data; boundary={boundary}"

        body = b""
        body += f"--{boundary}\r\n".encode()
        body += f'Content-Disposition: form-data; name="file"; filename="{file_name}"\r\n'.encode()
        body += b"Content-Type: application/octet-stream\r\n\r\n"
        body += file_data
        body += f"\r\n--{boundary}--\r\n".encode()
        data = body
    elif payload is not None:
        headers["Content-Type"] = "application/json"
        data = json.dumps(payload).encode()

    request = urllib.request.Request(url, data=data, headers=headers, method=method)

    with urllib.request.urlopen(request) as response:
        response_data = response.read().decode()
        if not response_data:
            return {}
        return json.loads(response_data)
