use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug, PartialEq, Clone)]
pub enum GTWError {
    #[error("Failed to lock the files map")]
    #[diagnostic(code(GTW001))]
    FilesLock,

    #[error("`{0}` not found")]
    #[diagnostic(
        code(GTW101),
        help("Make sure the path is absolute or can be resolved from the current directory")
    )]
    CanonicalizePath(String),

    #[error("Can't detect the kind of `{0}`")]
    #[diagnostic(
        code(GTW102),
        help("The file must be named `genotype(.*)?.toml` or `*.type`")
    )]
    DetectKind(String),

    #[error("Can't read `{0}`")]
    #[diagnostic(
        code(GTW103),
        help("Does the file exist? Do you have the right permissions?")
    )]
    ReadSource(String),
}
