use std::fs::File;
use std::io::{Read, Write};

use fairspec_metadata::{Dataset, Error, Result, denormalize_dataset, stringify_descriptor};
use serde_json::Value;
use zip::write::SimpleFileOptions;

use crate::actions::dataset::basepath::get_dataset_basepath;
use crate::actions::file::path::assert_local_path_vacant;
use crate::actions::resource::save::{SaveFileProps, save_resource_files};
use crate::actions::stream::load::load_file_stream;
use crate::plugins::descriptor::plugin::to_descriptor;

/// Writes a dataset's files and a `dataset.json` referencing them into a new zip archive.
pub fn save_dataset_to_zip(dataset: &Dataset, archive_path: &str, with_remote: bool) -> Result<()> {
    let basepath = get_dataset_basepath(dataset)?;

    assert_local_path_vacant(archive_path)?;
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();

    let mut save_file = |props: &SaveFileProps| -> Result<String> {
        let mut data = Vec::new();
        load_file_stream(props.normalized_path.as_str(), None, None)?.read_to_end(&mut data)?;
        set_file(&mut files, &props.denormalized_path, data);
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
    set_file(
        &mut files,
        "dataset.json",
        stringify_descriptor(&descriptor).into_bytes(),
    );

    let mut writer = zip::ZipWriter::new(File::create(archive_path)?);
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name, data) in files {
        writer
            .start_file(name, options)
            .map_err(|error| Error::Load(format!("{archive_path}: {error}")))?;
        writer.write_all(&data)?;
    }
    writer
        .finish()
        .map_err(|error| Error::Load(format!("{archive_path}: {error}")))?;
    Ok(())
}

// A dict assignment in Python: a repeated name keeps its place, with the new content
fn set_file(files: &mut Vec<(String, Vec<u8>)>, name: &str, data: Vec<u8>) {
    match files.iter_mut().find(|(existing, _)| existing == name) {
        Some(entry) => entry.1 = data,
        None => files.push((name.to_string(), data)),
    }
}
