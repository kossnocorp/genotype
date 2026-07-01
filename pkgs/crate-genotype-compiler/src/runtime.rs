use crate::prelude::internal::*;

pub trait GtcRuntime:
    GtpRuntime
    + GtpFileSource<<Self as GtpRuntime>::FileSourceKind>
    + GtpFileSink<<Self as GtpRuntime>::FileSinkKind>
    + GtpDiagnosticSink<<Self as GtpRuntime>::DiagnosticSinkKind>
    + GtpFormatterRunner<Self::FormatterRunnerKind, Self::DiagnosticSinkKind>
{
}

impl<Runtime: ?Sized> GtcRuntime for Runtime where
    Runtime: GtpRuntime
        + GtpFileSource<<Runtime as GtpRuntime>::FileSourceKind>
        + GtpFileSink<<Runtime as GtpRuntime>::FileSinkKind>
        + GtpDiagnosticSink<<Runtime as GtpRuntime>::DiagnosticSinkKind>
        + GtpFormatterRunner<
            <Runtime as GtpRuntime>::FormatterRunnerKind,
            <Runtime as GtpRuntime>::DiagnosticSinkKind,
        >
{
}
