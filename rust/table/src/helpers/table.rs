use fairspec_metadata::Result;
use polars::prelude::*;

use super::error::PolarsResultExt;
use crate::settings::QUERY_ENGINE;

/// Runs queries on the streaming engine, as py-polars' `engine=QUERY_ENGINE`;
/// the default `collect()` holds the whole source in memory.
pub trait LazyFrameExt {
    fn collect_table(self) -> Result<DataFrame>;
    fn sink_table(self, path: &str, format: FileWriteFormat) -> Result<()>;
}

impl LazyFrameExt for LazyFrame {
    fn collect_table(self) -> Result<DataFrame> {
        Ok(self
            .collect_with_engine(QUERY_ENGINE)
            .into_result()?
            .unwrap_single())
    }

    fn sink_table(self, path: &str, format: FileWriteFormat) -> Result<()> {
        let target = SinkTarget::Path(PlRefPath::new(path));
        self.sink(
            SinkDestination::File { target },
            format,
            UnifiedSinkArgs::default(),
        )
        .into_result()?
        .collect_with_engine(QUERY_ENGINE)
        .into_result()?;
        Ok(())
    }
}
