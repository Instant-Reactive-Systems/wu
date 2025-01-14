use leptos::{prelude::*, text_prop::TextProp};
use tailwind_fuse::*;

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellMarker<M>(std::marker::PhantomData<M>);

crate::generate_marker_type!(
	#[doc(hidden)]
	PushShellMarker
);
crate::generate_marker_type!(
	#[doc(hidden)]
	PopShellMarker
);

pub type PushShell<M> = crate::utils::Marked<ShellMarker<(M, PushShellMarker)>, Callback<ShellCtx>>;
pub type PopShell<M> = crate::utils::Marked<ShellMarker<(M, PushShellMarker)>, Callback<()>>;

/// Used to specify a composable common layout.
#[component]
pub fn Shell<M>(
	#[prop(optional)] _phant: std::marker::PhantomData<M>,
	/// Header slot.
	#[prop(optional, into)]
	header: ViewFn,
	/// Left sidebar slot.
	#[prop(optional, into)]
	left_sidebar: ViewFn,
	/// Right sidebar slot.
	#[prop(optional, into)]
	right_sidebar: ViewFn,
	/// Footer slot.
	#[prop(optional, into)]
	footer: ViewFn,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(default = "".into(), into)]
	class: TextProp,
	/// Children of the component.
	children: Children,
) -> impl IntoView
where
	M: Send + Sync + 'static,
{
	let shell_cxs = RwSignal::<Vec<ShellCtx>>::new(vec![ShellCtx {
		header: Some(header.clone()),
		left_sidebar: Some(left_sidebar.clone()),
		right_sidebar: Some(right_sidebar.clone()),
		footer: Some(footer.clone()),
	}]);
	let main_cx = Signal::derive(move || {
		// SAFETY: All of the below unwraps are guaranteed to succeed, given that
		// the initial context is always populated.
		let cxs = shell_cxs.get();
		let header = cxs.iter().rev().map(|cx| cx.header.clone()).find(Option::is_some).flatten().unwrap();
		let left_sidebar = cxs.iter().rev().map(|cx| cx.left_sidebar.clone()).find(Option::is_some).flatten().unwrap();
		let right_sidebar = cxs.iter().rev().map(|cx| cx.right_sidebar.clone()).find(Option::is_some).flatten().unwrap();
		let footer = cxs.iter().rev().map(|cx| cx.footer.clone()).find(Option::is_some).flatten().unwrap();
		MainShellContext {
			header,
			left_sidebar,
			right_sidebar,
			footer,
		}
	});
	provide_context(PushShell::<M>::new(Callback::new(move |cx| {
		shell_cxs.update(move |cxs| cxs.push(cx));
	})));
	provide_context(PopShell::<M>::new(Callback::new(move |_| {
		let last_one = shell_cxs.read_untracked().len() == 1;
		if last_one {
			return;
		} // must not pop first shell context
		shell_cxs.update(move |cxs| _ = cxs.pop());
	})));

	view! {
		<wu-shell class=move || tw_merge!("overlay flex flex-col", class.get())>
			// Header
			{move || main_cx.get().header.run()}
			// center area
			<div class="grow flex flex-row">
				// LeftSidebar
				{move || main_cx.get().left_sidebar.run()}
				// Main content area
				<div class="grow overlay-container">
					{children()}
				</div>
				// RightSidebar
				{move || main_cx.get().right_sidebar.run()}
			</div>
			// Footer
			{move || main_cx.get().footer.run()}
		</wu-shell>
	}
}

/// Holds all slots for a context.
#[derive(Clone)]
pub struct ShellCtx {
	/// Header slot.
	pub header: Option<ViewFn>,
	/// Left sidebar slot.
	pub left_sidebar: Option<ViewFn>,
	/// Right sidebar slot.
	pub right_sidebar: Option<ViewFn>,
	/// Footer slot.
	pub footer: Option<ViewFn>,
}

/// Holds the the slots of the currently displayed shell context.
#[derive(Clone)]
struct MainShellContext {
	/// Header slot.
	pub header: ViewFn,
	/// Left sidebar slot.
	pub left_sidebar: ViewFn,
	/// Right sidebar slot.
	pub right_sidebar: ViewFn,
	/// Footer slot.
	pub footer: ViewFn,
}
