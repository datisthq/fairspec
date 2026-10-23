from __future__ import annotations

import http.client
import socket
import urllib.parse
import urllib.request

from fairspec_metadata.actions.host.address import assert_public_address
from fairspec_metadata.actions.host.policy import get_host_policy


def fetch_remote(request: str | urllib.request.Request):
    """Open a remote URL under the host policy; HTTP errors raise as with urlopen."""
    if get_host_policy() == "any":
        return urllib.request.urlopen(request)  # noqa: S310

    url = request if isinstance(request, str) else request.full_url
    protocol = urllib.parse.urlparse(url).scheme
    if protocol not in ("http", "https"):
        raise Error(f"Unsupported remote protocol: {protocol}")

    return _create_public_opener().open(request)


class Error(Exception):
    pass


# Only http(s) handlers and no ProxyHandler: every hop, redirects included, must connect
# straight to the address that was checked
def _create_public_opener() -> urllib.request.OpenerDirector:
    opener = urllib.request.OpenerDirector()
    for handler in (
        _PublicHTTPHandler(),
        _PublicHTTPSHandler(),
        urllib.request.HTTPRedirectHandler(),
        urllib.request.HTTPDefaultErrorHandler(),
        urllib.request.HTTPErrorProcessor(),
        urllib.request.UnknownHandler(),
    ):
        opener.add_handler(handler)
    return opener


def _create_public_connection(
    address: tuple[str, int],
    timeout: object = socket._GLOBAL_DEFAULT_TIMEOUT,  # type: ignore[attr-defined]
    source_address: tuple[str, int] | None = None,
) -> socket.socket:
    host, port = address
    infos = socket.getaddrinfo(host, port, type=socket.SOCK_STREAM)
    for *_, sockaddr in infos:
        assert_public_address(host, str(sockaddr[0]))

    last_error: OSError | None = None
    for family, kind, proto, _, sockaddr in infos:
        sock = socket.socket(family, kind, proto)
        try:
            if isinstance(timeout, (int, float)):
                sock.settimeout(timeout)
            if source_address:
                sock.bind(source_address)
            sock.connect(sockaddr)
            return sock
        except OSError as error:
            sock.close()
            last_error = error
    raise last_error or OSError(f"Cannot connect to {host}")


class _PublicHTTPConnection(http.client.HTTPConnection):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self._create_connection = _create_public_connection


class _PublicHTTPSConnection(http.client.HTTPSConnection):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self._create_connection = _create_public_connection


class _PublicHTTPHandler(urllib.request.HTTPHandler):
    def http_open(self, req):
        return self.do_open(_PublicHTTPConnection, req)


class _PublicHTTPSHandler(urllib.request.HTTPSHandler):
    def https_open(self, req):
        return self.do_open(_PublicHTTPSConnection, req, context=self._context)  # type: ignore[attr-defined]
