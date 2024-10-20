use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum GTConfigError {
    #[error(r#"cannot find the config at "{0}""#)]
    #[diagnostic(code(GTCFG101))]
    MissingConfig(PathBuf),

    // #[error(r#"cannot find the config file in root directory "{0}""#)]
    // #[diagnostic(code(GTCFG102))]
    // MissingInRoot(PathBuf),

    // #[error(r#"cannot find the config file in any of the parent directories  "{0}""#)]
    // #[diagnostic(code(GTCFG103))]
    // MissingInParents(PathBuf),

    // #[error("cannot find the config file in current directory")]
    // #[diagnostic(code(GTCFG104))]
    // MissingInCurrent,
    #[error("failed to collect configuration")]
    #[diagnostic(code(GTCFG200))]
    FailedToCollect,
}

impl From<figment::Error> for GTConfigError {
    fn from(err: figment::Error) -> Self {
        eprintln!("=========== figment error: {}", err);
        GTConfigError::FailedToCollect
    }
}
