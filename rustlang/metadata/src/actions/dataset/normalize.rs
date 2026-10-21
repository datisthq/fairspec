use crate::actions::path::normalize::normalize_path;
use crate::actions::resource::normalize::map_resource_paths;
use crate::models::dataset::Dataset;
use crate::models::exception::Result;

// Unlike Python, the result is not re-validated: a cwd-relative path may start with `..`
pub fn normalize_dataset(dataset: &Dataset, basepath: Option<&str>) -> Result<Dataset> {
    let mut dataset = dataset.clone();
    for resource in dataset.resources.iter_mut().flatten() {
        map_resource_paths(resource, |path| normalize_path(path, basepath))?;
    }
    Ok(dataset)
}
