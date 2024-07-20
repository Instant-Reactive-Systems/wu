use crate::prelude::*;

#[component]
pub fn Modals() -> impl IntoView {
    view! {
        <button popovertarget="wu-modal-demo" class="btn btn-primary">
            "Open modal"
        </button>
        <dialog id="wu-modal-demo" popover>
            <div class="flex flex-col gap-4 surface-1">
                <h1 class="text-xl font-bold text-center">"A modal demo"</h1>
                <p>
                    "
                    Lorem ipsum dolor sit amet consectetur adipisicing elit. 
                    Assumenda soluta temporibus provident reprehenderit similique veniam repudiandae. 
                    Velit beatae minus, vel minima cupiditate quisquam distinctio harum assumenda accusamus qui 
                    laboriosam odio, molestiae repellat quam, esse at tenetur officia error!
                    "
                </p>
                <button class="btn btn-primary">"Ok"</button>
            </div>
        </dialog>
    }
}
