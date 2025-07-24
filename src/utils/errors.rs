use leptos::prelude::*;

/// A typedef over a [`std::collections::HashMap`].
pub type Errors = std::collections::HashMap<std::borrow::Cow<'static, str>, ArcSignal<String>>;

/// A utility signal type that provides a by-name access to errors that occur in a request.
#[derive(Clone, Copy)]
pub struct ReactiveErrors {
	errors: RwSignal<Errors>,
}

impl ReactiveErrors {
	/// Creates a [`ReactiveErrors`] from [`Errors`].
	pub fn new(errors: Errors) -> Self {
		Self { errors: RwSignal::new(errors) }
	}

	/// Gets the error with the specified name, if it exists.
	///
	/// # Note
	/// Call in a signal for reactivity.
	pub fn get(&self, name: impl Into<std::borrow::Cow<'static, str>>) -> Option<ArcSignal<String>> {
		self.errors.read().get(&name.into()).cloned()
	}

	/// Replaces this with another [`Errors`].
	pub fn replace(&self, other: Errors) {
		self.errors.set(other);
	}
}

impl Default for ReactiveErrors {
	fn default() -> Self {
		Self { errors: RwSignal::default() }
	}
}

/// Creates a [`Errors`] with a single error.
pub fn error(key: String, value: ArcSignal<String>) -> Errors {
	let mut errors = Errors::default();
	errors.insert(key.into(), value);
	errors
}

/// Shows an error as an injected message.
#[component]
pub fn ShowError(errors: ReactiveErrors, #[prop(into)] id: std::borrow::Cow<'static, str>) -> impl IntoView {
	view! {
		{move || errors.get(id.clone()).map(move |err| view! {
			<div class="input-error border rounded-md py-1 px-2">
				{err}
			</div>
		})}
	}
}
