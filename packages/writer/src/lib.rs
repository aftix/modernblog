include!(concat!(env!("OUT_DIR"), "/api.rs"));

pub mod home;
pub mod login;
pub mod missing;
pub mod new;
pub mod route;
pub mod util;

#[doc(hidden)]
#[cfg(test)]
pub(crate) mod test {
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser);
}