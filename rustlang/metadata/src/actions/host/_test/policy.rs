use crate::actions::host::policy::resolve_host_policy;
use crate::models::host::HostPolicy;

#[test]
fn test_defaults_to_any() {
    assert_eq!(resolve_host_policy(None, None).unwrap(), HostPolicy::Any);
    assert_eq!(
        resolve_host_policy(None, Some("")).unwrap(),
        HostPolicy::Any
    );
}

#[test]
fn test_reads_env() {
    assert_eq!(
        resolve_host_policy(None, Some("public")).unwrap(),
        HostPolicy::Public
    );
    assert_eq!(
        resolve_host_policy(None, Some("any")).unwrap(),
        HostPolicy::Any
    );
}

#[test]
fn test_rejects_invalid_env() {
    let error = resolve_host_policy(None, Some("private")).unwrap_err();
    assert_eq!(
        error.to_string(),
        r#"Invalid FAIRSPEC_HOST_POLICY: private (expected "any" or "public")"#
    );
}

#[test]
fn test_set_policy_wins_over_env() {
    assert_eq!(
        resolve_host_policy(Some(HostPolicy::Public), Some("any")).unwrap(),
        HostPolicy::Public
    );
}
