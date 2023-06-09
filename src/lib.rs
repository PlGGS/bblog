use cfg_if::cfg_if;
pub mod auth;
pub mod error_template;
pub mod errors;
pub mod fallback;
pub mod state;
pub mod app;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;
        use app::*;
        use leptos::view;

        #[wasm_bindgen]
        pub fn hydrate() {
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            leptos::mount_to_body(|cx| {
                view! { cx,  <App/> }
            });
        }
    }
}
