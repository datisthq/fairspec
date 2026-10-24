use crate::actions::profile::load::load_profile;
use crate::actions::profile::registry::PROFILE_REGISTRY;
use crate::models::exception::Error;
use crate::models::profile::ProfileType;
use crate::settings::FAIRSPEC_VERSION;

#[test]
fn test_resolves_bundled_version_offline() {
    for version in ["latest", FAIRSPEC_VERSION] {
        let path = format!("https://fairspec.org/profiles/{version}/dataset.json");
        let bundled = &PROFILE_REGISTRY
            .iter()
            .find(|item| item.path == path)
            .unwrap()
            .profile;

        let profile = load_profile(&path, ProfileType::Dataset).unwrap();

        assert_eq!(&profile, bundled);
    }
}

#[test]
fn test_goes_remote_for_unbundled_version() {
    let result = load_profile(
        "https://fairspec.org/profiles/0.4.0/dataset.json",
        ProfileType::Dataset,
    );
    assert!(matches!(result, Err(Error::Http(message)) if message.contains("disabled")));
}
