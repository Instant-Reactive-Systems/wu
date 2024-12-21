mod focus_trap;
mod errors;
mod show;
pub use focus_trap::*;
pub use errors::{ShowError, Errors, ReactiveErrors};
pub use show::ShowOption;
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
