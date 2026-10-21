use std::path::Path;

use fairspec_metadata::{Dataset, Descriptor, Result, denormalize_dataset, save_descriptor};
use serde_json::Value;

use crate::actions::dataset::basepath::get_dataset_basepath;
use crate::actions::file::copy::copy_file;
use crate::actions::file::path::assert_local_path_vacant;
use crate::actions::file::temp::format_path;
use crate::actions::folder::create::create_folder;
use crate::actions::resource::save::{SaveFileProps, save_resource_files};
use crate::plugins::descriptor::plugin::to_descriptor;

/// Copies a dataset's files into a new folder next to a `dataset.json` referencing them.
pub fn save_dataset_to_folder(
    dataset: &Dataset,
    folder_path: &str,
    with_remote: bool,
) -> Result<Descriptor> {
    let basepath = get_dataset_basepath(dataset)?;

    assert_local_path_vacant(folder_path)?;
    create_folder(folder_path)?;

    let mut save_file = |props: &SaveFileProps| -> Result<String> {
        let target_path = format_path(&Path::new(folder_path).join(&props.denormalized_path));
        copy_file(&props.normalized_path, &target_path, None)?;
        Ok(props.denormalized_path.clone())
    };
    let mut resource_descriptors = Vec::new();
    for resource in dataset.resources.iter().flatten() {
        let descriptor = save_resource_files(
            resource,
            &mut save_file,
            basepath.as_deref(),
            with_remote,
            false,
        )?;
        resource_descriptors.push(Value::Object(descriptor));
    }

    let denormalized = denormalize_dataset(dataset, basepath.as_deref())?;
    let mut descriptor = to_descriptor(&denormalized)?;
    descriptor.insert("resources".to_string(), Value::Array(resource_descriptors));

    let path = format_path(&Path::new(folder_path).join("dataset.json"));
    save_descriptor(&descriptor, &path, false)?;
    Ok(descriptor)
}
