use fairspec_metadata::{Error, Result};
use polars::prelude::PolarsResult;

/// Turns a polars failure into the crate error; the orphan rule rules out a `From` impl.
pub trait PolarsResultExt<T> {
    fn into_result(self) -> Result<T>;
}

impl<T> PolarsResultExt<T> for PolarsResult<T> {
    fn into_result(self) -> Result<T> {
        self.map_err(|error| Error::Load(error.to_string()))
    }
}
