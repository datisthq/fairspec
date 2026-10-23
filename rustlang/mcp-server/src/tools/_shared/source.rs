use fairspec_library::DatasetSource;
use fairspec_library::fairspec_metadata::Dataset;
use serde::Deserialize;

/// Python's `Dataset | str`: a file path or a dataset object.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DatasetInput {
    Path(String),
    Dataset(Box<Dataset>),
}

impl From<DatasetInput> for DatasetSource {
    fn from(input: DatasetInput) -> Self {
        match input {
            DatasetInput::Path(path) => DatasetSource::Path(path),
            DatasetInput::Dataset(dataset) => DatasetSource::Dataset(dataset),
        }
    }
}
