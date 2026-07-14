use crate::prelude::internal::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn js_error_to_string(value: JsValue) -> String {
    // Handle `throw new Error("message")`
    if let Some(error) = value.dyn_ref::<js_sys::Error>() {
        let message: String = error.message().into();

        let stack = Reflect::get(&value, &JsValue::from_str("stack"))
            .ok()
            .and_then(|stack| stack.as_string());

        return match stack {
            Some(stack) if !stack.is_empty() => stack,
            _ => message,
        };
    }

    // Handle `throw "message"`
    if let Some(string) = value.as_string() {
        return string;
    }

    // Handle custom objects with `.message`
    if let Ok(message) = Reflect::get(&value, &JsValue::from_str("message"))
        && let Some(message) = message.as_string()
    {
        return message;
    }

    // Fallback equivalent-ish to JS `String(value)`
    js_sys::JsString::from(value).into()
}
