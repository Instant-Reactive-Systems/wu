use std::time::Duration;

use leptos::{leptos_dom::helpers::TimeoutHandle, prelude::*};

use crate::utils::Text;

/// A toast message.
///
/// Can be either a simple text payload or a complex view.
#[derive(Clone)]
pub enum ToastMsg {
	/// Simple text payload.
	Text(ArcSignal<String>),
	/// Info message.
	Info(ArcSignal<String>),
	/// Warn message.
	Warn(ArcSignal<String>),
	/// Error message.
	Error(ArcSignal<String>),
	/// Complex view.
	View(ViewFn),
	/// Info message with an additional complex view.
	InfoView(ViewFn),
	/// Warn message with an additional complex view.
	WarnView(ViewFn),
	/// Error message with an additional complex view.
	ErrorView(ViewFn),
}

impl IntoRender for ToastMsg {
	type Output = AnyView;

	fn into_render(self) -> Self::Output {
		match self {
			ToastMsg::Text(text) => view! {
				<p class="overflow-hidden grow">{text}</p>
			}
			.into_any(),
			ToastMsg::Info(text) => view! {
				<div class="horizontal vcenter gap-4">
					<span class="icon i-o-info-circle"/>
					<p class="overflow-hidden grow">{text}</p>
				</div>
			}
			.into_any(),
			ToastMsg::Warn(text) => view! {
				<div class="horizontal vcenter gap-4">
					<span class="icon i-o-exclamation-circle icon-warning-500"/>
					<p class="overflow-hidden grow">{text}</p>
				</div>
			}
			.into_any(),
			ToastMsg::Error(text) => view! {
				<div class="horizontal vcenter gap-4">
					<span class="icon i-o-exclamation-triangle icon-error-500"/>
					<p class="overflow-hidden grow">{text}</p>
				</div>
			}
			.into_any(),
			ToastMsg::View(vw) => view! {
				<div class="overflow-hidden grow">{vw.run()}</div>
			}
			.into_any(),
			ToastMsg::InfoView(vw) => view! {
				<div class="horizontal vcenter gap-4">
					<span class="icon i-o-info-circle"/>
					<p class="overflow-hidden grow">{vw.run()}</p>
				</div>
			}
			.into_any(),
			ToastMsg::WarnView(vw) => view! {
				<div class="horizontal vcenter gap-4">
					<span class="icon i-o-exclamation-circle icon-warning-500"/>
					<p class="overflow-hidden grow">{vw.run()}</p>
				</div>
			}
			.into_any(),
			ToastMsg::ErrorView(vw) => view! {
				<div class="horizontal vcenter gap-4">
					<span class="icon i-o-exclamation-triangle icon-error-500"/>
					<p class="overflow-hidden grow">{vw.run()}</p>
				</div>
			}
			.into_any(),
		}
	}
}

/// A message to display for a set amount of time.
#[derive(Clone)]
pub struct Toast {
	/// Message being displayed.
	pub msg: ToastMsg,
	/// Duration of the close timeout.
	pub timeout: Duration,
	/// Is the toast dismissable?
	pub dismissable: bool,
}

impl Default for Toast {
	fn default() -> Self {
		Self {
			msg: ToastMsg::Warn(ArcSignal::stored("No message specified, default toast created".to_string())),
			timeout: Duration::from_secs(3),
			dismissable: true,
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
	/// Toast class.
	#[prop(optional, into)]
	class: Text,
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

	view! {
		{children()}
		<wu-toast-hook class="overlay overlay-container overflow-clip">
			<div class="overlay flex justify-end">
				<wu-toasts class="h-fit divide-y border bg-surface-2 border-surface-3 divide-surface-3 rounded-bl-lg shadow-lg z-100">
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
								<wu-toast class="flex flex-row vcenter gap-4 max-w-lvw min-h-10 px-2 py-1 first:rounded-tl-md last:rounded-bl-md">
									// content
									<div class=move || format!("grow {class}")>
										{toast.msg.clone()}
									</div>
									// close
									{toast.dismissable.then(move || view! {
										<button
											class="btn-icon highlight size-6"
											on:click=move |_| toast_handle.cancel()
										>
											<span class="icon i-o-x-mark"/>
										</button>
									})}
								</wu-toast>
							}
						}
					/>
				</wu-toasts>
			</div>
		</wu-toast-hook>
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
