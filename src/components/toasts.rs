use std::time::Duration;

use leptos::{leptos_dom::helpers::TimeoutHandle, prelude::*};

use crate::utils::{Text, Position};

/// All possible toast levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
	Info,
	Warn,
	Error,
}

/// A message to display for a set amount of time.
#[derive(Clone)]
pub struct Toast {
	/// Toast view.
	view: ViewFn,
	/// Duration of the close timeout.
	timeout: Duration,
	/// Is the toast dismissable?
	dismissable: bool,
}

impl Toast {
	const DEFAULT_TOAST_TIMEOUT: Duration = Duration::from_secs(3);

	/// Creates a toast from a view with default options.
	pub fn from_view(level: ToastLevel, view_fn: impl Into<ViewFn>) -> Self {
		Self::from_view_with(level, view_fn, Self::DEFAULT_TOAST_TIMEOUT, true)
	}

	/// Creates a toast from a view with custom options.
	pub fn from_view_with(level: ToastLevel, view_fn: impl Into<ViewFn>, timeout: Duration, dismissable: bool) -> Self {
		let (icon_class, color_class) = match level {
			ToastLevel::Info => ("i-o-info-circle", ""),
			ToastLevel::Warn => ("i-o-exclamation-circle", "icon-warning-700 dark:icon-warning-500"),
			ToastLevel::Error => ("i-o-exclamation-triangle", "icon-error-700 dark:icon-error-500"),
		};
		let view_fn = view_fn.into();

		Self {
			view: ViewFn::from(move || {
				view! {
					<div class="horizontal vcenter gap-4">
						<span class=format!("flex-none icon {icon_class} {color_class}") />
						<div class="grow overflow-hidden">
							{view_fn.run()}
						</div>
					</div>
				}
			}),
			timeout,
			dismissable,
		}
	}

	/// Creates a toast from text with default options.
	pub fn from_text(level: ToastLevel, text: impl Into<Text>) -> Self {
		Self::from_text_with(level, text, Self::DEFAULT_TOAST_TIMEOUT, true)
	}

	/// Creates a toast from text with custom options.
	pub fn from_text_with(level: ToastLevel, text: impl Into<Text>, timeout: Duration, dismissable: bool) -> Self {
		let (icon_class, color_class) = match level {
			ToastLevel::Info => ("i-o-info-circle", ""),
			ToastLevel::Warn => ("i-o-exclamation-circle", "icon-warning-700 dark:icon-warning-500"),
			ToastLevel::Error => ("i-o-exclamation-triangle", "icon-error-700 dark:icon-error-500"),
		};
		let text = text.into();

		Self {
			view: ViewFn::from(move || {
				view! {
					<div class="horizontal vcenter gap-4">
						<span class=format!("flex-none icon {icon_class} {color_class}") />
						<p class="grow overflow-hidden">{text}</p>
					</div>
				}
			}),
			timeout,
			dismissable,
		}
	}
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToastMarker<M>(std::marker::PhantomData<M>);

crate::generate_marker_type!(
	#[doc(hidden)]
	PushToastMarker
);

pub type PushToast<M> = crate::utils::Marked<ToastMarker<(M, PushToastMarker)>, Callback<Toast, ToastHandle>>;

/// A type that allows cancelling a toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToastHandle {
	toasts_ref: RwSignal<Vec<ToastWithId>>,
	toast_id: u64,
	timeout_handle: TimeoutHandle,
}

impl ToastHandle {
	/// Cancels the toast.
	pub fn cancel(&self) {
		self.timeout_handle.clear();
		self.toasts_ref.write().retain(|toast: &ToastWithId| toast.id != self.toast_id);
	}
}

/// Simple notifications utilizing a dynamic queue system.
#[component]
pub fn ToastHook<M>(
	#[prop(optional)] _phant: std::marker::PhantomData<M>,
	/// Toast position in the overlay.
	#[prop(default = Position::TopRight, into)]
	position: Position,
	/// Toast class.
	#[prop(optional, into)]
	class: Text,
	/// Toast container class.
	#[prop(optional, into)]
	container_class: Text,
	/// Children of the component.
	children: Children,
) -> impl IntoView
where
	M: Send + Sync + 'static,
{
	// toast creation
	let toast_id = RwSignal::new(0u64);
	let toasts = RwSignal::new(Vec::default());
	provide_context(PushToast::<M>::new(Callback::new(move |toast: Toast| {
		let id = toast_id.get_untracked();
		let timeout_handle = set_timeout_with_handle(move || toasts.write().retain(|toast: &ToastWithId| toast.id != id), toast.timeout);
		let Ok(timeout_handle) = timeout_handle else {
			unreachable!("should be able to construct a timeout handle always");
		};
		let toast_handle = ToastHandle {
			toasts_ref: toasts,
			toast_id: id,
			timeout_handle,
		};

		toasts.update(move |toasts| toasts.push(ToastWithId { id, toast, timeout_handle }));
		toast_id.update(|n| *n = n.overflowing_add(1).0);
		toast_handle
	})));

	let (position_class, radius_class) = match position {
		Position::TopLeft => ("overlay-tl", "last:rounded-br-(--wu-dynamic-toast-border-radius)"),
		Position::Top => ("overlay-t", "last:rounded-b-(--wu-dynamic-toast-border-radius)"),
		Position::TopRight => ("overlay-tr", "last:rounded-bl-(--wu-dynamic-toast-border-radius)"),
		Position::Right => ("overlay-r", "first:rounded-tl-(--wu-dynamic-toast-border-radius) last:rounded-bl-(--wu-dynamic-toast-border-radius)"),
		Position::BottomRight => ("overlay-br", "first:rounded-tl-(--wu-dynamic-toast-border-radius)"),
		Position::Bottom => ("overlay-b", "first:rounded-t-(--wu-dynamic-toast-border-radius)"),
		Position::BottomLeft => ("overlay-bl", "first:rounded-tr-(--wu-dynamic-toast-border-radius)"),
		Position::Left => ("overlay-l", "first:rounded-tr-(--wu-dynamic-toast-border-radius) last:rounded-br-(--wu-dynamic-toast-border-radius)"),
	};

	view! {
		{children()}
		<wu-toasts class="overlay overlay-container">
			<ul class=format!("overlay w-fit {position_class}")>
				<For
					each=move || toasts.get()
					key=move |toast| toast.id
					children=move |toast| {
						let toast_handle = ToastHandle {
							toasts_ref: toasts,
							toast_id: toast.id,
							timeout_handle: toast.timeout_handle,
						};

						view! {
							<wu-toast
								style="\
									background-color: var(--wu-dynamic-toast-bg-color);\
									border-color: var(--wu-dynamic-toast-border-color);\
									border-width: var(--wu-dynamic-toast-border-width);\
									box-shadow: var(--wu-dynamic-toast-shadow);\
								"
								class=format!("horizontal vcenter gap-4 max-w-lvw min-h-(--wu-dynamic-toast-min-height) p-(--wu-dynamic-toast-padding) {radius_class}")
							>
								// content
								<div class=move || format!("grow {class}")>
									{toast.view.run()}
								</div>
								// close
								{toast.dismissable.then(move || view! {
									<button
										class="flex-none btn-icon autohighlight size-6"
										on:click=move |_| toast_handle.cancel()
									>
										<span class="icon i-o-x-mark"/>
									</button>
								})}
							</wu-toast>
						}
					}
				/>
			</ul>
		</wu-toasts>
	}
}

#[derive(Clone)]
struct ToastWithId {
	pub id: u64,
	toast: Toast,
	timeout_handle: TimeoutHandle,
}

impl PartialEq for ToastWithId {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl std::ops::Deref for ToastWithId {
	type Target = Toast;

	fn deref(&self) -> &Self::Target {
		&self.toast
	}
}

impl std::ops::DerefMut for ToastWithId {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.toast
	}
}
