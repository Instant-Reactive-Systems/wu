use leptos::*;
use tailwind_fuse::*;

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

/// A wrapper around a `<input>` with a `String` value that handles reactive
/// interactivity automatically and displays an error if error occurs.
///
/// # Example
/// ```rust,ignore
/// let name = create_rw_signal(String::default());
/// <FallibleReactiveInput attr:type="password" value=name errors error_id="password" />
/// ```
#[component]
pub fn FallibleReactiveInput(
	/// Signal used for getting/setting the value.
	#[prop(into)]
	value: RwSignal<String>,
	/// Errors of the form.
	errors: crate::ReactiveErrors,
	/// Error ID of the field.
	#[prop(into)]
	error_id: std::borrow::Cow<'static, str>,
	/// Specifies the default 'class' attribute for all modals.
	#[prop(default = "".into(), into)]
	class: TextProp,
	/// List of attributes to put on the top-level of the component.
	#[prop(attrs)]
	attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
	view! {
		<div class="relative vertical">
			// if mobile
			<div class="flex desktop:hidden relative">
				<crate::ShowOption
					data={
						let error_id = error_id.clone();
						move || errors.get(error_id.clone())
					}
					let:err
				>
					<div
						class=
						"
							input-error border w-fit text-nowrap py-1 px-2 mb-2 rounded-md rounded-bl-none 
							before:content-[''] before:absolute before:bg-inherit before:border-inherit before:size-2 
							before:left-[2px] before:top-[calc(100%_-_12px)] before:rotate-45 before:border-r before:border-b 
						"
					>
						{err}
					</div>
				</crate::ShowOption>
			</div>

			// if >mobile
			<div class="hidden desktop:flex relative">
				<crate::ShowOption
					data={
						let error_id = error_id.clone();
						move || errors.get(error_id.clone())
					}
					let:err
				>
					<div
						class=
						"
							absolute end-0 left-[calc(100%_+_12px)] input-error border w-fit text-nowrap py-1 px-2 rounded-md rounded-tl-none 
							before:content-[''] before:absolute before:bg-inherit before:border-inherit before:size-2 
							before:left-[-5px] before:top-[1px] before:rotate-45 before:border-l before:border-b 
						"
					>
						{err}
					</div>
				</crate::ShowOption>
			</div>

			// all
			<Input
				{..attrs}
				value=value
				class={
					let error_id = error_id.clone();
					move || tw_merge!(class.get(), errors.get(error_id.clone()).map(move |_| "input-error"))
				}
			/>
		</div>
	}
}
