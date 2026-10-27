use fairspec_metadata::{
    Dataset, PathOr, Result, convert_dataset_to_frictionless, denormalize_resource, get_basepath,
    resolve_file_dialect, resolve_table_schema, save_descriptor,
};
use serde_json::Value;

use crate::models::dataset::SaveDatasetResult;
use crate::plugins::descriptor::plugin::to_descriptor;

/// Save a Dataset as a Frictionless Data Package (v2) descriptor
/// Paths are written relative to the descriptor; data files are not copied
pub fn save_dataset_to_frictionless(
    dataset: &Dataset,
    target: &str,
    overwrite: bool,
) -> Result<SaveDatasetResult> {
    let basepath = get_basepath(target);

    let mut resources = Vec::new();
    for resource in dataset.resources.iter().flatten() {
        let mut resource = resource.clone();
        resource.table_schema =
            resolve_table_schema(resource.table_schema.as_ref())?.map(PathOr::Value);
        resource.file_dialect =
            resolve_file_dialect(resource.file_dialect.as_ref())?.map(PathOr::Value);
        resources.push(Value::Object(denormalize_resource(
            &resource,
            Some(&basepath),
        )?));
    }

    let mut descriptor = to_descriptor(dataset)?;
    descriptor.insert("resources".to_string(), Value::Array(resources));

    let frictionless_package = convert_dataset_to_frictionless(&descriptor)?;
    save_descriptor(&frictionless_package, target, overwrite)?;

    Ok(SaveDatasetResult {
        path: Some(target.to_string()),
    })
}
