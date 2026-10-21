use serde_json::Value;

use super::load::load_descriptor_source;
use crate::actions::json::inspect::inspect_json;
use crate::actions::json_schema::load::METASCHEMA_ID;
use crate::actions::profile::load::load_profile;
use crate::actions::profile::registry::get_profile_type_name;
use crate::actions::report::create::create_report;
use crate::models::descriptor::Descriptor;
use crate::models::error::error::FairspecError;
use crate::models::error::metadata::{MetadataError, MetadataErrorType};
use crate::models::exception::Result;
use crate::models::path::PathOr;
use crate::models::profile::{Profile, ProfileType};
use crate::models::report::Report;

pub fn validate_descriptor(
    descriptor: &Descriptor,
    profile: &Profile,
    root_json_pointer: Option<&str>,
) -> Result<Report> {
    let issues = inspect_json(
        &Value::Object(descriptor.clone()),
        PathOr::Value(profile.clone()),
        root_json_pointer,
    )?;
    let errors = issues
        .into_iter()
        .map(|issue| {
            FairspecError::Metadata(MetadataError {
                r#type: MetadataErrorType::default(),
                resource_name: None,
                message: issue.message,
                json_pointer: issue.json_pointer,
            })
        })
        .collect();
    Ok(create_report(errors, None))
}

/// Loads a descriptor and validates it against its `$schema`, or the latest profile of the type.
pub(crate) fn validate_profiled_descriptor(
    source: impl Into<PathOr<Descriptor>>,
    profile_type: ProfileType,
    root_json_pointer: Option<&str>,
) -> Result<(Descriptor, Report)> {
    let descriptor = load_descriptor_source(source)?;

    let schema_url = match descriptor.get("$schema") {
        Some(Value::String(schema))
            if !(profile_type == ProfileType::TableSchema && schema == METASCHEMA_ID) =>
        {
            schema.clone()
        }
        _ => format!(
            "https://fairspec.org/profiles/latest/{}.json",
            get_profile_type_name(profile_type)
        ),
    };

    let profile = load_profile(&schema_url, profile_type)?;
    let report = validate_descriptor(&descriptor, &profile, root_json_pointer)?;
    Ok((descriptor, report))
}
