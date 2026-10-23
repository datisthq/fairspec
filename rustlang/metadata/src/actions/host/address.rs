use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

const NON_PUBLIC_IPV4_NETWORKS: &[(Ipv4Addr, u32)] = &[
    (Ipv4Addr::new(0, 0, 0, 0), 8),
    (Ipv4Addr::new(10, 0, 0, 0), 8),
    (Ipv4Addr::new(100, 64, 0, 0), 10),
    (Ipv4Addr::new(127, 0, 0, 0), 8),
    (Ipv4Addr::new(169, 254, 0, 0), 16),
    (Ipv4Addr::new(172, 16, 0, 0), 12),
    (Ipv4Addr::new(192, 0, 0, 0), 24),
    (Ipv4Addr::new(192, 0, 2, 0), 24),
    (Ipv4Addr::new(192, 88, 99, 0), 24),
    (Ipv4Addr::new(192, 168, 0, 0), 16),
    (Ipv4Addr::new(198, 18, 0, 0), 15),
    (Ipv4Addr::new(198, 51, 100, 0), 24),
    (Ipv4Addr::new(203, 0, 113, 0), 24),
    (Ipv4Addr::new(224, 0, 0, 0), 4),
    (Ipv4Addr::new(240, 0, 0, 0), 4),
];

const NON_PUBLIC_IPV6_NETWORKS: &[(Ipv6Addr, u32)] = &[
    (Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0), 96),
    (Ipv6Addr::new(0x64, 0xff9b, 1, 0, 0, 0, 0, 0), 48),
    (Ipv6Addr::new(0x100, 0, 0, 0, 0, 0, 0, 0), 64),
    (Ipv6Addr::new(0x2001, 0, 0, 0, 0, 0, 0, 0), 23),
    (Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0), 32),
    (Ipv6Addr::new(0x2002, 0, 0, 0, 0, 0, 0, 0), 16),
    (Ipv6Addr::new(0xfc00, 0, 0, 0, 0, 0, 0, 0), 7),
    (Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0), 10),
    (Ipv6Addr::new(0xfec0, 0, 0, 0, 0, 0, 0, 0), 10),
    (Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0), 8),
];

const IPV4_MAPPED_NETWORK: (Ipv6Addr, u32) = (Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0, 0), 96);
const NAT64_NETWORK: (Ipv6Addr, u32) = (Ipv6Addr::new(0x64, 0xff9b, 0, 0, 0, 0, 0, 0), 96);

/// Whether an IP address is publicly routable; anything unparsable is not.
pub fn get_is_public_address(address: &str) -> bool {
    let address = address.split('%').next().unwrap_or_default();
    match address.parse::<IpAddr>() {
        Ok(IpAddr::V4(ip)) => get_is_public_ipv4(ip),
        Ok(IpAddr::V6(ip)) => get_is_public_ipv6(ip),
        Err(_) => false,
    }
}

pub(crate) fn create_host_message(host: &str, address: &str) -> String {
    format!(
        "Host {host} resolves to non-public address {address} (set FAIRSPEC_HOST_POLICY=any to allow)"
    )
}

fn get_is_public_ipv4(ip: Ipv4Addr) -> bool {
    let value = u32::from(ip);
    !NON_PUBLIC_IPV4_NETWORKS.iter().any(|(network, prefix)| {
        let shift = 32 - prefix;
        value >> shift == u32::from(*network) >> shift
    })
}

fn get_is_public_ipv6(ip: Ipv6Addr) -> bool {
    let value = u128::from(ip);
    let is_in = |(network, prefix): (Ipv6Addr, u32)| {
        let shift = 128 - prefix;
        value >> shift == u128::from(network) >> shift
    };
    if is_in(IPV4_MAPPED_NETWORK) || is_in(NAT64_NETWORK) {
        return get_is_public_ipv4(Ipv4Addr::from(value as u32));
    }
    !NON_PUBLIC_IPV6_NETWORKS.iter().copied().any(is_in)
}
