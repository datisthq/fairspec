use super::data::get_data_first_path;
use crate::actions::path::general::get_file_name_slug;
use crate::models::resource::Resource;

pub fn infer_resource_name(resource: &Resource, resource_number: Option<usize>) -> String {
    if let Some(name) = get_data_first_path(resource)
        .and_then(|path| get_file_name_slug(&path))
        .filter(|name| !name.is_empty())
    {
        return name;
    }
    match resource_number {
        Some(number) => format!("resource{number}"),
        None => "resource".to_string(),
    }
}
