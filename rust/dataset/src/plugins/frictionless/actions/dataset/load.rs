use fairspec_metadata::{
    Descriptor, Result, assert_dataset, load_descriptor, normalize_path, resolve_basepath,
};
use serde_json::Value;

use super::from_frictionless::convert_dataset_from_frictionless;
use crate::plugins::descriptor::plugin::to_descriptor;
use crate::plugins::frictionless::helpers::inspect::assert_frictionless_model;
use crate::plugins::frictionless::models::dialect::FRICTIONLESS_DIALECT;
use crate::plugins::frictionless::models::package::FRICTIONLESS_PACKAGE;
use crate::plugins::frictionless::models::schema::FRICTIONLESS_SCHEMA;
use crate::plugins::frictionless::models::shape::Shape;

/// Load a Frictionless Data Package (v1 or v2) as a Dataset
/// A `datapackage.json` holding a Fairspec Dataset loads as it is
pub fn load_dataset_from_frictionless(path: &str) -> Result<Descriptor> {
    let basepath = resolve_basepath(path)?;
    let descriptor = load_descriptor(path, false)?;

    let schema = descriptor.get("$schema").and_then(Value::as_str);
    if schema.is_some_and(|schema| schema.starts_with("https://fairspec.org/")) {
        return to_descriptor(&assert_dataset(&descriptor, Some(&basepath))?);
    }

    let mut frictionless_package =
        assert_frictionless_model(&Value::Object(descriptor), &FRICTIONLESS_PACKAGE, None)?;

    if let Some(Value::Array(resources)) = frictionless_package.get_mut("resources") {
        for (index, resource) in resources.iter_mut().enumerate() {
            let Value::Object(resource) = resource else {
                continue;
            };
            let json_pointer = format!("/resources/{index}");
            for (key, shape) in [
                ("schema", &FRICTIONLESS_SCHEMA),
                ("dialect", &FRICTIONLESS_DIALECT),
            ] {
                if let Some(Value::String(reference)) = resource.get(key) {
                    let loaded = load_reference(
                        reference,
                        &basepath,
                        shape,
                        &format!("{json_pointer}/{key}"),
                    )?;
                    resource.insert(key.to_string(), Value::Object(loaded));
                }
            }
        }
    }

    let dataset = convert_dataset_from_frictionless(&frictionless_package);
    to_descriptor(&assert_dataset(&dataset, Some(&basepath))?)
}

fn load_reference(
    path: &str,
    basepath: &str,
    shape: &Shape,
    json_pointer: &str,
) -> Result<Descriptor> {
    let descriptor = load_descriptor(&normalize_path(path, Some(basepath))?, false)?;
    assert_frictionless_model(&Value::Object(descriptor), shape, Some(json_pointer))
}
