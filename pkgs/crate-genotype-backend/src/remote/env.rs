use crate::prelude::internal::*;

pub trait GtbRemoteEnv {
    /// Returns the remote interop implementation for the environment.
    fn remote_interop(&self) -> &dyn GtbRemoteInterop;
}
