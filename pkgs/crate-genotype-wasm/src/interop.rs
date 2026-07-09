use crate::prelude::internal::*;

pub struct GtwmInterop {
    backend_request_handler: Function,
}

impl GtwmInterop {
    pub fn new(backend_request_handler: Function) -> Self {
        Self {
            backend_request_handler,
        }
    }
}

impl GtbRemoteInterop for GtwmInterop {
    fn send_request<'a>(
        &'a self,
        request: GtbRemoteBackendRequest,
    ) -> LocalBoxFuture<'a, Result<GtbRemoteBackendRequestResponse>> {
        async move {
            let request = serde_wasm_bindgen::to_value(&request).map_err(|err| {
                miette!("Wasm interop failed to serialize remote backend request: {err}")
            })?;

            let response = self
                .backend_request_handler
                .call1(&JsValue::NULL, &request)
                .map_err(|err| {
                    miette!(
                        "Wasm interop remote request failed: {}",
                        js_error_to_string(err)
                    )
                })?;

            let promise: Promise = response
                .dyn_into()
                .map_err(|_| miette!("Wasm interop request handler did not return a Promise"))?;
            let response = JsFuture::from(promise).await.map_err(|err| {
                miette!(
                    "Wasm interop remote request promise rejected: {}",
                    js_error_to_string(err)
                )
            })?;

            serde_wasm_bindgen::from_value(response).map_err(|err| {
                miette!("Wasm interop failed to deserialize remote backend response: {err}")
            })
        }
        .boxed_local()
    }
}
