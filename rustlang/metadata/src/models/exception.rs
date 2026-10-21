use super::report::Report;

pub type Result<T> = std::result::Result<T, Error>;

/// The crate's error; `Invalid` is Python's `FairspecException`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{message}")]
    Invalid {
        message: String,
        report: Option<Report>,
    },
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Http(String),
    #[error("{0}")]
    Parse(String),
    #[error("{0}")]
    Load(String),
    #[error("{0}")]
    Path(String),
    #[error("{0}")]
    Profile(String),
}

impl Error {
    pub fn report(&self) -> Option<&Report> {
        match self {
            Error::Invalid { report, .. } => report.as_ref(),
            _ => None,
        }
    }
}
