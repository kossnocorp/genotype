use crate::prelude::internal::*;

pub struct GtcBackendSystem;

impl GtcNoticeSinkStdio for GtcBackendSystem {}

impl GtcFileProviderSystem for GtcBackendSystem {}

impl GtcBackend for GtcBackendSystem {}
