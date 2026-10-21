use crate::actions::path::denormalize::denormalize_path;
use crate::actions::resource::normalize::map_resource_paths;
use crate::models::dataset::Dataset;
use crate::models::exception::Result;

pub fn denormalize_dataset(dataset: &Dataset, basepath: Option<&str>) -> Result<Dataset> {
    let mut dataset = dataset.clone();
    for resource in dataset.resources.iter_mut().flatten() {
        map_resource_paths(resource, |path| denormalize_path(path, basepath))?;
    }
    Ok(dataset)
}
