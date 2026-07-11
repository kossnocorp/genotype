use crate::prelude::internal::*;
use std::time::{Duration, Instant};

pub struct GtcMetaState {
    pub timing: GtcMetaTimingState,
}

impl GtcMetaState {
    pub fn new() -> Self {
        Self {
            timing: GtcMetaTimingState::New {
                started_at: Instant::now(),
            },
        }
    }
}

pub enum GtcMetaTimingState {
    New {
        started_at: Instant,
    },
    LoadedProject {
        started_at: Instant,
        load_project: Duration,
    },
    LoadedModules {
        started_at: Instant,
        load_project: Duration,
        load_modules: Duration,
    },
    Compiled {
        started_at: Instant,
        load_project: Duration,
        load_modules: Duration,
        timing: GtMetaTiming,
    },
}

impl GtcMetaTimingState {
    pub fn started_at(&self) -> Option<Instant> {
        match self {
            Self::New { started_at }
            | Self::LoadedProject { started_at, .. }
            | Self::LoadedModules { started_at, .. }
            | Self::Compiled { started_at, .. } => Some(*started_at),
        }
    }
}

pub(crate) fn duration_ms(duration: Duration) -> u64 {
    duration.as_millis().try_into().unwrap_or(u64::MAX)
}
