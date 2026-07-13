use crate::prelude::internal::*;

#[wasm_bindgen]
#[wasm_bindgen(js_name = GtwmCompiler)]
pub struct GtwmCompiler {
    compiler: GtcRemote,
}

#[wasm_bindgen]
impl GtwmCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "cwdPath")] cwd_path: &str,
        #[wasm_bindgen(js_name = "basePath")] base_path: &str,
        #[wasm_bindgen(js_name = "backendRequestHandler")] backend_request_handler: Function,
    ) -> std::result::Result<Self, JsValue> {
        set_panic_hook();

        let interop = GtwmInterop::new(backend_request_handler.clone());

        let compiler = GtcRemote::new((
            Box::new(interop),
            GtpCwdPath::from(cwd_path),
            GtpCwdRelativePath::from(base_path),
            None,
        ))
        .map_err(|err| JsValue::from_str(&format!("Failed to create compiler: {err}")))?;

        Ok(Self { compiler })
    }

    #[wasm_bindgen(js_name = handleRuntimeRequest)]
    pub async fn handle_runtime_request(
        &mut self,
        request: JsValue,
    ) -> std::result::Result<JsValue, JsValue> {
        let request: GtcRemoteRuntimeRequest = serde_wasm_bindgen::from_value(request)
            .map_err(|err| JsValue::from_str(&format!("Failed to parse worker message: {err}")))?;

        let response = self
            .compiler
            .handle_runtime_request(request)
            .await
            .map_err(|err| {
                JsValue::from_str(&format!("Failed to handle runtime request: {err}"))
            })?;

        serde_wasm_bindgen::to_value(&response).map_err(|err| {
            JsValue::from_str(&format!(
                "Failed to serialize runtime request response: {err}"
            ))
        })
    }
}
