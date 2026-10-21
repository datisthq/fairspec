use std::fs::File;
use std::path::Path;

use fairspec_metadata::{Dataset, Error, Result, load_dataset_descriptor};

use crate::actions::file::temp::format_path;
use crate::actions::folder::temp::get_temp_folder_path;

pub fn load_dataset_from_zip(archive_path: &str) -> Result<Dataset> {
    let basepath = get_temp_folder_path(false)?;

    let mut archive = zip::ZipArchive::new(File::open(archive_path)?)
        .map_err(|error| Error::Load(format!("{archive_path}: {error}")))?;
    archive
        .extract(&basepath)
        .map_err(|error| Error::Load(format!("{archive_path}: {error}")))?;

    load_dataset_descriptor(&format_path(&Path::new(&basepath).join("dataset.json")))
}
