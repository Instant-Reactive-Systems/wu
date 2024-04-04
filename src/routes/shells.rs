use crate::components::*;
use leptos::*;

crate::generate_marker_type!(ShellsExample);

#[component]
pub fn ShellsRoute() -> impl IntoView {
    let push_shell_cx = expect_context::<PushShellContext<crate::Main>>();
    let pop_shell_cx = expect_context::<PopShellContext<crate::Main>>();
    create_effect(move |_| {
        push_shell_cx(ShellContext {
            header: None,
            left_sidebar: Some(ViewFn::from(move || view! { <div class="bg-surface-600 w-32 h-full h-8"/> })),
            right_sidebar: Some(ViewFn::from(move || view! { <div class="bg-surface-600 w-32 h-full h-8"/> })),
            footer: Some(ViewFn::from(move || view! { <div class="bg-surface-400 w-full h-10"/> })),
        })
    });
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
