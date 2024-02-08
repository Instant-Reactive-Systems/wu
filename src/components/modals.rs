use crate::utils::*;
use leptos::*;
use std::rc::Rc;
use deref_derive::{Deref, DerefMut};

/// Used to provide a custom view into the modal container.
#[derive(Clone)]
pub struct Modal {
    /// Custom view to insert into the modal container.
    pub content: Rc<dyn Fn() -> View>,
    /// Corresponds to the 'class' attribute of elements.
    pub class: TextProp,
}

/// A context signal which allows adding a custom modal into the container.
#[derive(Clone)]
pub struct AddModal(pub SignalSetter<Modal>);

/// A context signal which allows popping a modal from the stack.
#[derive(Clone)]
pub struct PopModal(pub SignalSetter<()>);

/// Used to provide support for modals.
#[component]
pub fn ModalHook(
    /// Specifies the default 'class' attribute for all modals.
    #[prop(default = "".into(), into)] class: TextProp,
    /// Children of the component.
    children: Children,
) -> impl IntoView {
    const RESERVED_ID: &str = "wu-active-modal";

    let (modal_id, set_modal_id) = create_signal(0u64);
    let (modals, set_modals) = create_signal(Vec::default());
    let (add_focus_trap, pop_focus_trap) = create_focus_trap(RESERVED_ID);

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
    provide_context(AddModal(add_modal));
    provide_context(PopModal(pop_modal));

    view! {
        {children()}
        // Main modal overlay container
        <For
            each=modals
            key=move |modal| modal.id
            children=move |modal| {
                let last_modal = move || modals.with(move |modals| modals.iter().last().map(|modal| modal.id).unwrap_or(0) == modal.id); // saturating_sub necessary because of signal execution order
                let classes = {
                    let classes = modal.class.clone();
                    let default_modal_classes = class.clone();
                    // need overlay-container for close button
                    move || format!("overlay-container {} {}", default_modal_classes.get(), classes.get())
                };

                view! {
                    <div
                        id=move || if last_modal() { RESERVED_ID } else { "" }
                        class="overlay flex center z-2"
                    >
                        <div class=classes>
                            <div class="overlay">
                                {(*modal.content)()}
                            </div>
                            {last_modal().then(move || view! {
                                <div class="overlay flex justify-end">
                                    <button
                                        class="flex center text-sm font-thin rounded-full hover:bg-surface-200/20 square-6 p-2"
                                        on:click=move |_| pop_modal(())
                                    >
                                        "✕"
                                    </button>
                                </div>
                            })}
                        </div>
                    </div>
                }
            }
        />
        <div class=move || format!("overlay z-1 {}", if !modals().is_empty() { "bg-black/50 backdrop-blur-sm" } else { "" })/>
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

