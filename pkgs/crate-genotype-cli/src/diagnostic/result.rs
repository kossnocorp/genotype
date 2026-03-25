use super::error::GtCliError;

pub type GtCliResult<T> = Result<T, GtCliError>;
