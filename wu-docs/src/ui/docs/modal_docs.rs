use crate::prelude::*;
use wu::Modal;

#[component]
pub fn ModalDocs() -> impl IntoView {
    let show_modal = RwSignal::new(false);
    
    view! {
        <Modal 
            toggle = show_modal
            no_blur_bg = false
            closeable = true>
            <div style="width:500px;">
            <p>"Modal text"</p>
            </div>
        </Modal>
        <button on:click=move |_| {show_modal.update(|val| *val = !*val);}>"Show/hide Modal"</button>
    }
}