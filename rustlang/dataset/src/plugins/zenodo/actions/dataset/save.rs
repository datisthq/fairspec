use std::io::Read;

use fairspec_metadata::{
    Dataset, Descriptor, Error, ResourceDataPath, Result, denormalize_dataset, stringify_descriptor,
};
use serde_json::Value;

use super::to_zenodo::convert_dataset_to_zenodo;
use crate::actions::dataset::basepath::get_dataset_basepath;
use crate::actions::resource::save::{SaveFileProps, save_resource_files};
use crate::actions::stream::load::load_file_stream;
use crate::plugins::zenodo::models::deposition::ZenodoDeposition;
use crate::plugins::zenodo::models::deposition_file::ZenodoDepositionFile;
use crate::plugins::zenodo::services::zenodo::make_zenodo_api_request;

pub fn save_dataset_to_zenodo(
    dataset: &Dataset,
    api_key: &str,
    sandbox: bool,
) -> Result<Descriptor> {
    let basepath = get_dataset_basepath(dataset)?;

    let new_zenodo_record = convert_dataset_to_zenodo(dataset);
    let zenodo_record: ZenodoDeposition = make_zenodo_api_request(
        "/deposit/depositions",
        Some("POST"),
        Some(&new_zenodo_record),
        None,
        Some(api_key),
        sandbox,
    )?;

    let record_id = zenodo_record.id;
    let files_endpoint = format!("/deposit/depositions/{record_id}/files");

    let mut resource_descriptors = Vec::new();
    for resource in dataset.resources.iter().flatten() {
        let mut save_file = |props: &SaveFileProps| {
            let file_data = read_file(&props.normalized_path)?;
            let _: ZenodoDepositionFile = make_zenodo_api_request(
                &files_endpoint,
                Some("POST"),
                None,
                Some((&props.denormalized_path, &file_data)),
                Some(api_key),
                sandbox,
            )?;
            Ok(props.denormalized_path.clone())
        };

        resource_descriptors.push(Value::Object(save_resource_files(
            resource,
            &mut save_file,
            basepath.as_deref(),
            false,
            true,
        )?));
    }

    let denormalized = denormalize_dataset(dataset, basepath.as_deref())?;
    let mut descriptor = match serde_json::to_value(&denormalized) {
        Ok(Value::Object(descriptor)) => descriptor,
        _ => return Err(Error::Parse("Dataset is not an object".to_string())),
    };
    descriptor.insert("resources".to_string(), Value::Array(resource_descriptors));

    let descriptor_bytes = stringify_descriptor(&descriptor).into_bytes();
    let _: ZenodoDepositionFile = make_zenodo_api_request(
        &files_endpoint,
        Some("POST"),
        None,
        Some(("dataset.json", &descriptor_bytes)),
        Some(api_key),
        sandbox,
    )?;

    let html_url = url::Url::parse(&zenodo_record.links.html)
        .map_err(|error| Error::Parse(format!("{}: {error}", zenodo_record.links.html)))?;
    let origin = &html_url[..url::Position::BeforePath];

    let mut output = Descriptor::new();
    output.insert(
        "path".to_string(),
        Value::from(format!("{origin}/records/{record_id}/files/dataset.json")),
    );
    output.insert(
        "dataset_url".to_string(),
        Value::from(format!("{origin}/uploads/{record_id}")),
    );
    Ok(output)
}

fn read_file(path: &str) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    load_file_stream(ResourceDataPath::Path(path.to_string()), None, None)?
        .read_to_end(&mut data)
        .map_err(Error::Io)?;
    Ok(data)
}
