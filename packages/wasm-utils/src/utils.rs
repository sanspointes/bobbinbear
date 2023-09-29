use wasm_bindgen::JsValue;

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    // #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn str_to_char(string: &str) -> Option<char> {
    if string.chars().count() == 1 {
        return string.chars().next();
    }
    None
}

pub fn js_value_to_char(js_value: JsValue) -> Option<char> {
    if let Some(js_string) = js_value.as_string() {
        // Convert the JavaScript string to a Rust string.
        let rust_string = js_string;

        // Ensure the Rust string consists of exactly one Unicode scalar value.
    }
    None
}
