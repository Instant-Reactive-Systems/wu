mod focus_trap;
mod errors;
mod states;
mod theme;
pub use focus_trap::*;
pub use errors::{Errors, ReactiveErrors, ShowError, error};
use leptos_router::NavigateOptions;
pub use states::*;
pub use theme::*;
use leptos::prelude::*;

/// Wrapper around a `T` that allows specifying a generic to denote different types.
pub struct Marked<M, T> {
	data: T,
	_phant: std::marker::PhantomData<M>,
}

impl<M, T> Marked<M, T> {
	pub fn new(data: T) -> Self {
		Self { data, _phant: Default::default() }
	}
}

impl<M, T: std::fmt::Debug> std::fmt::Debug for Marked<M, T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.data.fmt(f)
	}
}

impl<M, T: Clone> Clone for Marked<M, T> {
	fn clone(&self) -> Self {
		Self {
			data: self.data.clone(),
			_phant: Default::default(),
		}
	}
}

impl<M, T: Copy> Copy for Marked<M, T> {}

impl<M, T: PartialEq> PartialEq for Marked<M, T> {
	fn eq(&self, other: &Self) -> bool {
		self.data.eq(&other.data)
	}
}

impl<M, T: Eq> Eq for Marked<M, T> {}

impl<M, T> std::ops::Deref for Marked<M, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<M, T> std::ops::DerefMut for Marked<M, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

/// Generates a unit struct that acts as a marker type.
#[macro_export]
macro_rules! generate_marker_type {
    ($(#[$outer:meta])* $name:ident) => {
        $(#[$outer])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name;
    };
}

/// Creates a copyable version of `use_navigate()`.
pub fn use_copy_navigate() -> Callback<(String, NavigateOptions)> {
	let navigate = leptos_router::hooks::use_navigate();
	Callback::new(move |(url, opts): (String, NavigateOptions)| navigate(&url, opts))
}

/// New-type wrapper for a function that returns a view with `From` and `Default` traits implemented
/// to enable optional props in for example `<Show>` and `<Suspense>`.
#[derive(Clone)]
pub struct ViewFnWithArgs<T>(std::sync::Arc<dyn Fn(T) -> AnyView + Send + Sync + 'static>)
where
	T: Send + Sync + 'static;

impl<T> Default for ViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
{
	fn default() -> Self {
		Self(std::sync::Arc::new(|_t: T| ().into_any()))
	}
}

impl<T, F, C> From<F> for ViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
	F: Fn(T) -> C + Send + Sync + 'static,
	C: RenderHtml + Send + 'static,
{
	fn from(value: F) -> Self {
		Self(std::sync::Arc::new(move |t: T| value(t).into_any()))
	}
}

impl<T> ViewFnWithArgs<T>
where
	T: Send + Sync + 'static,
{
	/// Execute the wrapped function
	pub fn run(&self, t: T) -> AnyView {
		(self.0)(t)
	}
}
