use leptos::*;
use tailwind_fuse::*;

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
	#[prop(default = DrawerPosition::Left)]
	position: DrawerPosition,
	/// Initial state of the drawer.
	#[prop(default = false)]
	open: bool,
	/// Signal to open the drawer programmatically.
	#[prop(optional, into)]
	signal_to_open: Signal<()>,
	/// Size of the drawer in px.
	#[prop(default = 300)]
	size: i32,
	/// Drawer class.
	#[prop(default = "".into(), into)]
	class: TextProp,
	/// List of attributes to put on the top-level of the component.
	#[prop(attrs)]
	attrs: Vec<(&'static str, Attribute)>,
	/// Children of the component.
	children: Children,
) -> impl IntoView {
	// vars
	let offset = create_rw_signal(0.0);
	let is_dragging = create_rw_signal(false);
	let is_open = create_rw_signal(open);
	let dialog_ref = create_node_ref::<html::Dialog>();

	// logic
	_ = watch(move || signal_to_open.get(), move |_, _, _| is_open.set(true), false);

	create_effect(move |_| {
		if let Some(dialog) = dialog_ref.get() {
			if is_open.get() && !dialog.open() {
				_ = dialog.show_modal();
			} else if !is_open.get() && dialog.open() {
				dialog.close();
			}
		}
	});

	let get_initial_position = move || -> String {
		match position {
			DrawerPosition::Left => format!("left: {}px; top: 0; height: 100%; width: {}px;", -size, size),
			DrawerPosition::Right => format!("right: {}px; top: 0; height: 100%; width: {}px;", -size, size),
			DrawerPosition::Top => format!("top: {}px; left: 0; width: 100%; height: {}px;", -size, size),
			DrawerPosition::Bottom => format!("bottom: {}px; left: 0; width: 100%; height: {}px;", -size, size),
		}
	};

	let get_transform = move || -> String {
		let translate = match position {
			DrawerPosition::Left => format!(
				"translateX({}px)",
				if is_dragging.get() {
					offset.get()
				} else {
					if is_open.get() {
						size as f64
					} else {
						0.0
					}
				}
			),
			DrawerPosition::Right => format!(
				"translateX({}px)",
				if is_dragging.get() {
					-offset.get()
				} else {
					if is_open.get() {
						-size as f64
					} else {
						0.0
					}
				}
			),
			DrawerPosition::Top => format!(
				"translateY({}px)",
				if is_dragging.get() {
					offset.get()
				} else {
					if is_open.get() {
						size as f64
					} else {
						0.0
					}
				}
			),
			DrawerPosition::Bottom => format!(
				"translateY({}px)",
				if is_dragging.get() {
					-offset.get()
				} else {
					if is_open.get() {
						-size as f64
					} else {
						0.0
					}
				}
			),
		};
		translate
	};

	view! {
		<wu-drawer class="contents">
			<dialog node_ref=dialog_ref>
				<div class="overlay-viewport-container"> //  style="position: fixed; inset: 0; overflow: hidden;"
					// Content
					<div
						{..attrs}
						class=move || tw_merge!("overlay", class.get())
						style=move || format!(
							"position: absolute; \
							overflow: hidden; \
							{} \
							transform: {}; \
							transition: transform 0.3s ease-out; \
							transition-behavior: allow-discrete; \
							box-shadow: 0 0 10px rgba(0,0,0,0.2);",
							get_initial_position(),
							get_transform(),
						)
					>
						{children()}
					</div>
					// Close button
					<div class="overlay flex justify-end p-2">
						<div class="horizontal w-fit h-fit vcenter gap-2 opacity-50">
							<span class="hidden desktop:kbd surface-2">"ESC"</span>
							<span class="hidden desktop:block text-xs">"or"</span>
							<button
								class="flex center btn-circle p-2 focus-within:bg-light-3/20 dark:focus-within:bg-dark-3/20 hover:bg-light-3/20 dark:hover:bg-dark-3/20"
								on:click=move |_| dialog_ref.get().unwrap().close()
							>
								<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
									<path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
								</svg>
							</button>
						</div>
					</div>
				</div>
			</dialog>
		</wu-drawer>
	}
}
