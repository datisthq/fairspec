use super::normalize::{map_resource_paths, to_descriptor};
use crate::actions::path::denormalize::denormalize_path;
use crate::models::descriptor::Descriptor;
use crate::models::exception::Result;
use crate::models::resource::Resource;

pub fn denormalize_resource(resource: &Resource, basepath: Option<&str>) -> Result<Descriptor> {
    let mut resource = resource.clone();
    map_resource_paths(&mut resource, |path| denormalize_path(path, basepath))?;
    to_descriptor(&resource)
}
