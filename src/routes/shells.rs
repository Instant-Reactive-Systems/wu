use leptos::*;
use crate::components::*;

crate::generate_marker_type!(ShellsExample);

#[component]
pub fn ShellsRoute() -> impl IntoView {
    let push_shell_cx = expect_context::<PushShellContext<crate::Main>>();
    let pop_shell_cx = expect_context::<PopShellContext<crate::Main>>();
    let active_shell_cx = expect_context::<ActiveShellContext<crate::Main>>()();
    create_effect(move |_| push_shell_cx(ShellContext {
        header: active_shell_cx.header.clone(),
        left_sidebar: ViewFn::from(move || view! { <div class="bg-surface-600 w-32 h-full h-8"/> }),
        right_sidebar: ViewFn::from(move || view! { <div class="bg-surface-600 w-32 h-full h-8"/> }),
        footer: ViewFn::from(move || view! { <div class="bg-surface-400 w-full h-10"/> }),
    }));
    on_cleanup(move || pop_shell_cx(()));

    view! {
        <div class="overlay">
            <Shell<ShellsExample>
                header=move || view! { <div class="bg-blue-600 h-10"/> }
            >
                <div class="flex center">
                    <p class="bg-surface-800">"content"</p>
                </div>
            </Shell<ShellsExample>>
        </div>
    }
}

