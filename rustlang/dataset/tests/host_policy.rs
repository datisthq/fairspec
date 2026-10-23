use fairspec_dataset::load_file_stream;
use fairspec_metadata::{HostPolicy, set_host_policy};

// Its own test binary: the policy is process-wide, so it must not race the unit tests
#[test]
fn test_public_policy_blocks_remote_stream() {
    set_host_policy(Some(HostPolicy::Public));

    let error = load_file_stream("http://169.254.169.254/latest/meta-data/", None, None)
        .err()
        .unwrap();
    assert_eq!(
        error.to_string(),
        "Host 169.254.169.254 resolves to non-public address 169.254.169.254 (set FAIRSPEC_HOST_POLICY=any to allow)"
    );

    set_host_policy(None);
}
