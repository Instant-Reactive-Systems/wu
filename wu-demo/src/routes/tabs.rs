use crate::components::*;
use leptos::*;
use crate::Main;

struct Tab {
    name: TextProp,
    content: TextProp,
}

#[component]
pub fn TabsRoute() -> impl IntoView {
    view! {
        <wu::Tabs<Main, Tab>
            class="flex flex-row divide-x divide-surface-800"
            list_class="flex flex-col divide-y divide-surface-800 w-[300px] scroll-lock"
            item=#[allow(unused_parens)] (move |tab: wu::TabSignal<Tab>| {
                let switch_active_tab = expect_context::<wu::SwitchActiveTab<Tab>>();
                view! {
                    <li class="w-full">
                        <button on:click=move |_| switch_active_tab(tab.with(|tab| tab.id)) class="cover flex center font-bold btn">
                            {move || tab.with(|tab| tab.name.clone())}
                        </button>
                    </li>
                }
            })
            content=#[allow(unused_parens)] (move |tab: wu::TabSignal<Tab>| {
                let add_tab = expect_context::<wu::AddTab<Main, Tab>>();
                let remove_tab = expect_context::<wu::RemoveTab<Main>>();
                let modify_tab = expect_context::<wu::ModifyTab<Main, Tab>>();
                let remove_tabs = expect_context::<wu::RemoveTabs<Main>>();
                let remove_other_tabs = expect_context::<wu::RemoveOtherTabs<Main>>();
                view! {
                    <div class="grow flex flex-col p-8">
                        <h1 class="text-xl tablet:text-3xl text-center">{move || tab.with(|tab| tab.content.clone())}</h1>
                        <div class="grow flex center">
                            <ul class="flex flex-row gap-4">
                                <button on:click=move |_| add_tab(Tab { name: "New Tab".into(), content: "New Tab content".into() }) class="btn bg-yellow-600 rounded-lg">"add tab"</button>
                                <button on:click=move |_| remove_tab(tab.with(|tab| tab.id)) class="btn bg-yellow-600 rounded-lg">"remove tab"</button>
                                <button on:click=move |_| modify_tab((tab.with(|tab| tab.id), Tab { name: "Modified Tab".into(), content: "Modified Tab content".into() })) class="btn bg-yellow-600 rounded-lg">"modify tab"</button>
                                <button on:click=move |_| remove_tabs(vec![tab.with(|tab| tab.id)]) class="btn bg-yellow-600 rounded-lg">"remove tabs"</button>
                                <button on:click=move |_| remove_other_tabs(tab.with(|tab| tab.id)) class="btn bg-yellow-600 rounded-lg">"remove other tabs"</button>
                            </ul>
                        </div>
                    </div>
                }
            })
            list_fallback=move || "No tabs"
            content_fallback=move || "No tab selected"
            tabs=vec![
                Tab { name: "Tab #1".into(), content: "Tab #1 content".into() },
                Tab { name: "Tab #2".into(), content: "Tab #2 content".into() },
            ]
        />
    }
}
