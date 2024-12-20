use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq)]
pub enum GTCliError {
    #[error("missing command")]
    #[diagnostic(code(GTC100))]
    MissingCommand,

    #[error("unknown command `{0}`")]
    #[diagnostic(code(GTC101))]
    UnknownCommand(String),

    #[error("failed to read line for {0}")]
    #[diagnostic(code(GTC102))]
    FailedReadline(&'static str),

    #[error("failed to write file `{0}`")]
    #[diagnostic(code(GTC103))]
    FailedWrite(String),

    #[error("failed to create directory `{0}`")]
    #[diagnostic(code(GTC104))]
    FailedCreateDir(String),

    #[error("{0} not found")]
    #[diagnostic(code(GTC200))]
    Canonicalize(String),

    #[error("failed to generate project")]
    #[diagnostic(code(GTC300))]
    Generate,

    #[error("failed to render project")]
    #[diagnostic(code(GTC400))]
    Render,

    #[error("failed to stringify config")]
    #[diagnostic(code(GTC401))]
    StringifyConfig,

    #[error("failed to write project")]
    #[diagnostic(code(GTC500))]
    Write,
}
