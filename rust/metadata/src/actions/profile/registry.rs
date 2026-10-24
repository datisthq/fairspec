use std::sync::LazyLock;

use crate::models::profile::{Profile, ProfileRegistry, ProfileRegistryItem, ProfileType};
use crate::settings::FAIRSPEC_VERSION;

/// The bundle is a snapshot of `FAIRSPEC_VERSION`, so it answers to that version's URL as well
/// as "latest" -- every save action stamps the versioned one, and a miss goes remote.
pub static PROFILE_REGISTRY: LazyLock<ProfileRegistry> = LazyLock::new(|| {
    let bundled = [
        (
            ProfileType::Catalog,
            include_str!("../../../profiles/catalog.json"),
        ),
        (
            ProfileType::Dataset,
            include_str!("../../../profiles/dataset.json"),
        ),
        (
            ProfileType::FileDialect,
            include_str!("../../../profiles/file-dialect.json"),
        ),
        (
            ProfileType::DataSchema,
            include_str!("../../../profiles/data-schema.json"),
        ),
        (
            ProfileType::TableSchema,
            include_str!("../../../profiles/table-schema.json"),
        ),
    ];
    bundled
        .into_iter()
        .flat_map(|(profile_type, text)| {
            let profile: Profile = serde_json::from_str(text).expect("bundled profile is JSON");
            ["latest", FAIRSPEC_VERSION].map(|version| ProfileRegistryItem {
                r#type: profile_type,
                path: format!(
                    "https://fairspec.org/profiles/{version}/{}.json",
                    get_profile_type_name(profile_type)
                ),
                version: version.to_string(),
                profile: profile.clone(),
            })
        })
        .collect()
});

pub(crate) fn get_profile_type_name(profile_type: ProfileType) -> &'static str {
    match profile_type {
        ProfileType::Catalog => "catalog",
        ProfileType::Dataset => "dataset",
        ProfileType::FileDialect => "file-dialect",
        ProfileType::DataSchema => "data-schema",
        ProfileType::TableSchema => "table-schema",
    }
}
