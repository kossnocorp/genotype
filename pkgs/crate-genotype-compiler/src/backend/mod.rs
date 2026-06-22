use crate::prelude::internal::*;

mod system;
pub use system::*;

pub trait GtcBackend: GtcFileProvider + GtcNoticeSink {}
