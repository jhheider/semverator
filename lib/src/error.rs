use thiserror::Error;

/// Errors from parsing and manipulating semantic versions and ranges.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    #[error("invalid semver: {0}")]
    Semver(String),
    #[error("invalid range: {0}")]
    Range(String),
}

/// Convenience alias for results returning [`Error`].
pub type Result<T> = std::result::Result<T, Error>;
