use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum GTCliError {
    #[error("missing command")]
    #[diagnostic(code(GTCL100))]
    MissingCommand,

    #[error("unknown command `{0}`")]
    #[diagnostic(code(GTCL101))]
    UnknownCommand(String),

    #[error("failed to read line for {0}")]
    #[diagnostic(code(GTCL102))]
    FailedReadline(&'static str),

    #[error("failed to write file `{0}`")]
    #[diagnostic(code(GTCL103))]
    FailedWrite(String),

    #[error("failed to create directory `{0}`")]
    #[diagnostic(code(GTCL104))]
    FailedCreateDir(String),

    #[error("{0} not found")]
    #[diagnostic(code(GTCL200))]
    Canonicalize(String),

    #[error("failed to generate project")]
    #[diagnostic(code(GTCL300))]
    Generate,

    #[error("failed to render project")]
    #[diagnostic(code(GTCL400))]
    Render,

    #[error("failed to stringify config")]
    #[diagnostic(code(GTCL401))]
    StringifyConfig,

    #[error("failed to write project")]
    #[diagnostic(code(GTCL500))]
    Write,
}
