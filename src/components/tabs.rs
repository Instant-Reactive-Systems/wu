use leptos::{prelude::*, text_prop::TextProp};

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabMarker<M>(std::marker::PhantomData<M>);

crate::generate_marker_type!(
	#[doc(hidden)]
	AddTabMarker
);
crate::generate_marker_type!(
	#[doc(hidden)]
	RemoveTabMarker
);
crate::generate_marker_type!(
	#[doc(hidden)]
	ModifyTabMarker
);
crate::generate_marker_type!(
	#[doc(hidden)]
	RemoveTabsMarker
);
crate::generate_marker_type!(
	#[doc(hidden)]
	RemoveOtherTabsMarker
);
crate::generate_marker_type!(
	#[doc(hidden)]
	SwitchActiveTabMarker
);

pub type TabId = u64;
pub type TabSignal<T> = RwSignal<TabWithId<T>>;
pub type AddTab<M, T> = crate::utils::Marked<TabMarker<(M, AddTabMarker)>, Callback<T>>;
pub type RemoveTab<M> = crate::utils::Marked<TabMarker<(M, RemoveTabMarker)>, Callback<TabId>>;
pub type ModifyTab<M, T> = crate::utils::Marked<TabMarker<(M, ModifyTabMarker)>, Callback<(TabId, T)>>;
pub type RemoveTabs<M> = crate::utils::Marked<TabMarker<(M, RemoveTabsMarker)>, Callback<Vec<TabId>>>;
pub type RemoveOtherTabs<M> = crate::utils::Marked<TabMarker<(M, RemoveOtherTabsMarker)>, Callback<TabId>>;
pub type SwitchActiveTab<M> = crate::utils::Marked<TabMarker<(M, SwitchActiveTabMarker)>, Callback<TabId>>;

/// Use tabs to quickly switch between different views and pages.
#[component]
pub fn Tabs<M: Send + Sync + 'static, T: Send + Sync + 'static>(
	#[prop(optional)] _phant: std::marker::PhantomData<(M, T)>,
	/// Default list of tab contexts.
	#[prop(default = Vec::default(), into)]
	tabs: Vec<T>,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(default = "".into(), into)]
	class: TextProp,
	/// List class.
	#[prop(default = "".into(), into)]
	list_class: TextProp,
	/// List item slot.
	#[prop(into)]
	item: Callback<TabSignal<T>, AnyView>,
	/// Content slot.
	#[prop(into)]
	content: Callback<TabSignal<T>, AnyView>,
	/// List fallback.
	#[prop(optional, into)]
	list_fallback: ViewFn,
	/// Content fallback.
	#[prop(optional, into)]
	content_fallback: ViewFn,
) -> impl IntoView {
	let is_default_full = !tabs.is_empty();
	let (tab_id, set_tab_id) = signal::<TabId>(tabs.len() as TabId);
	let (tabs, set_tabs) = signal::<Vec<TabSignal<T>>>(
		tabs.into_iter()
			.enumerate()
			.map(|(id, tab)| TabWithId { id: id as u64, tab })
			.map(|tab| RwSignal::new(tab))
			.collect(),
	);
	let (active_tab_id, set_active_tab_id) = signal::<Option<TabId>>(is_default_full.then_some(0));
	let active_tab = Signal::derive(move || {
		active_tab_id
			.read()
			.and_then(move |active_tab_id| tabs.read().iter().find(|tab| tab.with(|tab| tab.id == active_tab_id)).cloned())
	});

	provide_context(AddTab::<M, T>::new(Callback::new(move |tab| {
		// add to tabs
		let id = tab_id.get_untracked();
		let tab = RwSignal::new(TabWithId { id, tab });
		set_tabs.update(move |tabs| tabs.push(tab));
		// autoincrement tab id
		set_tab_id.update(move |tab_id| *tab_id += 1);
		// assign active tab if not set
		if active_tab_id.get_untracked().is_none() {
			set_active_tab_id.update(move |active_tab_id| *active_tab_id = Some(0))
		}
	})));
	provide_context(RemoveTab::<M>::new(Callback::new(move |id| {
		set_tabs.update(move |tabs| tabs.retain(|tab| tab.with_untracked(|tab| tab.id != id)));
		set_active_tab_id.update(move |active_tab_id| *active_tab_id = tabs.with_untracked(|tabs| (!tabs.is_empty()).then_some(tabs[0].with_untracked(|tab| tab.id))));
	})));
	provide_context(ModifyTab::<M, T>::new(Callback::new(move |(id, new_tab)| {
		let new_tab = TabWithId { id, tab: new_tab };
		tabs.with(move |tabs| {
			let tab = tabs.iter().find(|tab| tab.with_untracked(|tab| tab.id == id));
			if let Some(tab) = tab {
				tab.update(move |tab| *tab = new_tab)
			}
		});
	})));
	provide_context(RemoveTabs::<M>::new(Callback::new(move |tab_ids: Vec<TabId>| {
		set_tabs.update(move |tabs| tabs.retain(|tab| !tab_ids.contains(&tab.with_untracked(|tab| tab.id))));
		set_active_tab_id.update(move |active_tab_id| *active_tab_id = tabs.with_untracked(|tabs| (!tabs.is_empty()).then_some(tabs[0].with_untracked(|tab| tab.id))));
	})));
	provide_context(RemoveOtherTabs::<M>::new(Callback::new(move |id| {
		set_tabs.update(move |tabs| tabs.retain(|tab| tab.with_untracked(|tab| tab.id == id)));
		set_active_tab_id.update(move |active_tab_id| *active_tab_id = tabs.with_untracked(|tabs| (!tabs.is_empty()).then_some(tabs[0].with_untracked(|tab| tab.id))));
	})));
	provide_context(SwitchActiveTab::<T>::new(Callback::new(move |id| {
		// find if the tab exists
		if tabs.with(move |tabs| !tabs.iter().any(|tab| tab.with_untracked(|tab| tab.id == id))) {
			log::error!("SwitchActiveTab: tab with id '{id}' does not exist");
		}
		// update the active tab id
		set_active_tab_id.update(move |active_tab_id| *active_tab_id = Some(id))
	})));

	view! {
		<wu-tabs class=move || class.get()>
			<ul class=move || list_class.get()>
				<Show
					when=move || !tabs.read().is_empty()
					fallback=list_fallback
				>
					<For
						each=move || tabs.get()
						key=move |tab| tab.read().id
						let:tab
					>
						{move || item.run(tab)}
					</For>
				</Show>
			</ul>
			<crate::ShowOption
				data=move || active_tab.get()
				fallback=content_fallback
				let:tab
			>
				{content.run(tab)}
			</crate::ShowOption>
		</wu-tabs>
	}
}

/// A wrapper around a Tab that holds its ID.
///
/// Used for keyed-for.
#[derive(Clone)]
pub struct TabWithId<T> {
	/// Tab's unique ID.
	pub id: u64,
	/// The tab itself.
	tab: T,
}

impl<T: std::fmt::Debug> std::fmt::Debug for TabWithId<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct(std::any::type_name::<Self>()).field("id", &self.id).field("tab", &self.tab).finish()
	}
}

impl<T> PartialEq for TabWithId<T> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl<T> std::ops::Deref for TabWithId<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.tab
	}
}

impl<T> std::ops::DerefMut for TabWithId<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.tab
	}
}
