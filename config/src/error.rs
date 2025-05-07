use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum GtConfigError {
    #[error(r#"cannot find the config at "{0}""#)]
    #[diagnostic(code(GTCF101))]
    MissingConfig(PathBuf),

    #[error("failed to collect configuration")]
    #[diagnostic(code(GTCF200))]
    FailedToCollect(#[from] figment::Error),

    #[error("failed to construct entry pattern")]
    #[diagnostic(code(GTCF301))]
    FailedToConstructEntry,

    #[error("cannot derive Python package name, please set `python.name` or `name`.")]
    #[diagnostic(code(GTCF401))]
    PythonMissingPackageName,

    #[error(
        "cannot derive Python module name, please set `python.module`, `python.name` or `name`."
    )]
    #[diagnostic(code(GTCF402))]
    PythonMissingModuleName,
}

// impl From<figment::Error> for GtConfigError {
//     fn from(err: figment::Error) -> Self {
//         GtConfigError::FailedToCollect(err)
//     }
// }
