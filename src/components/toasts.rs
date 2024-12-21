use std::{borrow::Cow, time::Duration};

use leptos::{leptos_dom::helpers::TimeoutHandle, prelude::*, text_prop::TextProp};
use tailwind_fuse::*;

/// A toast message.
///
/// Can be either a simple text payload or a complex view.
#[derive(Clone)]
pub enum ToastMsg {
	/// Simple text payload.
	Text(Cow<'static, str>),
	/// Info message.
	Info(Cow<'static, str>),
	/// Warn message.
	Warn(Cow<'static, str>),
	/// Error message.
	Error(Cow<'static, str>),
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
				<div class="horizontal hcenter gap-4">
					<span class="icon i-o-info-circle"/>
					<p class="overflow-hidden grow">{text}</p>
				</div>
			}
			.into_any(),
			ToastMsg::Warn(text) => view! {
				<div class="horizontal hcenter gap-4">
					<span class="icon i-o-exclamation-circle icon-warning"/>
					<p class="overflow-hidden grow">{text}</p>
				</div>
			}
			.into_any(),
			ToastMsg::Error(text) => view! {
				<div class="horizontal hcenter gap-4">
					<span class="icon i-o-exclamation-triangle icon-error"/>
					<p class="overflow-hidden grow">{text}</p>
				</div>
			}
			.into_any(),
			ToastMsg::View(vw) => view! {
				<div class="overflow-hidden grow">{vw.run()}</div>
			}
			.into_any(),
			ToastMsg::InfoView(vw) => view! {
				<div class="horizontal hcenter gap-4">
					<span class="icon i-o-info-circle"/>
					<p class="overflow-hidden grow">{vw.run()}</p>
				</div>
			}
			.into_any(),
			ToastMsg::WarnView(vw) => view! {
				<div class="horizontal hcenter gap-4">
					<span class="icon i-o-exclamation-circle icon-warning"/>
					<p class="overflow-hidden grow">{vw.run()}</p>
				</div>
			}
			.into_any(),
			ToastMsg::ErrorView(vw) => view! {
				<div class="horizontal hcenter gap-4">
					<span class="icon i-o-exclamation-triangle icon-error"/>
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
			msg: ToastMsg::Warn("No message specified, default toast created".into()),
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

pub type PushToast<M> = crate::utils::Marked<ToastMarker<(M, PushToastMarker)>, Callback<Toast>>;

/// Simple notifications utilizing a dynamic queue system.
#[component]
pub fn ToastHook<M>(
	#[prop(optional)] _phant: std::marker::PhantomData<M>,
	/// Toast class.
	#[prop(default = "".into(), into)]
	class: TextProp,
	/// Dismiss button class.
	#[prop(default = "".into(), into)]
	dismiss_btn_class: TextProp,
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
		let timeout_handle = set_timeout_with_handle(move || toasts.update(move |toasts| toasts.retain(|toast: &ToastWithId| toast.id != id)), toast.timeout);
		let timeout_handle = if let Ok(timeout_handle) = timeout_handle {
			timeout_handle
		} else {
			log::error!("could not create timeout handle. toast will not be created.");
			return;
		};

		toasts.update(move |toasts| toasts.push(ToastWithId { id, toast, timeout_handle }));
		toast_id.update(|n| *n = n.overflowing_add(1).0);
	})));

	view! {
		{children()}
		<wu-toast-hook class="overlay-viewport-container overflow-clip">
			<div class="overlay flex justify-end">
				<wu-toasts class="h-fit divide-y border surface-2 rounded-bl-lg shadow-lg">
					<For
						each=move || toasts.get()
						key=move |toast| toast.id
						children=move |toast| {
							let id = toast.id;
							let timeout_handle = toast.timeout_handle;
							let class = {
								let class = class.clone();
								move || tw_merge!("flex flex-row vcenter gap-4 min-w-[400px] h-10 px-4 pr-2 py-1 surface-bg-1 last:rounded-bl-md", class.get())
							};
							let dismiss_btn_class = {
								let class = dismiss_btn_class.clone();
								move || tw_merge!("flex center text-sm font-thin rounded-full highlight p-2", class.get())
							};

							view! {
								<wu-toast class=class>
									// content
									{toast.msg.clone()}
									// close
									{toast.dismissable.then(move || view! {
										<button
											class=dismiss_btn_class
											on:click=move |_| {
												timeout_handle.clear();
												toasts.update(|toasts| toasts.retain(|toast| toast.id != id));
											}
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
