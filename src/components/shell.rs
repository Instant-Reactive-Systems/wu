use leptos::{prelude::*, either::*};
use crate::utils::{Text, LocatableViewFn};

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
	header: LocatableViewFn,
	/// Inner header slot (a part of the main content area).
	#[prop(optional, into)]
	inner_header: LocatableViewFn,
	/// Left sidebar container slot (immutable).
	#[prop(optional, into)]
	left_sidebar_container: Option<crate::utils::ViewFnWithArgs<Memo<LocatableViewFn>>>,
	/// Left sidebar slot.
	#[prop(optional, into)]
	left_sidebar: LocatableViewFn,
	/// Right sidebar container slot (immutable).
	#[prop(optional, into)]
	right_sidebar_container: Option<crate::utils::ViewFnWithArgs<Memo<LocatableViewFn>>>,
	/// Right sidebar slot.
	#[prop(optional, into)]
	right_sidebar: LocatableViewFn,
	/// Inner footer slot (a part of the main content area).
	#[prop(optional, into)]
	inner_footer: LocatableViewFn,
	/// Footer slot.
	#[prop(optional, into)]
	footer: LocatableViewFn,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	class: Text,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	center_class: Text,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	main_class: Text,
	/// Children of the component.
	children: Children,
) -> impl IntoView
where
	M: Send + Sync + 'static,
{
	// vars
	let shell_cxs = RwSignal::<Vec<ShellCtx>>::new(vec![ShellCtx {
		header: Some(header.clone()),
		inner_header: Some(inner_header.clone()),
		left_sidebar: Some(left_sidebar.clone()),
		right_sidebar: Some(right_sidebar.clone()),
		inner_footer: Some(inner_footer.clone()),
		footer: Some(footer.clone()),
	}]);
	let main_cx = Signal::derive(move || {
		// SAFETY: All of the below unwraps are guaranteed to succeed, given that
		// the initial context is always populated.
		let cxs = shell_cxs.get();
		let header = cxs.iter().rev().map(|cx| cx.header.clone()).find(Option::is_some).flatten().unwrap();
		let inner_header = cxs.iter().rev().map(|cx| cx.inner_header.clone()).find(Option::is_some).flatten().unwrap();
		let left_sidebar = cxs.iter().rev().map(|cx| cx.left_sidebar.clone()).find(Option::is_some).flatten().unwrap();
		let right_sidebar = cxs.iter().rev().map(|cx| cx.right_sidebar.clone()).find(Option::is_some).flatten().unwrap();
		let inner_footer = cxs.iter().rev().map(|cx| cx.inner_footer.clone()).find(Option::is_some).flatten().unwrap();
		let footer = cxs.iter().rev().map(|cx| cx.footer.clone()).find(Option::is_some).flatten().unwrap();
		MainShellContext {
			header,
			inner_header,
			left_sidebar,
			right_sidebar,
			inner_footer,
			footer,
		}
	});
	let header = Memo::new(move |_| main_cx.get().header);
	let inner_header = Memo::new(move |_| main_cx.get().inner_header);
	let left_sidebar = Memo::new(move |_| main_cx.get().left_sidebar);
	let right_sidebar = Memo::new(move |_| main_cx.get().right_sidebar);
	let inner_footer = Memo::new(move |_| main_cx.get().inner_footer);
	let footer = Memo::new(move |_| main_cx.get().footer);

	// provide contexts
	provide_context(PushShell::<M>::new(Callback::new(move |cx| {
		shell_cxs.write().push(cx);
	})));
	provide_context(PopShell::<M>::new(Callback::new(move |_| {
		let last_one = shell_cxs.read_untracked().len() == 1;
		if last_one {
			return;
		} // must not pop first shell context
		shell_cxs.write().pop();
	})));

	view! {
		<wu-shell class=move || format!("overlay vertical {class}")>
			// wu.shell.header
			{move || header.get().run()}
			// center area
			<wu-shell-center class=move || format!("grow horizontal {center_class}")>
				// wu.shell.left_sidebar
				{match left_sidebar_container {
					Some(sidebar) => Either::Left(move || sidebar.run(left_sidebar)),
					None => Either::Right(move || left_sidebar.get().run()),
				}}
				// Main content area
				<wu-shell-main class=move || format!("grow vertical {main_class}")>
					// wu.shell.inner_header
					{move || inner_header.get().run()}
					// wu.shell.content
					<wu-shell-content class="grow overlay-container">
						{children()}
					</wu-shell-content>
					// wu.shell.inner_footer
					{move || inner_footer.get().run()}
				</wu-shell-main>
				// wu.shell.right_sidebar
				{match right_sidebar_container {
					Some(sidebar) => Either::Left(move || sidebar.run(right_sidebar)),
					None => Either::Right(move || right_sidebar.get().run()),
				}}
			</wu-shell-center>
			// wu.shell.footer
			{move || footer.get().run()}
		</wu-shell>
	}
}

/// A utility function to push a new shell onto the stack only for the duration of the
/// current reactive owner.
#[track_caller]
pub fn push_new_shell_ctx<M>(ctx: ShellCtx)
where
	M: Send + Sync + 'static,
{
	let location = std::panic::Location::caller();
	let push_shell_cx = expect_context::<PushShell<M>>();
	let pop_shell_cx = expect_context::<PopShell<M>>();
	Effect::new(move |_| {
		log::trace!("{location} | pushing a new shell ctx");
		push_shell_cx.run(ctx.clone().into());
	});
	on_cleanup(move || {
		log::trace!("{location} | popping a shell ctx");
		pop_shell_cx.run(());
	});
}

/// Holds all slots for a context.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct ShellCtx {
	/// Header slot.
	pub header: Option<LocatableViewFn>,
	/// Inner header slot (a part of the main content area).
	pub inner_header: Option<LocatableViewFn>,
	/// Left sidebar slot.
	pub left_sidebar: Option<LocatableViewFn>,
	/// Right sidebar slot.
	pub right_sidebar: Option<LocatableViewFn>,
	/// Inner footer slot (a part of the main content area).
	pub inner_footer: Option<LocatableViewFn>,
	/// Footer slot.
	pub footer: Option<LocatableViewFn>,
}

impl ShellCtx {
	/// Returns a [`ShellCtx`] which sets all shell fragments to an empty view.
	pub fn cleaned() -> Self {
		Self {
			header: Some(LocatableViewFn::new(move || Some(()))),
			inner_header: Some(LocatableViewFn::new(move || Some(()))),
			left_sidebar: Some(LocatableViewFn::new(move || Some(()))),
			right_sidebar: Some(LocatableViewFn::new(move || Some(()))),
			inner_footer: Some(LocatableViewFn::new(move || Some(()))),
			footer: Some(LocatableViewFn::new(move || Some(()))),
		}
	}
}

/// Holds the the slots of the currently displayed shell context.
#[derive(Clone, PartialEq, Eq, Hash)]
struct MainShellContext {
	/// Header slot.
	pub header: LocatableViewFn,
	/// Inner header slot (a part of the main content area).
	pub inner_header: LocatableViewFn,
	/// Left sidebar slot.
	pub left_sidebar: LocatableViewFn,
	/// Right sidebar slot.
	pub right_sidebar: LocatableViewFn,
	/// Inner footer slot (a part of the main content area).
	pub inner_footer: LocatableViewFn,
	/// Footer slot.
	pub footer: LocatableViewFn,
}
