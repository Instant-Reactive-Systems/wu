mod ui;
pub mod prelude;

use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { <ui::App/> })
}