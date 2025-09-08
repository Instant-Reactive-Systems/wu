use leptos::prelude::*;

use crate::utils::{ShowError, Text};

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
	error_id: Text,
	/// Specifies the `type` attribute on the element.
	#[prop(optional, into)]
	r#type: Text,
	/// Specifies the `placeholder` attribute on the element.
	#[prop(optional, into)]
	placeholder: Text,
	/// Specifies the `required` attribute on the element.
	#[prop(optional, into)]
	required: Signal<bool>,
	/// Specifies the default 'class' attribute for all modals.
	#[prop(optional, into)]
	class: Text,
) -> impl IntoView {
	// vars
	let error = Memo::new(move |_| errors.get(error_id.get()));

	view! {
		<div class="vertical gap-1">
			// Input field
			<input
				type=r#type
				bind:value=value
				class=move || format!("{} {}", class.get(), error.get().map(move |_| "input-error").unwrap_or(""))
				placeholder=placeholder
				required=move || required.get()
				on:input=move |_| {
					errors.remove(error_id.get());
					errors.remove("default");
				}
			/>
			// Error description
			<ShowError errors error_id />
		</div>
	}
}

/// A wrapper around a `<input>` with a `String` value that handles reactive
/// interactivity automatically with styling designed to mimic a OTP code input.
///
/// # Example
/// ```rust,ignore
/// let code = create_rw_signal(String::default());
/// <InputCode
///     value=code
///     code_length=6
///     code_field_size=50
///     code_field_thickness=1
/// />
/// ```
#[component]
pub fn InputCode(
	/// Signal used for getting/setting the value.
	#[prop(into)]
	value: RwSignal<String>,
	/// Errors of the form.
	errors: crate::ReactiveErrors,
	/// Error ID of the field.
	#[prop(into)]
	error_id: Text,
	/// How long is the code.
	#[prop(into)]
	code_length: i32,
	/// How large is one field input of the code.
	#[prop(into)]
	field_size: i32,
	/// How thick is one field input of the code.
	#[prop(into)]
	field_thickness: i32,
	/// Specifies the `placeholder` attribute on the element.
	#[prop(optional, into)]
	placeholder: Text,
	/// Specifies the `required` attribute on the element.
	#[prop(optional, into)]
	required: Signal<bool>,
	/// Specifies the default 'class' attribute for all modals.
	#[prop(optional, into)]
	class: Text,
) -> impl IntoView {
	// vars
	let half_field_size = field_size / 2;
	let total_field_size = code_length * field_size;

	view! {
		<div class="overflow-hidden" style=format!("max-width: {total_field_size}px")> // prevents scroll-past-last-character behaviour
			<div class="sticky left-0"> // necessary because it hard-fixes the input field not to scroll
				<input
					type="text"
					bind:value=value
					maxlength=code_length
					style=format!("
						--tw-inset-ring-shadow: 0;\
						font-family: monospace;\
						padding-left: calc({half_field_size}px - (1ch / 2));\
						letter-spacing: calc({field_size}px - 1ch);\
						border-width: 0;\
						background-color: transparent;\
						filter: none;\
						overflow: hidden;\
						background-image: linear-gradient(to right, transparent 0%, transparent 15%, currentColor 15%, currentColor 85%, transparent 85%, transparent 0%);\
						background-position: bottom left;\
						background-size: {field_size}px {field_thickness}px;\
						background-repeat: repeat-x;\
						width: calc({total_field_size}px + {field_size}px);\
						min-width: calc({total_field_size}px + {field_size}px);\
						max-width: calc({total_field_size}px + {field_size}px);\
						outline: none;\
					")
					class=move || format!("selection:bg-transparent {class}")
					placeholder=placeholder
					required=move || required.get()
					on:input=move |_| {
						errors.remove(error_id.get());
						errors.remove("default");
					}
				/>
			</div>
		</div>
	}
}
