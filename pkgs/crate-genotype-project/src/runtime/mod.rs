use crate::prelude::internal::*;

mod system;
pub use system::*;

pub trait GtpRuntime<ProjectRef>: GtpLoader<ProjectRef> + GtpFileSource {}
