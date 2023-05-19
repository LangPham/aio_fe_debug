mod app;
pub(crate) mod components;
pub(crate) mod layout;
pub(crate) mod models;
pub(crate) mod pages;

use app::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // console_error_panic_hook::set_once();
    leptos::mount_to_body(|cx| leptos::view! { cx,  <App/> })
}
