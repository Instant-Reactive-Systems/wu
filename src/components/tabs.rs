use crate::utils::ParamViewFn;
use deref_derive::{Deref, DerefMut};
use leptos::*;

crate::generate_generic_marker_signal_setter!(
    /// Adds a tab.
    AddTab, T, T
);

crate::generate_marker_signal_setter!(
    /// Removes a tab.
    RemoveTab, TabId
);

crate::generate_generic_marker_signal_setter!(
    /// Modify a tab.
    ModifyTab, (TabId, T), T
);

crate::generate_marker_signal_setter!(
    /// Removes several tabs.
    RemoveTabs, Vec<TabId>
);

crate::generate_marker_signal_setter!(
    /// Removes all other tabs.
    RemoveOtherTabs, TabId
);

crate::generate_marker_signal_setter!(
    /// Switches the active tab.
    SwitchActiveTab, TabId
);

/// Type alias for the tab ID.
pub type TabId = u64;
pub type TabSignal<T> = RwSignal<TabWithId<T>>;

/// Use tabs to quickly switch between different views and pages.
#[component]
pub fn Tabs<M: 'static, T: 'static>(
    #[prop(optional)] _phant: std::marker::PhantomData<(M, T)>,
    /// List of tab contexts.
    #[prop(default = Vec::default(), into)]
    tabs: Vec<T>,
    /// Corresponds to the 'class' attribute of elements.
    #[prop(default = "".into(), into)]
    class: TextProp,
    /// List class.
    #[prop(default = "".into(), into)]
    list_class: TextProp,
    /// List item slot.
    #[prop(optional, into)]
    item: ParamViewFn<TabSignal<T>>,
    /// Content slot.
    #[prop(optional, into)]
    content: ParamViewFn<TabSignal<T>>,
    /// List fallback.
    #[prop(optional, into)]
    list_fallback: ViewFn,
    /// Content fallback.
    #[prop(optional, into)]
    content_fallback: ViewFn,
) -> impl IntoView {
    let is_default_full = !tabs.is_empty();
    let (tab_id, set_tab_id) = create_signal::<TabId>(tabs.len() as TabId);
    let (tabs, set_tabs) = create_signal::<Vec<TabSignal<T>>>(
        tabs.into_iter()
            .enumerate()
            .map(|(id, tab)| TabWithId { id: id as u64, tab })
            .map(|tab| create_rw_signal(tab))
            .collect(),
    );
    let (active_tab_id, set_active_tab_id) =
        create_signal::<Option<TabId>>(is_default_full.then_some(0));
    let active_tab = move || {
        with!(|tabs, active_tab_id| {
            active_tab_id.and_then(|active_tab_id| {
                tabs.iter()
                    .find(|tab| tab.with(|tab| tab.id == active_tab_id))
                    .cloned()
            })
        })
    };

    provide_context(AddTab::<M, T>::new(move |tab| {
        // add to tabs
        let id = tab_id.get_untracked();
        let tab = create_rw_signal(TabWithId { id, tab });
        set_tabs.update(move |tabs| tabs.push(tab));
        // autoincrement tab id
        set_tab_id.update(move |tab_id| *tab_id += 1);
        // assign active tab if not set
        if active_tab_id.get_untracked().is_none() {
            set_active_tab_id.update(move |active_tab_id| *active_tab_id = Some(0))
        }
    }));
    provide_context(RemoveTab::<M>::new(move |id| {
        set_tabs.update(move |tabs| tabs.retain(|tab| tab.with_untracked(|tab| tab.id != id)));
        set_active_tab_id.update(move |active_tab_id| {
            *active_tab_id = tabs.with_untracked(|tabs| {
                (!tabs.is_empty()).then_some(tabs[0].with_untracked(|tab| tab.id))
            })
        });
    }));
    provide_context(ModifyTab::<M, T>::new(move |(id, new_tab)| {
        let new_tab = TabWithId { id, tab: new_tab };
        tabs.with(move |tabs| {
            let tab = tabs
                .iter()
                .find(|tab| tab.with_untracked(|tab| tab.id == id));
            if let Some(tab) = tab {
                tab.update(move |tab| *tab = new_tab)
            }
        });
    }));
    provide_context(RemoveTabs::<M>::new(move |tab_ids| {
        set_tabs.update(move |tabs| {
            tabs.retain(|tab| !tab_ids.contains(&tab.with_untracked(|tab| tab.id)))
        });
        set_active_tab_id.update(move |active_tab_id| {
            *active_tab_id = tabs.with_untracked(|tabs| {
                (!tabs.is_empty()).then_some(tabs[0].with_untracked(|tab| tab.id))
            })
        });
    }));
    provide_context(RemoveOtherTabs::<M>::new(move |id| {
        set_tabs.update(move |tabs| tabs.retain(|tab| tab.with_untracked(|tab| tab.id == id)));
        set_active_tab_id.update(move |active_tab_id| {
            *active_tab_id = tabs.with_untracked(|tabs| {
                (!tabs.is_empty()).then_some(tabs[0].with_untracked(|tab| tab.id))
            })
        });
    }));
    provide_context(SwitchActiveTab::<T>::new(move |id| {
        // find if the tab exists
        if tabs.with(move |tabs| {
            !tabs
                .iter()
                .any(|tab| tab.with_untracked(|tab| tab.id == id))
        }) {
            tracing::error!("SwitchActiveTab: tab with id '{id}' does not exist");
        }
        // update the active tab id
        set_active_tab_id.update(move |active_tab_id| *active_tab_id = Some(id))
    }));

    let item = store_value(item);

    view! {
        <wu-tabs class=class>
            <ul class=list_class>
                <Show
                    when=move || tabs.with(|tabs| !tabs.is_empty())
                    fallback=list_fallback
                >
                    <For
                        each=tabs
                        key=move |tab| tab.with(|tab| tab.id)
                        let:tab
                    >
                        {move || item.get_value().run(tab)}
                    </For>
                </Show>
            </ul>
            {move || match active_tab() {
                Some(tab) => content.run(tab),
                None => content_fallback.run(),
            }}
        </wu-tabs>
    }
}

/// A wrapper around a Tab that holds its ID.
///
/// Used for keyed-for.
#[derive(Clone, Deref, DerefMut)]
pub struct TabWithId<T> {
    /// Tab's unique ID.
    pub id: u64,
    /// The tab itself.
    #[deref]
    tab: T,
}

impl<T: std::fmt::Debug> std::fmt::Debug for TabWithId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::any::type_name::<Self>())
            .field("id", &self.id)
            .field("tab", &self.tab)
            .finish()
    }
}

impl<T> PartialEq for TabWithId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
