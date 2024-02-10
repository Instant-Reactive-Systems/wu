use crate::components::*;
use leptos::*;

#[component]
pub fn Drawers() -> impl IntoView {
    let open_drawer = expect_context::<OpenDrawer<crate::Main>>();
    let close_drawer = expect_context::<CloseDrawer<crate::Main>>();

    view! {
        <div class="flex center">
            <button on:click=move |_| open_drawer(()) class="btn bg-green-600 rounded-lg">"open drawer"</button>
            <button on:click=move |_| close_drawer(()) class="btn bg-red-600 rounded-lg">"close drawer"</button>
        </div>
    }
}
