use super::data::get_data_paths;
use crate::actions::path::general::get_is_remote_path;
use crate::models::resource::Resource;

pub fn get_is_remote_resource(resource: &Resource) -> bool {
    get_data_paths(resource)
        .iter()
        .any(|path| get_is_remote_path(path))
}
