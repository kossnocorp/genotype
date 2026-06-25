use crate::prelude::internal::*;

pub struct GtcBackendSystem;

impl GtcDiagnosticSinkStdio for GtcBackendSystem {}

impl GtcFileProviderSystem for GtcBackendSystem {}

impl GtcBackend for GtcBackendSystem {}
