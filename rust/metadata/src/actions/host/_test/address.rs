use crate::actions::host::address::{create_host_message, get_is_public_address};

#[test]
fn test_public_addresses() {
    for address in [
        "8.8.8.8",
        "1.1.1.1",
        "2606:4700::1111",
        "::ffff:8.8.8.8",
        "64:ff9b::808:808",
    ] {
        assert!(get_is_public_address(address), "{address}");
    }
}

#[test]
fn test_non_public_addresses() {
    for address in [
        "0.0.0.0",
        "10.1.2.3",
        "100.64.0.1",
        "127.0.0.1",
        "169.254.169.254",
        "172.16.0.1",
        "192.0.0.1",
        "192.0.2.1",
        "192.88.99.1",
        "192.168.1.1",
        "198.18.0.1",
        "198.51.100.1",
        "203.0.113.1",
        "224.0.0.1",
        "255.255.255.255",
        "::",
        "::1",
        "::127.0.0.1",
        "100::1",
        "2001::1",
        "2001:db8::1",
        "2002::1",
        "fc00::1",
        "fd12:3456::1",
        "fe80::1",
        "fe80::1%eth0",
        "fec0::1",
        "ff02::1",
        "::ffff:127.0.0.1",
        "::ffff:169.254.169.254",
        "64:ff9b::a00:1",
        "64:ff9b:1::808:808",
        "not-an-address",
    ] {
        assert!(!get_is_public_address(address), "{address}");
    }
}

#[test]
fn test_host_message() {
    assert_eq!(
        create_host_message("localhost", "127.0.0.1"),
        "Host localhost resolves to non-public address 127.0.0.1 (set FAIRSPEC_HOST_POLICY=any to allow)"
    );
}
