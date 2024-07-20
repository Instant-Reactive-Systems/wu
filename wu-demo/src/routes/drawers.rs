use crate::components::*;
use leptos::*;

#[component]
pub fn Drawers() -> impl IntoView {
    let open_drawer = expect_context::<wu::OpenDrawer<crate::Main>>();
    let close_drawer = expect_context::<wu::CloseDrawer<crate::Main>>();

    view! {
        <div class="h-[1400px] flex center">
            <button on:click=move |_| open_drawer(()) class="btn bg-green-600 rounded-lg">"open drawer"</button>
            <button on:click=move |_| close_drawer(()) class="btn bg-red-600 rounded-lg">"close drawer"</button>
        </div>
    }
}
