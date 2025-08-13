use leptos::{html, prelude::*};

use crate::utils::Text;

/// All possible drawer positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerPosition {
	Left,
	Right,
	Top,
	Bottom,
}

/// Displays a panel on an arbitrary side of the screen.
#[component]
pub fn Drawer(
	/// What side to put the drawer on.
	#[prop(default = DrawerPosition::Right)]
	position: DrawerPosition,
	/// Signal to open or close the drawer programmatically.
	#[prop(optional, into)]
	toggle: Signal<bool>,
	/// Size of the drawer in px.
	#[prop(default = 300)]
	size: i32,
	/// Drawer class.
	#[prop(default = "".into(), into)]
	class: Text,
	/// Children of the component.
	children: Children,
) -> impl IntoView {
	// vars
	let location = leptos_router::hooks::use_location();
	let dialog_ref = NodeRef::<html::Dialog>::new();
	let is_open = RwSignal::new(false);

	// logic
	Effect::new(move |_| is_open.set(toggle.get()));
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
	_ = leptos_use::use_event_listener(dialog_ref, leptos::ev::close, move |_| is_open.set(false));
	Effect::new(move |_| {
		location.pathname.track();
		is_open.set(false);
	});

	let get_initial_position = move || -> String {
		match position {
			DrawerPosition::Left => format!("left: {}px; top: 0; height: 100%; width: {}px;", -size, size),
			DrawerPosition::Right => format!("right: {}px; top: 0; height: 100%; width: {}px;", -size, size),
			DrawerPosition::Top => format!("top: {}px; left: 0; width: 100%; height: {}px;", -size, size),
			DrawerPosition::Bottom => format!("bottom: {}px; left: 0; width: 100%; height: {}px;", -size, size),
		}
	};

	let get_transform = Signal::derive(move || -> String {
		let translate = match position {
			DrawerPosition::Left => format!("translateX({}px)", if is_open.get() { size as f64 } else { 0.0 }),
			DrawerPosition::Right => format!("translateX({}px)", if is_open.get() { -size as f64 } else { 0.0 }),
			DrawerPosition::Top => format!("translateY({}px)", if is_open.get() { size as f64 } else { 0.0 }),
			DrawerPosition::Bottom => format!("translateY({}px)", if is_open.get() { -size as f64 } else { 0.0 }),
		};
		translate
	});

	view! {
		<wu-drawer class="contents">
			<dialog node_ref=dialog_ref>
				<div class="overlay-viewport-container">
					// Content
					<div
						class=move || format!("overlay {class}")
						style=move || format!(
							"position: absolute; \
							overflow: hidden; \
							{} \
							transform: {}; \
							transition: transform 0.3s ease-out; \
							transition-behavior: allow-discrete; \
							box-shadow: 0 0 10px rgba(0,0,0,0.2);",
							get_initial_position(),
							get_transform.get(),
						)
					>
						{children()}
					</div>
					// Close button
					<div class="overlay flex justify-end p-2">
						<div class="horizontal w-fit h-fit vcenter gap-2 opacity-50">
							<div class="hidden xl:inline-flex gap-2 vcenter">
								<span class="kbd">"ESC"</span>
								<span class="text-xs">"or"</span>
							</div>
							<button
								class="btn-icon size-8 highlight"
								on:click=move |_| is_open.set(false)
							>
								<span class="icon i-o-x-mark"/>
							</button>
						</div>
					</div>
				</div>
			</dialog>
		</wu-drawer>
	}
}
