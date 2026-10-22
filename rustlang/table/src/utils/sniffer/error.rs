#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SnifferError {
    #[error("{0}")]
    Sniffer(String),
    #[error("{0}")]
    Encoding(String),
    #[error("{0}")]
    Parse(String),
}
