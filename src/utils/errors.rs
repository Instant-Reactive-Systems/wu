use leptos::prelude::*;

use crate::utils::Text;

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

	/// Gets all the errors as a `HashMap`.
	///
	/// # Note
	/// Call in a signal for reactivity.
	pub fn get_all(&self) -> Errors {
		self.errors.get()
	}

	/// Gets the error with the specified name, if it exists.
	///
	/// # Note
	/// Call in a signal for reactivity.
	pub fn get(&self, name: impl Into<std::borrow::Cow<'static, str>>) -> Option<ArcSignal<String>> {
		self.errors.read().get(&name.into()).cloned()
	}

	/// Gets the error with the specified name, if it exists.
	pub fn get_untracked(&self, name: impl Into<std::borrow::Cow<'static, str>>) -> Option<ArcSignal<String>> {
		self.errors.read_untracked().get(&name.into()).cloned()
	}

	/// Removes an error from the list.
	pub fn remove(&self, name: impl Into<std::borrow::Cow<'static, str>>) {
		_ = self.errors.write().remove(&name.into())
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

/// Creates a [`Errors`] with an iterator of errors.
pub fn errors(iter: impl IntoIterator<Item = (std::borrow::Cow<'static, str>, ArcSignal<String>)>) -> Errors {
	Errors::from_iter(iter)
}

/// Shows an error as an injected message.
#[component]
pub fn ShowError(errors: ReactiveErrors, #[prop(into)] error_id: Text) -> impl IntoView {
	// vars
	let error = Memo::new(move |_| errors.get(error_id.get()));

	move || {
		error.get().map(move |err| {
			view! {
				<div class="horizontal gap-1 text-sm">
					<span class="flex-none icon i-o-exclamation-circle icon-error-500 size-5" />
					<span class="grow text-error-500">{err}</span>
				</div>
			}
		})
	}
}
