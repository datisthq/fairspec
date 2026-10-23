from __future__ import annotations

import ipaddress

NON_PUBLIC_NETWORKS = [
    ipaddress.ip_network(network)
    for network in (
        "0.0.0.0/8",
        "10.0.0.0/8",
        "100.64.0.0/10",
        "127.0.0.0/8",
        "169.254.0.0/16",
        "172.16.0.0/12",
        "192.0.0.0/24",
        "192.0.2.0/24",
        "192.88.99.0/24",
        "192.168.0.0/16",
        "198.18.0.0/15",
        "198.51.100.0/24",
        "203.0.113.0/24",
        "224.0.0.0/4",
        "240.0.0.0/4",
        "::/96",
        "64:ff9b:1::/48",
        "100::/64",
        "2001::/23",
        "2001:db8::/32",
        "2002::/16",
        "fc00::/7",
        "fe80::/10",
        "fec0::/10",
        "ff00::/8",
    )
]

NAT64_NETWORK = ipaddress.ip_network("64:ff9b::/96")


def get_is_public_address(address: str) -> bool:
    """Whether an IP address is publicly routable; anything unparsable is not."""
    try:
        ip = ipaddress.ip_address(address)
    except ValueError:
        return False

    if isinstance(ip, ipaddress.IPv6Address):
        if ip.ipv4_mapped:
            return get_is_public_address(str(ip.ipv4_mapped))
        if ip in NAT64_NETWORK:
            return get_is_public_address(str(ipaddress.IPv4Address(int(ip) & 0xFFFFFFFF)))

    return not any(ip in network for network in NON_PUBLIC_NETWORKS)


def assert_public_address(host: str, address: str) -> None:
    """Raise when a host resolves to an address the public host policy refuses."""
    if not get_is_public_address(address):
        raise Error(
            f"Host {host} resolves to non-public address {address} "
            "(set FAIRSPEC_HOST_POLICY=any to allow)"
        )


class Error(Exception):
    pass
