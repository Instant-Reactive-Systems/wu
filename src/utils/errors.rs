use leptos::*;

/// A typedef over a [`std::collections::HashMap`].
pub type Errors = std::collections::HashMap<std::borrow::Cow<'static, str>, String>;

/// A utility signal type that provides a by-name access to errors that occur in a request.
#[derive(Clone, Copy)]
pub struct ReactiveErrors {
	errors: RwSignal<Errors>,
}

impl ReactiveErrors {
	/// Creates an empty [`Errors`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Gets the error with the specified name, if it exists.
	///
	/// # Note
	/// Call in a signal for reactivity.
	pub fn get(&self, name: impl Into<std::borrow::Cow<'static, str>>) -> Option<String> {
		self.errors.with(move |errors| errors.get(&name.into()).cloned())
	}

	/// Replaces this with another [`Errors`].
	pub fn replace(&self, other: Errors) {
		self.errors.set(other);
	}
}

impl Default for ReactiveErrors {
	fn default() -> Self {
		Self {
			errors: create_rw_signal(Default::default()),
		}
	}
}
