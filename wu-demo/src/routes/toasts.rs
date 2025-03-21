use crate::components::*;
use leptos::*;

#[component]
pub fn Toasts() -> impl IntoView {
    let create_toast = expect_context::<wu::PushToast<crate::Main>>();
    let make_text_toast = move |_| {
        create_toast(wu::Toast {
            msg: wu::ToastMsg::Text("hello world".into()),
            timeout: std::time::Duration::from_secs(5),
            dismissable: true,
        })
    };
    let make_view_toast = move |_| {
        create_toast(wu::Toast {
            msg: wu::ToastMsg::View(ViewFn::from(move || {
                view! {
                    <p>"hello world from view"</p>
                }
            })),
            timeout: std::time::Duration::from_secs(5),
            dismissable: true,
        })
    };

    view! {
        <div class="h-[1400px] flex flex-row center gap-4">
            <button on:click=make_text_toast class="btn bg-yellow-600 rounded-lg">"make text toast"</button>
            <button on:click=make_view_toast class="btn bg-yellow-600 rounded-lg">"make view toast"</button>
        </div>
    }
}
