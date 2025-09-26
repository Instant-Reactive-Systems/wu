use leptos::prelude::*;

use crate::utils::*;

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
	/// Left sidebar slot.
	#[prop(optional, into)]
	left_sidebar: LocatableViewFn,
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
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	content_class: Text,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	header_class: Text,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	left_sidebar_class: Text,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	right_sidebar_class: Text,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	inner_header_class: Text,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	inner_footer_class: Text,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	footer_class: Text,
	/// Children of the component.
	children: Children,
) -> impl IntoView
where
	M: Send + Sync + 'static,
{
	// vars
	let shell_cxs = RwSignal::<Vec<ShellCtx>>::new(vec![ShellCtx {
		header: header.clone().into(),
		inner_header: inner_header.clone().into(),
		left_sidebar: left_sidebar.clone().into(),
		right_sidebar: right_sidebar.clone().into(),
		inner_footer: inner_footer.clone().into(),
		footer: footer.clone().into(),
	}]);
	let main_cx = Signal::derive(move || {
		// SAFETY: All of the below unwraps are guaranteed to succeed, given that
		// the initial context is always populated.
		let cxs = shell_cxs.get();
		let header = cxs.iter().rev().map(|cx| cx.header.clone()).find(ShellCtxItem::is_not_inherit).unwrap();
		let inner_header = cxs.iter().rev().map(|cx| cx.inner_header.clone()).find(ShellCtxItem::is_not_inherit).unwrap();
		let left_sidebar = cxs.iter().rev().map(|cx| cx.left_sidebar.clone()).find(ShellCtxItem::is_not_inherit).unwrap();
		let right_sidebar = cxs.iter().rev().map(|cx| cx.right_sidebar.clone()).find(ShellCtxItem::is_not_inherit).unwrap();
		let inner_footer = cxs.iter().rev().map(|cx| cx.inner_footer.clone()).find(ShellCtxItem::is_not_inherit).unwrap();
		let footer = cxs.iter().rev().map(|cx| cx.footer.clone()).find(ShellCtxItem::is_not_inherit).unwrap();
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
		<wu-shell class=move || format!("flex-1 vertical {class}")>
			// wu.shell.header
			{move || header.get().run().map(move |vw| view! {
				<wu-shell-header class=move || format!("flex-none {header_class}")>
					{vw}
				</wu-shell-header>
			})}
			// center area
			<wu-shell-center class=move || format!("flex-1 horizontal {center_class}")>
				// wu.shell.left_sidebar
				{move || left_sidebar.get().run().map(move |vw| view! {
					<wu-shell-left-sidebar role="complementary" class=move || format!("flex-none {left_sidebar_class}")>
						{vw}
					</wu-shell-left-sidebar>
				})}
				// Main content area
				<wu-shell-main class=move || format!("flex-1 vertical {main_class}")>
					// wu.shell.inner_header
					{move || inner_header.get().run().map(move |vw| view! {
						<wu-shell-inner-header class=move || format!("flex-none {inner_header_class}")>
							{vw}
						</wu-shell-inner-header>
					})}
					// wu.shell.content
					<wu-shell-content role="main" class=move || format!("flex-1 flex relative *:flex-1 {content_class}")>
						{children()}
					</wu-shell-content>
					// wu.shell.inner_footer
					{move || inner_footer.get().run().map(move |vw| view! {
						<wu-shell-inner-footer class=move || format!("flex-none {inner_footer_class}")>
							{vw}
						</wu-shell-inner-footer>
					})}
				</wu-shell-main>
				// wu.shell.right_sidebar
				{move || right_sidebar.get().run().map(move |vw| view! {
					<wu-shell-right-sidebar role="complementary" class=move || format!("flex-none {right_sidebar_class}")>
						{vw}
					</wu-shell-right-sidebar>
				})}
			</wu-shell-center>
			// wu.shell.footer
			{move || footer.get().run().map(move |vw| view! {
				<wu-shell-footer class=move || format!("flex-none {footer_class}")>
					{vw}
				</wu-shell-footer>
			})}
		</wu-shell>
	}
}

/// A utility function to push a new shell onto the stack only for the duration of the
/// current reactive owner.
pub fn push_new_shell_ctx<M>(ctx: ShellCtx)
where
	M: Send + Sync + 'static,
{
	let push_shell_cx = expect_context::<PushShell<M>>();
	let pop_shell_cx = expect_context::<PopShell<M>>();
	Effect::new(move |_| push_shell_cx.run(ctx.clone().into()));
	on_cleanup(move || pop_shell_cx.run(()));
}

/// Item specifying what the slot of the [`ShellCtx`] will be.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
enum ShellCtxItem {
	#[default]
	Inherit,
	None,
	Some(LocatableViewFn),
}

impl ShellCtxItem {
	fn is_not_inherit(&self) -> bool {
		!matches!(self, Self::Inherit)
	}

	fn run(&self) -> Option<AnyView> {
		match self {
			Self::Some(vw) => Some(vw.run()),
			_ => None,
		}
	}
}

impl From<LocatableViewFn> for ShellCtxItem {
	fn from(value: LocatableViewFn) -> Self {
		if value.is_default {
			Self::None
		} else {
			Self::Some(value)
		}
	}
}

/// Holds all slots for a context.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct ShellCtx {
	/// Header slot.
	header: ShellCtxItem,
	/// Inner header slot (a part of the main content area).
	inner_header: ShellCtxItem,
	/// Left sidebar slot.
	left_sidebar: ShellCtxItem,
	/// Right sidebar slot.
	right_sidebar: ShellCtxItem,
	/// Inner footer slot (a part of the main content area).
	inner_footer: ShellCtxItem,
	/// Footer slot.
	footer: ShellCtxItem,
}

impl ShellCtx {
	/// Creates a new inherited [`ShellCtx`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Creates a [`ShellCtx`] which removes all shell fragments.
	pub fn cleaned() -> Self {
		Self {
			header: ShellCtxItem::None,
			inner_header: ShellCtxItem::None,
			left_sidebar: ShellCtxItem::None,
			right_sidebar: ShellCtxItem::None,
			inner_footer: ShellCtxItem::None,
			footer: ShellCtxItem::None,
		}
	}

	/// Removes the header from the context.
	pub fn no_header(mut self) -> Self {
		self.header = ShellCtxItem::None;
		self
	}

	/// Adds a header to the context.
	pub fn header(mut self, header: impl Into<LocatableViewFn>) -> Self {
		self.header = ShellCtxItem::Some(header.into());
		self
	}

	/// Removes the inner header from the context.
	pub fn no_inner_header(mut self) -> Self {
		self.inner_header = ShellCtxItem::None;
		self
	}

	/// Adds an inner header to the context.
	pub fn inner_header(mut self, inner_header: impl Into<LocatableViewFn>) -> Self {
		self.inner_header = ShellCtxItem::Some(inner_header.into());
		self
	}

	/// Removes the left sidebar from the context.
	pub fn no_left_sidebar(mut self) -> Self {
		self.left_sidebar = ShellCtxItem::None;
		self
	}

	/// Adds a left sidebar to the context.
	pub fn left_sidebar(mut self, left_sidebar: impl Into<LocatableViewFn>) -> Self {
		self.left_sidebar = ShellCtxItem::Some(left_sidebar.into());
		self
	}

	/// Removes the right sidebar from the context.
	pub fn no_right_sidebar(mut self) -> Self {
		self.right_sidebar = ShellCtxItem::None;
		self
	}

	/// Adds a right sidebar to the context.
	pub fn right_sidebar(mut self, right_sidebar: impl Into<LocatableViewFn>) -> Self {
		self.right_sidebar = ShellCtxItem::Some(right_sidebar.into());
		self
	}

	/// Removes the inner footer from the context.
	pub fn no_inner_footer(mut self) -> Self {
		self.inner_footer = ShellCtxItem::None;
		self
	}

	/// Adds an inner footer to the context.
	pub fn inner_footer(mut self, inner_footer: impl Into<LocatableViewFn>) -> Self {
		self.inner_footer = ShellCtxItem::Some(inner_footer.into());
		self
	}

	/// Removes the footer from the context.
	pub fn no_footer(mut self) -> Self {
		self.footer = ShellCtxItem::None;
		self
	}

	/// Adds a footer to the context.
	pub fn footer(mut self, footer: impl Into<LocatableViewFn>) -> Self {
		self.footer = ShellCtxItem::Some(footer.into());
		self
	}
}

/// Holds the the slots of the currently displayed shell context.
#[derive(Clone, PartialEq, Eq, Hash)]
struct MainShellContext {
	/// Header slot.
	pub header: ShellCtxItem,
	/// Inner header slot (a part of the main content area).
	pub inner_header: ShellCtxItem,
	/// Left sidebar slot.
	pub left_sidebar: ShellCtxItem,
	/// Right sidebar slot.
	pub right_sidebar: ShellCtxItem,
	/// Inner footer slot (a part of the main content area).
	pub inner_footer: ShellCtxItem,
	/// Footer slot.
	pub footer: ShellCtxItem,
}
