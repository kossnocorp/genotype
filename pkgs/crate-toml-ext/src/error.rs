use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum TomlExtError {
    #[error("Failed to cast {0} to table")]
    FailedCast(String),

    #[error("Invalid path `{0}`")]
    InvalidPath(String),
}

pub type TomlExtResult<T> = std::result::Result<T, TomlExtError>;
