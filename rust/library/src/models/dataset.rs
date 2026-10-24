use fairspec_metadata::{Dataset, Descriptor};

/// Python's `Dataset | Descriptor | str`: a dataset, its descriptor, or a source to load.
#[derive(Debug, Clone)]
pub enum DatasetSource {
    Dataset(Box<Dataset>),
    Descriptor(Descriptor),
    Path(String),
}

impl From<Dataset> for DatasetSource {
    fn from(dataset: Dataset) -> Self {
        DatasetSource::Dataset(Box::new(dataset))
    }
}

impl From<Descriptor> for DatasetSource {
    fn from(descriptor: Descriptor) -> Self {
        DatasetSource::Descriptor(descriptor)
    }
}

impl From<String> for DatasetSource {
    fn from(path: String) -> Self {
        DatasetSource::Path(path)
    }
}

impl From<&str> for DatasetSource {
    fn from(path: &str) -> Self {
        DatasetSource::Path(path.to_string())
    }
}
