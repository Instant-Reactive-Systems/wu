use crate::components::*;
use leptos::*;

#[component]
pub fn Drawers() -> impl IntoView {
    let toggle_drawer = expect_context::<ToggleDrawer>().0;
    let open_drawer = move |_| {
        toggle_drawer(true);
    };

    view! {
        <div class="overlay flex center">
            <button on:click=open_drawer class="btn bg-green-600 rounded-lg">"open drawer"</button>
        </div>
    }
}
