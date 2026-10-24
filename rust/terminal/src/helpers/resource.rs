use fairspec_library::fairspec_metadata::{Dataset, Error, Resource, infer_resource_name};
use fairspec_library::load_dataset;

use crate::models::output::Exit;
use crate::session::Session;

pub fn select_resource(
    session: &Session,
    dataset: Option<&str>,
    resource: Option<&str>,
) -> Result<Resource, Exit> {
    let loaded = session.task("Loading dataset", || load_selected_dataset(dataset))?;
    session.task("Selecting resource", || find_resource(&loaded, resource))
}

fn load_selected_dataset(dataset_path: Option<&str>) -> Result<Dataset, Error> {
    let Some(dataset_path) = dataset_path.filter(|path| !path.is_empty()) else {
        return Err(create_value_error(
            "Please provide a path argument or a dataset option",
        ));
    };
    let Some(descriptor) = load_dataset(dataset_path)? else {
        return Err(create_value_error("Could not load dataset"));
    };
    serde_json::from_value(serde_json::Value::Object(descriptor))
        .map_err(|error| Error::Parse(error.to_string()))
}

fn find_resource(dataset: &Dataset, resource_name: Option<&str>) -> Result<Resource, Error> {
    let Some(resource_name) = resource_name.filter(|name| !name.is_empty()) else {
        return Err(create_value_error("Please provide a resource option"));
    };
    for resource in dataset.resources.iter().flatten() {
        let name = resource
            .name
            .clone()
            .unwrap_or_else(|| infer_resource_name(resource, None));
        if name == resource_name {
            return Ok(resource.clone());
        }
    }
    Err(create_value_error(&format!(
        "Resource \"{resource_name}\" not found"
    )))
}

/// Python's `ValueError`: its message is the whole `str()`.
pub fn create_value_error(message: &str) -> Error {
    Error::Load(message.to_string())
}
