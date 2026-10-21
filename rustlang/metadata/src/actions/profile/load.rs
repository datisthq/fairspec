use super::assert::assert_profile;
use crate::actions::json_schema::load::load_json_schema;
use crate::models::exception::Result;
use crate::models::profile::{Profile, ProfileType};

pub fn load_profile(path: &str, profile_type: ProfileType) -> Result<Profile> {
    let json_schema = load_json_schema(path, false)?;
    assert_profile(json_schema, path, profile_type)
}
