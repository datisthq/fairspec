use fairspec_metadata::{HostPolicy, get_host_policy, load_descriptor, set_host_policy};

// Its own test binary: the policy is process-wide, so it must not race the unit tests
#[test]
fn test_public_policy_blocks_descriptor_loading() {
    set_host_policy(Some(HostPolicy::Public));
    assert_eq!(get_host_policy().unwrap(), HostPolicy::Public);

    let error = load_descriptor("http://169.254.169.254/latest/meta-data/", false).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Host 169.254.169.254 resolves to non-public address 169.254.169.254 (set FAIRSPEC_HOST_POLICY=any to allow)"
    );

    set_host_policy(None);
}
