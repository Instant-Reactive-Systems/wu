use leptos::*;

/// A wrapper around a `<select>` and `<option>` that automatically
/// interactivity automatically.
///
/// # Example
/// ```rust,ignore
/// let num = create_rw_signal(0u32);
/// <Dropdown
///     attr:id="fruits"
///     value=num
///     items=[0, 2, 4]
///     class="surface-3"
///  />
/// ```
#[component]
pub fn Dropdown<T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Display + Clone + 'static>(
	/// Signal used for getting/setting the value.
	#[prop(into)]
	value: RwSignal<T>,
	/// Possible items to choose from.
	#[prop(into)]
	items: Vec<T>,
	/// Specifies the default 'class' attribute for all modals.
	#[prop(default = "".into(), into)]
	class: TextProp,
	/// List of attributes to put on the top-level of the component.
	#[prop(attrs)]
	attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
	view! {
		<select
			{..attrs}
			class=class
			on:change=move |ev| {
				let new_value: T = match serde_json::from_str(&event_target_value(&ev)) {
					Ok(val) => val,
					Err(err) => {
						log::error!("dropdown value is invalid and cannot be parsed\nerror: {err}");
						return;
					},
				};
				value.set(new_value);
			}
			prop:value=move || serde_json::to_string(&value.get()).expect("should be valid")
		>
			{
				items
					.into_iter()
					.map(|item| view! {
						<option value=serde_json::to_string(&item).expect("should be valid, check your input data")>
							{item.to_string()}
						</option>
					})
					.collect::<Vec<_>>()
			}
		</select>
	}
}
