use leptos::{html, prelude::*};

use crate::utils::{Text, Position};

/// Displays a panel on an arbitrary side of the screen.
#[component]
pub fn Drawer(
	/// What side to put the drawer on.
	///
	/// # Note
	/// Anything other than `Left`, `Right`, `Top` and `Bottom` will be ignored.
	#[prop(default = Position::Right)]
	position: Position,
	/// Signal to open or close the drawer programmatically.
	#[prop(into)]
	open: Signal<bool>,
	/// Specifies the default 'class' attribute for the drawer.
	#[prop(optional, into)]
	class: Text,
	/// Specifies the default 'class' attribute for the drawer container.
	#[prop(optional, into)]
	container_class: Text,
	/// Children of the component.
	children: Children,
) -> impl IntoView {
	// vars
	let location = leptos_router::hooks::use_location();
	let dialog_ref = NodeRef::<html::Dialog>::new();
	let is_open = RwSignal::new(false);

	// logic
	Effect::new(move |_| {
		is_open.set(open.get());
	});
	Effect::new(move |_| match is_open.get() {
		true => {
			if let Some(dialog) = dialog_ref.get() {
				_ = dialog.show_modal();
			}
		},
		false => {
			if let Some(dialog) = dialog_ref.get() {
				dialog.close();
			}
		},
	});
	Effect::new(move |_| {
		location.pathname.track();
		is_open.set(false);
	});

	// default to Right if no valid position was passed
	let position = match position {
		Position::Left => Position::Left,
		Position::Top => Position::Top,
		Position::Bottom => Position::Bottom,
		_ => Position::Right,
	};

	#[rustfmt::skip]
	let (position_class, border_class, shadow_class, size_class) = match position {
		Position::Left => (
			"overlay-tl",
			"border-r-(--wu-dynamic-drawer-border-width) rounded-r-(--wu-dynamic-drawer-border-radius)",
			"shadow-right-lg",
			"w-(--wu-dynamic-drawer-size)",
		),
		Position::Right => (
			"overlay-tr",
			"border-l-(--wu-dynamic-drawer-border-width) rounded-l-(--wu-dynamic-drawer-border-radius)",
			"shadow-left-lg",
			"w-(--wu-dynamic-drawer-size)",
		),
		Position::Top => (
			"overlay-tl",
			"border-b-(--wu-dynamic-drawer-border-width) rounded-b-(--wu-dynamic-drawer-border-radius)",
			"shadow-lg",
			"h-(--wu-dynamic-drawer-size)",
		),
		Position::Bottom => (
			"overlay-bl",
			"border-t-(--wu-dynamic-drawer-border-width) rounded-t-(--wu-dynamic-drawer-border-radius)",
			"shadow-top-lg",
			"h-(--wu-dynamic-drawer-size)",
		),
		_ => unreachable!("cannot happen since we limited the position to the 4 main ones"),
	};

	view! {
		<wu-drawer class="contents">
			<dialog node_ref=dialog_ref class="group/drawer overlay overflow-hidden">
				<div
					style="\
						background-color: var(--wu-dynamic-drawer-bg-color);\
						border-color: var(--wu-dynamic-drawer-border-color);\
						padding: var(--wu-dynamic-drawer-padding);\
					"
					class=move || format!("overlay max-w-lvw max-h-svh transition transition-discrete {position_class} {border_class} {shadow_class} {size_class} {container_class}")
				>
					// Content
					<div class=move || format!("cover {class}")>
						{children()}
					</div>
					// Close button
					<div
						style="\
							margin-top: var(--wu-dynamic-drawer-padding);\
							margin-right: var(--wu-dynamic-drawer-padding);\
						"
						class="overlay overlay-tr size-fit"
					>
						<button on:click=move |_| is_open.set(false) class="btn-icon size-8 autohighlight text-content-sideinfo">
							<span class="icon i-o-x-mark"/>
						</button>
					</div>
				</div>
			</dialog>
		</wu-drawer>
	}
}
