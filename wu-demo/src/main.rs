pub mod routes;
pub mod components;
pub mod utils;
pub mod prelude;

use prelude::*;
use routes::AppRouter;


wu::generate_marker_type!(
    /// General marker type.
    Main
);

pub fn main() {
    #[cfg(debug_assertions)]
    let level = log::Level::Debug;
    #[cfg(not(debug_assertions))]
    let level = log::Level::Info;
    _ = console_log::init_with_level(level);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Body class="min-h-lvh overlay-container surface-bg-1 text-light-content dark:text-dark-content [&_*]:x-[thin-scrollbar,scroll-light-2,thumb-light-3] dark:[&_*]:x-[scroll-dark-2,thumb-dark-3]"/>
        <Title text="wu demo"/>
        <AppRouter />
    }
}

#[derive(thiserror::Error, Clone, Debug)]
#[error("({code}): {what}")]
pub struct AppError {
    pub code: http::status::StatusCode,
    pub what: String,
}
