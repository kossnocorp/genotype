use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum GTConfigError {
    #[error(r#"cannot find the config at "{0}""#)]
    #[diagnostic(code(GTCFG101))]
    MissingConfig(PathBuf),

    #[error("failed to collect configuration")]
    #[diagnostic(code(GTCFG200))]
    FailedToCollect,

    #[error("failed to construct entry pattern")]
    #[diagnostic(code(GTCFG301))]
    FailedToConstructEntry,

    #[error("cannot derive Python package name, please set `python.name` or `name`.")]
    #[diagnostic(code(GTCFG401))]
    PythonMissingPackageName,

    #[error(
        "cannot derive Python module name, please set `python.module`, `python.name` or `name`."
    )]
    #[diagnostic(code(GTCFG402))]
    PythonMissingModuleName,
}

impl From<figment::Error> for GTConfigError {
    fn from(err: figment::Error) -> Self {
        eprintln!("=========== figment error: {}", err);
        GTConfigError::FailedToCollect
    }
}
