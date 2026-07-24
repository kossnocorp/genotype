use crate::prelude::internal::*;

mod py;
mod rs;
mod ts;

use py::format_py;
use rs::format_rs;
use ts::format_ts;

#[wasm_bindgen]
#[wasm_bindgen(js_name = GtwmFormatters)]
pub struct GtwmFormatters;

#[wasm_bindgen]
impl GtwmFormatters {
    #[wasm_bindgen(constructor)]
    pub fn new() -> std::result::Result<Self, JsValue> {
        set_panic_hook();
        Ok(Self)
    }

    #[wasm_bindgen(js_name = formatTs)]
    pub fn format_ts(&mut self, code: String) -> std::result::Result<String, JsValue> {
        format_ts(&code).map_err(|err| JsValue::from_str(&err))
    }

    #[wasm_bindgen(js_name = formatRs)]
    pub fn format_rs(&mut self, code: String) -> std::result::Result<String, JsValue> {
        format_rs(&code).map_err(|err| JsValue::from_str(&err))
    }

    #[wasm_bindgen(js_name = formatPy)]
    pub fn format_py(&mut self, code: String) -> std::result::Result<String, JsValue> {
        format_py(&code).map_err(|err| JsValue::from_str(&err))
    }
}
