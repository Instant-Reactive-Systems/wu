use leptos::{prelude::*, text_prop::TextProp};
use tailwind_fuse::*;

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
	/// Specifies the `type` attribute on the element.
	#[prop(default = "".into(), into)]
	r#type: TextProp,
	/// Specifies the `placeholder` attribute on the element.
	#[prop(default = "".into(), into)]
	placeholder: TextProp,
	/// Specifies the `required` attribute on the element.
	#[prop(optional, into)]
	required: Signal<bool>,
	/// Specifies the default 'class' attribute for all modals.
	#[prop(default = "".into(), into)]
	class: TextProp,
) -> impl IntoView {
	// TODO: wait for AttributeInterceptor to pass it to the inner input
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
			<input
				type=move || r#type.get()
				bind:value=value
				class={
					let error_id = error_id.clone();
					move || tw_merge!(class.get(), errors.get(error_id.clone()).map(move |_| "input-error"))
				}
				placeholder=move || placeholder.get()
				required=move || required.get()
			/>
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
	/// How long is the code.
	#[prop(into)]
	code_length: i32,
	/// How huge is one field input of the code.
	#[prop(into)]
	field_size: i32,
	/// How huge is one field input of the code.
	#[prop(into)]
	field_thickness: i32,
	/// Specifies the `placeholder` attribute on the element.
	#[prop(default = "".into(), into)]
	placeholder: TextProp,
	/// Specifies the `required` attribute on the element.
	#[prop(optional, into)]
	required: Signal<bool>,
	/// Specifies the default 'class' attribute for all modals.
	#[prop(default = "".into(), into)]
	class: TextProp,
) -> impl IntoView {
	// vars
	let half_field_size = field_size / 2;
	let total_field_size = code_length * field_size;

	// TODO: wait for AttributeInterceptor to pass it to the inner input
	view! {
		<div class="overflow-hidden" style=format!("max-width: {total_field_size}px")> // prevents scroll-past-last-character behaviour
			<div class="sticky left-0"> // necessary because it hard-fixes the input field not to scroll
				<input
					type="text"
					bind:value=value
					maxlength=code_length
					inputmode="numeric"
					style=format!("
						--tw-ring-inset: 0;
						font-family: monospace;
						padding-left: calc({half_field_size}px - (1ch / 2));
						letter-spacing: calc({field_size}px - 1ch);
						border-width: 0;
						background-color: transparent;
						filter: none;
						overflow: hidden;
						background-image: linear-gradient(to right, transparent 0%, transparent 15%, currentColor 15%, currentColor 85%, transparent 85%, transparent 0%);
						background-position: bottom left;
						background-size: {field_size}px {field_thickness}px;
						background-repeat: repeat-x;
						width: calc({total_field_size}px + {field_size}px);
						min-width: calc({total_field_size}px + {field_size}px);
						max-width: calc({total_field_size}px + {field_size}px);
						outline: none;
					")
					class=move || class.get()
				/>
			</div>
		</div>
	}
}
