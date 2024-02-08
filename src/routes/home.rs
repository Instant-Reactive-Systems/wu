use leptos::*;
use leptos_router::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="overlay flex center flex-col gap-8">
            <h1 class="font-bold text-2xl">"Components"</h1>
            <nav>
                <ul class="flex flex-row flex-wrap gap-4 [&>*>a]:link [&>*>a]:link-white">
                    <li><A href="toasts">"toasts"</A></li>
                    <li><A href="modals">"modals"</A></li>
                    <li><A href="collapses">"collapses"</A></li>
                    <li><A href="drawers">"drawers"</A></li>
                    <li><A href="shells">"shells"</A></li>
                    <li><A href="tabs">"tabs"</A></li>
                    <li><A href="accordion">"accordion"</A></li>
                </ul>
            </nav>
        </div>
    }
}
