use std::path::PathBuf;

use miette::Diagnostic;
use semver::Version;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum GtpConfigError {
    #[error(r#"cannot find the config at "{0}""#)]
    #[diagnostic(code(GTCF101))]
    MissingConfig(PathBuf),

    #[error("failed to collect configuration")]
    #[diagnostic(code(GTCF200))]
    FailedToCollect(#[from] figment::Error),

    #[error("failed to construct entry pattern")]
    #[diagnostic(code(GTCF301))]
    FailedToConstructEntry,

    #[error("failed to stringify config")]
    #[diagnostic(code(GTCF302))]
    FailedToStringify,

    #[error("failed to save config file `{0}`")]
    #[diagnostic(code(GTCF303))]
    FailedSaveConfig(String),

    #[error("cannot derive Python package name, please set `python.name` or `name`.")]
    #[diagnostic(code(GTCF401))]
    PythonMissingPackageName,

    #[error(
        "cannot derive Python module name, please set `python.module`, `python.name` or `name`."
    )]
    #[diagnostic(code(GTCF402))]
    PythonMissingModuleName,

    #[error("invalid manifest version `{0}`, expected x.y.z")]
    #[diagnostic(code(GTCF500))]
    VersionInvalid(String),

    #[error("cannot set manifest version {next} lower than current {current}")]
    #[diagnostic(code(GTCF501))]
    VersionLower { current: Version, next: Version },

    #[error("cannot bump manifest version because no global or language manifest versions are set")]
    #[diagnostic(code(GTCF502))]
    ManifestVersionMissingForBump,
}
