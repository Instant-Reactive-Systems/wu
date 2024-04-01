use crate::utils::*;
use deref_derive::{Deref, DerefMut};
use leptos::*;
use tailwind_fuse::*;

/// Used to provide a custom view into the modal container.
#[derive(Clone)]
pub struct Modal {
    /// Corresponds to the 'class' attribute of elements.
    pub class: TextProp,
    /// Custom view to insert into the modal container.
    pub content: ViewFn,
}

crate::generate_marker_signal_setter!(
    /// A context signal which allows popping a modal from the stack.
    PopModal, ()
);

crate::generate_marker_signal_setter!(
    /// A context signal which allows adding a custom modal into the container.
    AddModal, Modal
);

/// Used to provide support for modals.
#[component]
pub fn ModalHook<M: 'static>(
    #[prop(optional)] _phant: std::marker::PhantomData<M>,
    /// Specifies the default 'class' attribute for all modals.
    #[prop(default = "".into(), into)]
    class: TextProp,
    /// Children of the component.
    children: Children,
) -> impl IntoView {
    const ACTIVE_MODAL_ID: &str = "wu-active-modal";

    let (modal_id, set_modal_id) = create_signal(0u64);
    let (modals, set_modals) = create_signal(Vec::default());
    let (add_focus_trap, pop_focus_trap) = create_focus_trap(ACTIVE_MODAL_ID);

    let add_modal = SignalSetter::map(move |modal: Modal| {
        let id = modal_id.get_untracked();
        set_modals.update(move |modals| modals.push(ModalWithId { id, modal }));
        set_modal_id.update(|n| *n = n.overflowing_add(1).0);
        add_focus_trap(());
    });
    let pop_modal = SignalSetter::map(move |_: ()| {
        set_modals.update(move |modals| {
            modals.pop();
        });
        pop_focus_trap(());
    });
    provide_context(AddModal::<M>::new(add_modal));
    provide_context(PopModal::<M>::new(pop_modal));

    view! {
        {children()}
        // backdrop
        <wu-modal-backdrop class=move || tw_merge!("overlay", if modals.with(|modals| !modals.is_empty()) { "bg-black/25 backdrop-blur-sm" } else { "" })/>
        // modals
        <For
            each=modals
            key=move |modal| modal.id
            children=move |modal| {
                let last_modal = move || modals.with(move |modals| modals.iter().last().map(|modal| modal.id).unwrap_or(0) == modal.id); // saturating_sub necessary because of signal execution order
                let classes = {
                    let classes = modal.class.clone();
                    let default_modal_classes = class.clone();
                    // need overlay-container for close button
                    move || tw_merge!("overlay-container", default_modal_classes.get(), classes.get())
                };

                view! {
                    <wu-modal
                        id=move || if last_modal() { ACTIVE_MODAL_ID } else { "" }
                        class="overlay flex center z-1"
                    >
                        <div class=classes>
                            <div class="overlay">
                                {modal.content.run()}
                            </div>
                            {last_modal().then(move || view! {
                                <div class="overlay flex justify-end">
                                    <button
                                        class="flex center text-sm font-thin rounded-full hover:bg-light-content/20 hover:dark:bg-dark-content/20 w-fit h-fit p-2"
                                        on:click=move |_| pop_modal(())
                                    >
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                                        </svg>
                                    </button>
                                </div>
                            })}
                        </div>
                    </wu-modal>
                }
            }
        />
    }
}

/// A wrapper around a Modal that holds its ID.
///
/// Used for keyed-for.
#[derive(Clone, Deref, DerefMut)]
struct ModalWithId {
    /// Modal's unique ID.
    id: u64,
    /// The modal itself.
    #[deref]
    modal: Modal,
}

impl PartialEq for ModalWithId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
