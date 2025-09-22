use leptos::{html, prelude::*};

use crate::utils::Text;

/// A modal that provides an ergonomic wrapper around `<dialog>`.
#[component]
pub fn Modal(
	/// Specifies the default 'class' attribute for the modal.
	#[prop(optional, into)]
	class: Text,
	/// Specifies the default 'class' attribute for the modal container.
	#[prop(optional, into)]
	container_class: Text,
	/// Signal that opens or closes the modal programmatically.
	#[prop(into)]
	open: Signal<bool>,
	/// Indicates whether the modal is closeable.
	#[prop(default = true, into)]
	closeable: bool,
	/// Indicates whether the modal is local or global (within the local container or on the viewport).
	#[prop(default = true, into)]
	global: bool,
	/// Children of the component.
	children: Children,
) -> impl IntoView {
	// vars
	let dialog_ref = NodeRef::<html::Dialog>::new();
	let is_open = RwSignal::new(false);

	// logic
	Effect::new(move |_| {
		is_open.set(open.get());
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
		<wu-modal class="contents">
			<dialog node_ref=dialog_ref class=format!("group/modal {}", global.then_some("overlay-viewport").unwrap_or("overlay"))>
				<div
					style="\
						background-color: var(--wu-dynamic-modal-bg-color);\
						border-color: var(--wu-dynamic-modal-border-color);\
						border-width: var(--wu-dynamic-modal-border-width);\
						border-radius: var(--wu-dynamic-modal-border-radius);\
						box-shadow: var(--wu-dynamic-modal-shadow);\
						padding: var(--wu-dynamic-modal-padding);\
					"
					class=move || format!("overlay overlay-center flex sm:w-auto h-auto max-h-svh transition starting:group-open/modal:opacity-0 {container_class}")
				>
					// Content
					<div class=move || format!("w-full flex-1 {class}")>
						{children()}
					</div>
					// Close button
					{closeable.then(move || view! {
						<div
							style="\
								margin-top: var(--wu-dynamic-modal-padding);\
								margin-right: var(--wu-dynamic-modal-padding);\
							"
							class="overlay overlay-tr size-fit"
						>
							<button on:click=move |_| is_open.set(false) class="btn-icon size-8 autohighlight text-content-sideinfo">
								<span class="icon i-o-x-mark"/>
							</button>
						</div>
					})}
				</div>
			</dialog>
		</wu-modal>
	}
}
