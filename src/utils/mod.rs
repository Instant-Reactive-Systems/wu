mod focus_trap;
mod errors;
mod states;
mod theme;
pub use focus_trap::*;
pub use errors::{error, Errors, ReactiveErrors, ShowError};
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

/// A utility signal type that provides easy manipulation of a string.
///
/// Aims to replace `TextProp`.
#[derive(Clone, Copy)]
pub struct Text(Signal<std::borrow::Cow<'static, str>>);

impl Text {
	/// Gets the underlying string.
	pub fn get(&self) -> std::borrow::Cow<'static, str> {
		self.0.get()
	}
}

impl std::convert::From<String> for Text {
	fn from(value: String) -> Self {
		Self(Signal::stored(value.into()))
	}
}

impl std::convert::From<&'static str> for Text {
	fn from(value: &'static str) -> Self {
		Self(Signal::stored(value.into()))
	}
}

impl std::convert::From<std::borrow::Cow<'static, str>> for Text {
	fn from(value: std::borrow::Cow<'static, str>) -> Self {
		Self(Signal::stored(value))
	}
}

impl<S> std::convert::From<Signal<S>> for Text
where
	S: std::convert::Into<std::borrow::Cow<'static, str>> + Clone + Send + Sync + 'static,
{
	fn from(value: Signal<S>) -> Self {
		Self(Signal::derive(move || value.get().into()))
	}
}

impl<F, S> std::convert::From<F> for Text
where
	F: Fn() -> S + Send + Sync + 'static,
	S: Into<std::borrow::Cow<'static, str>>,
{
	fn from(value: F) -> Self {
		Self(Signal::derive(move || value().into()))
	}
}

impl Default for Text {
	fn default() -> Self {
		Self(Signal::stored("".into()))
	}
}

impl IntoRender for Text {
	type Output = Signal<std::borrow::Cow<'static, str>>;

	fn into_render(self) -> Self::Output {
		self.0
	}
}

impl IntoAttributeValue for Text {
	type Output = Signal<String>;

	fn into_attribute_value(self) -> Self::Output {
		Signal::derive(move || self.0.get().into_owned())
	}
}

impl std::fmt::Display for Text {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0.get())
	}
}

impl std::fmt::Debug for Text {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0.get())
	}
}

/// Prints a message on the initial effect run and the cleanup.
///
/// Useful for debugging.
#[track_caller]
pub fn print_on_enter_exit(msg: impl ToString) {
	print_on_enter_exit_impl(msg.to_string());
}

#[track_caller]
fn print_on_enter_exit_impl(msg: String) {
	let location = std::panic::Location::caller();
	Effect::new({
		let msg = msg.clone();
		move || log::info!("{location} | on first effect: {msg}")
	});
	on_cleanup(move || log::info!("{location} | on cleanup: {msg}"));
}

/// A utility for comparing whether an option changed from None to Some and vice versa, without
/// checking the data underneath.
pub fn nested_option_memo_compare_fn<T>(old: Option<&Option<T>>, new: Option<&Option<T>>) -> bool {
	match (old, new) {
		(Some(..), None) => true,
		(None, Some(..)) => true,
		(old, new) => old.flatten().is_some() != new.flatten().is_some(),
	}
}

/// A [`ViewFn`] that also knows the location of where it originated in order to use it in a [`Memo`].
pub struct LocatableViewFn {
	location: &'static std::panic::Location<'static>,
	view: ViewFn,
}

impl LocatableViewFn {
	/// Creates a new view.
	#[track_caller]
	pub fn new(view: impl Into<ViewFn>) -> Self {
		Self {
			location: std::panic::Location::caller(),
			view: view.into(),
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
	fn from(value: F) -> Self {
		Self {
			location: std::panic::Location::caller(),
			view: ViewFn::from(value),
		}
	}
}

impl Default for LocatableViewFn {
	#[track_caller]
	fn default() -> Self {
		Self {
			location: std::panic::Location::caller(),
			view: ViewFn::default(),
		}
	}
}

/// A utility for pushing messages downstream.
pub struct MsgChannel<M, Msg>
where
	M: Send + Sync + 'static,
	Msg: Clone + Send + Sync + 'static,
{
	msgs: RwSignal<Vec<Msg>>,
	_phant: std::marker::PhantomData<M>,
}

impl<M, Msg> MsgChannel<M, Msg>
where
	M: Send + Sync + 'static,
	Msg: Clone + Send + Sync + 'static,
{
	/// Create a new message channel.
	pub fn new() -> Self {
		Self::default()
	}

	/// Pushes a new message.
	pub fn push(&self, msg: Msg) {
		self.msgs.write().push(msg);
	}

	/// Consumes the message queue.
	///
	/// # Note
	/// To be called in a reactive context for reactivity.
	pub fn consume(&self) -> Vec<Msg> {
		let msgs = self.msgs.get();
		self.msgs.write_untracked().clear();
		msgs
	}
}

impl<M, Msg> Default for MsgChannel<M, Msg>
where
	M: Send + Sync + 'static,
	Msg: Clone + Send + Sync + 'static,
{
	fn default() -> Self {
		Self {
			msgs: Default::default(),
			_phant: Default::default(),
		}
	}
}

impl<M, Msg> Clone for MsgChannel<M, Msg>
where
	M: Send + Sync + 'static,
	Msg: Clone + Send + Sync + 'static,
{
	fn clone(&self) -> Self {
		Self {
			msgs: self.msgs.clone(),
			_phant: Default::default(),
		}
	}
}

impl<M, Msg> Copy for MsgChannel<M, Msg>
where
	M: Send + Sync + 'static,
	Msg: Clone + Send + Sync + 'static,
{
}

/// Component flavor.
///
/// Designed to select the component to be rendered as an icon or by text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flavor {
	Icon,
	Text,
}
