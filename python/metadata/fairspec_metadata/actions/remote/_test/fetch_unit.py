import threading
import urllib.error
from collections.abc import Iterator
from http.server import BaseHTTPRequestHandler, HTTPServer

import pytest

from fairspec_metadata.actions.host import address
from fairspec_metadata.actions.host.address import Error as HostError
from fairspec_metadata.actions.host.policy import set_host_policy

from ..fetch import Error, fetch_remote


@pytest.fixture(autouse=True)
def reset_host_policy(monkeypatch):
    monkeypatch.delenv("FAIRSPEC_HOST_POLICY", raising=False)
    yield
    set_host_policy(None)


@pytest.fixture
def base_url() -> Iterator[str]:
    server = HTTPServer(("127.0.0.1", 0), _Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    yield f"http://127.0.0.1:{server.server_port}"
    server.shutdown()
    server.server_close()


class TestFetchRemote:
    def test_any_reaches_loopback(self, base_url):
        with fetch_remote(f"{base_url}/data.json") as response:
            assert response.read() == b'{"name": "test"}'

    def test_public_blocks_loopback_literal(self, base_url):
        set_host_policy("public")
        with pytest.raises(HostError, match="non-public address 127.0.0.1"):
            fetch_remote(f"{base_url}/data.json")

    def test_public_blocks_resolved_name(self, base_url):
        set_host_policy("public")
        url = base_url.replace("127.0.0.1", "localhost")
        with pytest.raises(HostError, match="Host localhost resolves to non-public"):
            fetch_remote(f"{url}/data.json")

    def test_public_blocks_from_env(self, base_url, monkeypatch):
        monkeypatch.setenv("FAIRSPEC_HOST_POLICY", "public")
        with pytest.raises(HostError):
            fetch_remote(f"{base_url}/data.json")

    def test_public_rechecks_redirect(self, base_url, monkeypatch):
        set_host_policy("public")
        monkeypatch.setattr(
            address, "get_is_public_address", lambda value: value == "127.0.0.1"
        )
        with fetch_remote(f"{base_url}/data.json") as response:
            assert response.read() == b'{"name": "test"}'
        with pytest.raises(HostError, match="non-public address 169.254.169.254"):
            fetch_remote(f"{base_url}/redirect")

    def test_public_raises_http_error(self, base_url, monkeypatch):
        set_host_policy("public")
        monkeypatch.setattr(address, "get_is_public_address", lambda value: True)
        with pytest.raises(urllib.error.HTTPError):
            fetch_remote(f"{base_url}/missing")

    def test_public_rejects_other_protocols(self):
        set_host_policy("public")
        with pytest.raises(Error, match="Unsupported remote protocol: ftp"):
            fetch_remote("ftp://example.com/file.csv")


class _Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/data.json":
            body = b'{"name": "test"}'
            self.send_response(200)
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)
        elif self.path == "/redirect":
            self.send_response(302)
            self.send_header("Location", "http://169.254.169.254/latest/meta-data/")
            self.send_header("Content-Length", "0")
            self.end_headers()
        else:
            self.send_response(404)
            self.send_header("Content-Length", "0")
            self.end_headers()

    def log_message(self, format, *args):
        pass
