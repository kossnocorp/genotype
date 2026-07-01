mod system;
pub use system::*;

pub trait GtpRuntime {
    type LoaderKind;

    type FileSourceKind;

    type FileSinkKind;

    type DiagnosticSinkKind;

    type FormatterRunnerKind;

    type ProjectRef;
}
