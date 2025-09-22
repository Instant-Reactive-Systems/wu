mod focus_trap;
mod errors;
mod states;
mod theme;
mod text;
mod marked;
mod locatable_view_fn;
pub use focus_trap::*;
pub use errors::{error, errors, Errors, ReactiveErrors, ShowError};
pub use states::*;
pub use theme::*;
pub use text::Text;
pub use marked::Marked;
pub use locatable_view_fn::{LocatableViewFn, LocatableViewFnWithArgs};
use leptos::prelude::*;
use leptos_router::NavigateOptions;

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

/// A signal to check whether the current page is in a given set.
///
/// # Note
/// You can specify a wildcard at the end of the route in order to match a parametered route.
///
/// # Safety
/// Needs to be called in a owner context since it relies on `Location`.
pub fn is_route_in_set(iter: impl IntoIterator<Item = &'static str>) -> Memo<bool> {
	let location = leptos_router::hooks::use_location();
	let routes = Vec::from_iter(iter);
	Memo::new(move |_| {
		let path = location.pathname.get();
		routes
			.iter()
			.find(|route| match route.ends_with("*") {
				true => path.starts_with(route.strip_suffix("*").unwrap()),
				false => **route == path.strip_suffix("/").unwrap_or(&path),
			})
			.is_some()
	})
}

/// Wraps some functionality in an action.
pub fn actionize<F, Fu, I, R, T>(ctx: T, f: F) -> (Action<I, bool>, ReactiveErrors)
where
	F: Send + Sync + Fn(T, I) -> Fu + 'static,
	I: Send + Sync + Clone + 'static,
	R: Send + Sync + 'static,
	T: Send + Sync + Clone + 'static,
	Fu: std::future::Future<Output = Result<R, Errors>> + 'static,
{
	let errors = ReactiveErrors::default();
	let f = Callback::new(move |i: I| f(ctx.clone(), i));
	let action = Action::new(move |i: &I| {
		let i = i.clone();
		send_wrapper::SendWrapper::new(async move {
			match f.run(i).await {
				Ok(_) => true,
				Err(err) => {
					errors.replace(err.into());
					false
				},
			}
		})
	});

	(action, errors)
}

/// Wraps some functionality in an action, with a user-provided `ReactiveErrors`.
pub fn actionize_with_custom_error_sink<F, Fu, I, R, T>(errors: ReactiveErrors, ctx: T, f: F) -> Action<I, bool>
where
	F: Send + Sync + Fn(T, I) -> Fu + 'static,
	I: Send + Sync + Clone + 'static,
	R: Send + Sync + 'static,
	T: Send + Sync + Clone + 'static,
	Fu: std::future::Future<Output = Result<R, Errors>> + 'static,
{
	let f = Callback::new(move |i: I| f(ctx.clone(), i));
	let action = Action::new(move |i: &I| {
		let i = i.clone();
		send_wrapper::SendWrapper::new(async move {
			match f.run(i).await {
				Ok(_) => true,
				Err(err) => {
					errors.replace(err.into());
					false
				},
			}
		})
	});

	action
}

/// All possible positions inside of a box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
	TopLeft,
	Top,
	TopRight,
	Right,
	BottomRight,
	Bottom,
	BottomLeft,
	Left,
}
