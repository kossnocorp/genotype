use crate::prelude::internal::*;

pub trait GtbRemoteInterop {
    fn send_request<'a>(
        &'a self,
        request: GtbRemoteBackendRequest,
    ) -> LocalBoxFuture<'a, Result<GtbRemoteBackendRequestResponse>>;
}
