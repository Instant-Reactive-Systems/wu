use leptos::{html, prelude::*};

use crate::utils::Text;

/// A modal that provides an ergonomic wrapper around `<dialog>`.
#[component]
pub fn Modal(
	/// Specifies the default 'class' attribute for all modals.
	#[prop(optional, into)]
	class: Text,
	/// Signal that opens or closes the modal.
	#[prop(into)]
	toggle: Signal<bool>,
	/// Signal that indicates whether the background should be blurred.
	#[prop(default = false, into)]
	no_blur_bg: bool,
	/// Signal that indicates whether the modal is closeable.
	#[prop(default = true, into)]
	closeable: bool,
	/// Children of the component.
	children: Children,
) -> impl IntoView {
	// vars
	let dialog_ref = NodeRef::<html::Dialog>::new();
	let is_open = RwSignal::new(false);

	// logic
	Effect::new(move |_| {
		is_open.set(toggle.get());
	});
	Effect::new(move |_| match is_open.get() {
		true => {
			if let Some(dialog) = dialog_ref.get() {
				if closeable {
					_ = dialog.show_modal();
				} else {
					_ = dialog.show();
				}
			}
		},
		false => {
			if let Some(dialog) = dialog_ref.get() {
				dialog.close();
			}
		},
	});

	view! {
		<dialog node_ref=dialog_ref>
			<div class=format!("overlay-viewport-container flex hcenter h-sm:vcenter bg-transparent z-9999 {}", if !no_blur_bg { "backdrop:blur-sm backdrop:backdrop-blur-sm backdrop:bg-black/50" } else { "backdrop:bg-transparent" })>
				<div class="overlay-container px-4 not-md:w-full">
					// Content
					<div class=move || format!("overlay p-4 not-md:w-full max-w-lvw not-h-sm:h-fit max-h-lvh {class}")>
						{children()}
					</div>
					// Close button
					{closeable.then(move || view! {
						<div class="overlay p-4 flex justify-end">
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
					})}
				</div>
			</div>
		</dialog>
	}
}
