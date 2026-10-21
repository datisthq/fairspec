use std::io::Read;

use fairspec_metadata::{
    Dataset, Descriptor, Error, ResourceDataPath, Result, denormalize_dataset, get_file_extension,
    stringify_descriptor,
};
use serde_json::Value;

use super::to_ckan::convert_dataset_to_ckan;
use crate::actions::dataset::basepath::get_dataset_basepath;
use crate::actions::resource::save::{SaveFileProps, save_resource_files};
use crate::actions::stream::load::load_file_stream;
use crate::plugins::ckan::actions::resource::to_ckan::convert_resource_to_ckan;
use crate::plugins::ckan::models::dataset::CkanDataset;
use crate::plugins::ckan::models::resource::CkanResource;
use crate::plugins::ckan::services::ckan::{
    dump_descriptor, get_url_origin, make_ckan_api_request,
};

pub fn save_dataset_to_ckan(
    dataset: &Dataset,
    api_key: &str,
    ckan_url: &str,
    owner_org: &str,
    dataset_name: &str,
) -> Result<Descriptor> {
    let basepath = get_dataset_basepath(dataset)?;
    let ckan_dataset = convert_dataset_to_ckan(dataset)?;

    let mut payload = dump_descriptor(&ckan_dataset)?;
    payload.insert("name".to_string(), Value::from(dataset_name));
    payload.insert("owner_org".to_string(), Value::from(owner_org));
    payload.insert("resources".to_string(), Value::Array(vec![]));

    let result: CkanDataset =
        make_ckan_api_request(ckan_url, "package_create", &payload, None, Some(api_key))?;

    let dataset_url = format!("{}/dataset/{}", get_url_origin(ckan_url)?, result.name);

    let mut resource_descriptors = Vec::new();
    for resource in dataset.resources.iter().flatten() {
        let mut save_file = |props: &SaveFileProps| {
            let ckan_resource = convert_resource_to_ckan(resource)?;
            let extension = get_file_extension(&props.normalized_path);

            let mut upload_payload = dump_descriptor(&ckan_resource)?;
            upload_payload.insert("package_id".to_string(), Value::from(dataset_name));
            upload_payload.insert(
                "name".to_string(),
                Value::from(props.denormalized_path.as_str()),
            );
            if let Some(extension) = extension.filter(|extension| !extension.is_empty()) {
                upload_payload.insert("format".to_string(), Value::from(extension.to_uppercase()));
            }

            let file_data = read_file(&props.normalized_path)?;
            let upload_result: CkanResource = make_ckan_api_request(
                ckan_url,
                "resource_create",
                &upload_payload,
                Some((&props.denormalized_path, &file_data)),
                Some(api_key),
            )?;

            Ok(upload_result.url)
        };

        resource_descriptors.push(Value::Object(save_resource_files(
            resource,
            &mut save_file,
            basepath.as_deref(),
            true,
            true,
        )?));
    }

    let denormalized = denormalize_dataset(dataset, basepath.as_deref())?;
    let mut descriptor = dump_descriptor(&denormalized)?;
    descriptor.insert("resources".to_string(), Value::Array(resource_descriptors));

    let descriptor_bytes = stringify_descriptor(&descriptor).into_bytes();

    let mut descriptor_payload = Descriptor::new();
    descriptor_payload.insert("package_id".to_string(), Value::from(dataset_name));
    descriptor_payload.insert("name".to_string(), Value::from("dataset.json"));
    let descriptor_resource: CkanResource = make_ckan_api_request(
        ckan_url,
        "resource_create",
        &descriptor_payload,
        Some(("dataset.json", &descriptor_bytes)),
        Some(api_key),
    )?;

    let mut output = Descriptor::new();
    output.insert("path".to_string(), Value::from(descriptor_resource.url));
    output.insert("dataset_url".to_string(), Value::from(dataset_url));
    Ok(output)
}

fn read_file(path: &str) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    load_file_stream(ResourceDataPath::Path(path.to_string()), None, None)?
        .read_to_end(&mut data)
        .map_err(Error::Io)?;
    Ok(data)
}
