use super::denormalize::denormalize_dataset;
use crate::actions::descriptor::save::save_profiled_model;
use crate::actions::path::basepath::get_basepath;
use crate::models::dataset::Dataset;
use crate::models::exception::Result;
use crate::models::profile::ProfileType;

pub fn save_dataset_descriptor(dataset: &Dataset, path: &str, overwrite: bool) -> Result<()> {
    let basepath = get_basepath(path);
    let denormalized = denormalize_dataset(dataset, Some(&basepath))?;
    save_profiled_model(&denormalized, ProfileType::Dataset, path, overwrite)
}
