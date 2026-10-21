use super::assert::assert_dataset;
use crate::actions::descriptor::load::load_descriptor;
use crate::actions::path::basepath::resolve_basepath;
use crate::models::dataset::Dataset;
use crate::models::exception::Result;

pub fn load_dataset_descriptor(path: &str) -> Result<Dataset> {
    let basepath = resolve_basepath(path)?;
    let descriptor = load_descriptor(path, false)?;
    assert_dataset(&descriptor, Some(&basepath))
}
