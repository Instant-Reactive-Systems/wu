use std::sync::Arc;

use leptos::prelude::*;

/// A [`ViewFn`] that also knows the location of where it originated in order to use it in a [`Memo`].
pub struct LocatableViewFn {
	location: &'static std::panic::Location<'static>,
	view: ViewFn,
	// used to mitigate using Option in components
	pub(crate) is_default: bool,
}

impl LocatableViewFn {
	/// Creates a new view.
	#[track_caller]
	pub fn new(view: impl Into<ViewFn>) -> Self {
		Self {
			location: std::panic::Location::caller(),
			view: view.into(),
			is_default: false,
		}
	}

	/// Execute the wrapped function.
	pub fn run(&self) -> AnyView {
		self.view.run()
	}

	/// Removes the wrapper and returns the inner [`ViewFn`].
	pub fn into_view_fn(self) -> ViewFn {
		self.view
	}
}

impl From<ViewFn> for LocatableViewFn {
	#[track_caller]
	fn from(vw: ViewFn) -> Self {
		Self::new(vw)
	}
}

impl std::fmt::Debug for LocatableViewFn {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Location: {:?}", self.location)
	}
}

impl Clone for LocatableViewFn {
	fn clone(&self) -> Self {
		Self {
			location: self.location,
			view: self.view.clone(),
			is_default: self.is_default,
		}
	}
}

impl PartialEq for LocatableViewFn {
	fn eq(&self, other: &Self) -> bool {
		self.location.eq(other.location)
	}
}

impl Eq for LocatableViewFn {}

impl std::hash::Hash for LocatableViewFn {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.location.hash(state)
	}
}

impl<F, C> From<F> for LocatableViewFn
where
	F: Fn() -> C + Send + Sync + 'static,
	C: RenderHtml + Send + 'static,
{
	#[track_caller]
	fn from(value: F) -> Self {
		Self {
			location: std::panic::Location::caller(),
			view: ViewFn::from(value),
			is_default: false,
		}
	}
}

impl Default for LocatableViewFn {
	#[track_caller]
	fn default() -> Self {
		Self {
			location: std::panic::Location::caller(),
			view: ViewFn::default(),
			is_default: true,
		}
	}
}

/// A [`ViewFn`] that also knows the location of where it originated in order to
/// use it in a [`Memo`], but this time with custom arguments.
pub struct LocatableViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
{
	location: &'static std::panic::Location<'static>,
	view: std::sync::Arc<dyn Fn(T) -> AnyView + Send + Sync + 'static>,
	// used to mitigate using Option in components
	pub(crate) is_default: bool,
}

impl<T> LocatableViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
{
	/// Execute the wrapped function.
	pub fn run(&self, value: T) -> AnyView {
		(self.view)(value)
	}
}

impl<T> std::fmt::Debug for LocatableViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Location: {:?}", self.location)
	}
}

impl<T> Clone for LocatableViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
{
	fn clone(&self) -> Self {
		Self {
			location: self.location,
			view: self.view.clone(),
			is_default: self.is_default,
		}
	}
}

impl<T> PartialEq for LocatableViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
{
	fn eq(&self, other: &Self) -> bool {
		self.location.eq(other.location)
	}
}

impl<T> Eq for LocatableViewFnWithArgs<T> where T: Send + Sync + 'static {}

impl<T> std::hash::Hash for LocatableViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.location.hash(state)
	}
}

impl<T, F, C> From<F> for LocatableViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
	F: Fn(T) -> C + Send + Sync + 'static,
	C: RenderHtml + Send + 'static,
{
	#[track_caller]
	fn from(f: F) -> Self {
		Self {
			location: std::panic::Location::caller(),
			view: Arc::new(move |value| f(value).into_any()),
			is_default: false,
		}
	}
}

impl<T> Default for LocatableViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
{
	#[track_caller]
	fn default() -> Self {
		Self {
			location: std::panic::Location::caller(),
			view: std::sync::Arc::new(|_| ().into_any()),
			is_default: true,
		}
	}
}
