use fairspec_metadata::{
    Descriptor, Error, FrictionlessKind, Result, assert_dataset,
    convert_descriptor_from_frictionless, create_report, load_descriptor, map_frictionless_errors,
    resolve_basepath,
};

use crate::plugins::descriptor::plugin::to_descriptor;

/// Load a descriptor as a Frictionless Data Package (v1 or v2), whatever its content
/// looks like; `load_dataset` recognises one by content without this
pub fn load_dataset_from_frictionless(path: &str) -> Result<Descriptor> {
    let basepath = resolve_basepath(path)?;
    let descriptor = load_descriptor(path, false)?;

    let (converted, mapping, errors) =
        convert_descriptor_from_frictionless(&descriptor, FrictionlessKind::Dataset, None);
    let Some(converted) = converted else {
        return Err(Error::Invalid {
            message: "Invalid Frictionless Data Package".to_string(),
            report: Some(create_report(errors, None)),
        });
    };

    match assert_dataset(&converted, Some(&basepath)) {
        Ok(dataset) => to_descriptor(&dataset),
        Err(Error::Invalid {
            message,
            report: Some(report),
        }) => Err(Error::Invalid {
            message,
            report: Some(create_report(
                map_frictionless_errors(report.errors, &mapping),
                None,
            )),
        }),
        Err(error) => Err(error),
    }
}
