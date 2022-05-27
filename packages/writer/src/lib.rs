include!(concat!(env!("OUT_DIR"), "/api.rs"));

pub mod home;
pub mod login;
pub mod missing;
pub mod new;
pub mod route;

#[doc(hidden)]
#[cfg(test)]
pub(crate) mod test {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);

    pub fn obtain_result() -> String {
        gloo::utils::document()
            .get_element_by_id("result")
            .expect("No result found. Most likely, the application crashed and burned")
            .inner_html()
    }
}