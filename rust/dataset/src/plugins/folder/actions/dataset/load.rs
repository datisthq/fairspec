use std::path::Path;

use fairspec_metadata::{Dataset, Result, load_dataset_descriptor};

use crate::actions::file::temp::format_path;

pub fn load_dataset_from_folder(folder_path: &str) -> Result<Dataset> {
    load_dataset_descriptor(&format_path(&Path::new(folder_path).join("dataset.json")))
}
