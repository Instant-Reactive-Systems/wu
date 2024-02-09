use crate::components::*;
use leptos::*;
use leptos_router::*;
use std::rc::Rc;

#[component]
pub fn Modals() -> impl IntoView {
    let add_modal = expect_context::<AddModal<crate::Main>>();
    let pop_modal = expect_context::<PopModal<crate::Main>>();

    let open_modal_2 = move |_| {
        add_modal(Modal {
            class: "w-[700px] h-[500px]".into(),
            content: ViewFn::from(move || {
                view! {
                    <div class="flex flex-col gap-4">
                        <h1 class="text-xl font-bold text-center">"Epic time #2"</h1>
                        <p>
                            "
                            Lorem ipsum dolor sit amet consectetur adipisicing elit. 
                            Assumenda soluta temporibus provident reprehenderit similique veniam repudiandae. 
                            Velit beatae minus, vel minima cupiditate quisquam distinctio harum assumenda accusamus qui 
                            laboriosam odio, molestiae repellat quam, esse at tenetur officia error!
                            "
                        </p>
                    </div>
                }
            }),
        })
    };

    let open_modal = move |_| {
        add_modal(Modal {
            class: "w-[400px] h-[300px]".into(),
            content: ViewFn::from(move || {
                view! {
                    <div class="flex flex-col gap-4">
                        <h1 class="text-xl font-bold text-center">"Epic time"</h1>
                        <p>
                            "
                            Lorem ipsum dolor sit amet consectetur adipisicing elit. 
                            Assumenda soluta temporibus provident reprehenderit similique veniam repudiandae. 
                            Velit beatae minus, vel minima cupiditate quisquam distinctio harum assumenda accusamus qui 
                            laboriosam odio, molestiae repellat quam, esse at tenetur officia error!
                            "
                        </p>
                        <button on:click=open_modal_2 class="btn bg-red-600 rounded-lg">"open modal"</button>
                    </div>
                }
            }),
        })
    };

    view! {
        <div class="flex center">
            <button on:click=open_modal class="btn bg-red-600 rounded-lg">"open modal"</button>
        </div>
    }
}
