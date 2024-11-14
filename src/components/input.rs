use leptos::*;

/// A wrapper around a `<input>` with a `String` value that handles reactive
/// interactivity automatically.
///
/// # Example
/// ```rust,ignore
/// let name = create_rw_signal(String::default());
/// <Input attr:type="password" value=name />
/// ```
#[component]
pub fn Input(
	/// Signal used for getting/setting the value.
	#[prop(into)]
	value: RwSignal<String>,
	/// Specifies the default 'class' attribute for all modals.
	#[prop(default = "".into(), into)]
	class: TextProp,
	/// List of attributes to put on the top-level of the component.
	#[prop(attrs)]
	attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
	view! {
		<input
			{..attrs}
			on:input=move |ev| value.set(event_target_value(&ev))
			prop:value=value
			class=class
		/>
	}
}
