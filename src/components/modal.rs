use leptos::{html, prelude::*, text_prop::TextProp};
use tailwind_fuse::*;

/// A modal that provides an ergonomic wrapper around `<dialog>`.
#[component]
pub fn Modal(
	/// Specifies the default 'class' attribute for all modals.
	#[prop(default = "".into(), into)]
	class: TextProp,
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
	let class = Signal::derive(move || tw_merge!("overlay p-4", class.get()));
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

	// TODO: wait for AttributeInterceptor to pass it to the inner input
	view! {
		<wu-modal class="contents">
			<dialog node_ref=dialog_ref class=format!("w-lvw h-lvh bg-transparent {}", if !no_blur_bg { "backdrop:blur-sm backdrop:backdrop-blur-sm backdrop:bg-black/50" } else { "backdrop:bg-transparent" })>
				<div class="overlay-viewport-container">
					<div class="overlay flex center">
						<div class="overlay-container max-w-lvw p-4 tablet:p-8">
							// Content
							<div class=move || format!("z-100 {}", class.get())>
								{children()}
							</div>
							// Close button
							{closeable.then(move || view! {
								<div class="overlay p-4 flex justify-end">
									<div class="horizontal w-fit h-fit vcenter gap-2 opacity-50">
										<span class="hidden desktop:kbd surface-2">"ESC"</span>
										<span class="hidden desktop:block text-xs">"or"</span>
										<button
											class="flex center btn-circle p-2 highlight"
											on:click=move |_| is_open.set(false)
										>
											<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
												<path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
											</svg>
										</button>
									</div>
								</div>
							})}
						</div>
					</div>
				</div>
			</dialog>
		</wu-modal>
	}
}
