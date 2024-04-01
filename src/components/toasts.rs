use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::*;
use tailwind_fuse::*;

use deref_derive::{Deref, DerefMut};
use std::borrow::Cow;
use std::time::Duration;

/// A toast message.
///
/// Can be either a simple text payload or a complex view.
#[derive(Clone)]
pub enum ToastMsg {
    /// Simple text payload.
    Text(Cow<'static, str>),
    /// Complex view.
    View(ViewFn),
}

/// A message to display for a set amount of time.
#[derive(Clone)]
pub struct Toast {
    /// Message being displayed.
    pub msg: ToastMsg,
    /// Duration of the close timeout.
    pub timeout: Duration,
    /// Is the toast dismissable?
    pub dismissable: bool,
}

crate::generate_marker_signal_setter!(
    /// Pushes a toast onto the hook.
    PushToast, Toast
);

/// Simple notifications utilizing a dynamic queue system.
#[component]
pub fn ToastHook<M: 'static>(
    #[prop(optional)] _phant: std::marker::PhantomData<M>,
    /// Toast class.
    #[prop(default = "".into(), into)]
    class: TextProp,
    /// Dismiss button class.
    #[prop(default = "".into(), into)]
    dismiss_btn_class: TextProp,
    /// Children of the component.
    children: Children,
) -> impl IntoView {
    // toast creation
    let (toast_id, set_toast_id) = create_signal(0u64);
    let (toasts, set_toasts) = create_signal(Vec::default());
    provide_context(PushToast::<M>::new(move |toast| {
        let id = toast_id.get_untracked();
        let timeout_handle = set_timeout_with_handle(
            move || {
                set_toasts.update(move |toasts| toasts.retain(|toast: &ToastWithId| toast.id != id))
            },
            toast.timeout,
        );
        let timeout_handle = if let Ok(timeout_handle) = timeout_handle {
            timeout_handle
        } else {
            tracing::error!("could not create timeout handle. toast will not be created.");
            return;
        };

        set_toasts.update(move |toasts| {
            toasts.push(ToastWithId {
                id,
                toast,
                timeout_handle,
            })
        });
        set_toast_id.update(|n| *n = n.overflowing_add(1).0);
    }));

    view! {
        {children()}
        <wu-toast-hook class="overlay flex justify-end overflow-clip">
            <ul class="h-fit divide-y divide-light-2 border border-light-2 dark:x-[divide-dark-2,border-dark-2] rounded-bl-lg shadow-lg">
                <For
                    each=toasts
                    key=move |toast| toast.id
                    children=move |toast| {
                        let id = toast.id;
                        let timeout_handle = toast.timeout_handle;
                        let class = {
                            let class = class.clone();
                            move || tw_merge!("flex flex-row vcenter gap-4 min-w-[400px] h-10 px-4 pr-2 py-1 bg-light-1 dark:bg-dark-1 last:rounded-bl-md", class.get())
                        };
                        let dismiss_btn_class = {
                            let class = dismiss_btn_class.clone();
                            move || tw_merge!("flex center text-sm font-thin rounded-full hover:bg-light-2 hover:dark:bg-dark-2 text-light-content dark:text-dark-content p-2", class.get())
                        };

                        view! {
                            <li class=class>
                                // content
                                {
                                    let msg = toast.msg.clone();
                                    move || match msg.clone() {
                                        ToastMsg::Text(text) => view! {
                                            <p class="overflow-hidden grow">{text}</p>
                                        }.into_view(),
                                        ToastMsg::View(vw) => view! {
                                            <div class="overflow-hidden grow">{vw.run()}</div>
                                        }.into_view(),
                                    }
                                }
                                // close
                                {toast.dismissable.then(move || view! {
                                    <button
                                        class=dismiss_btn_class
                                        on:click=move |_| {
                                            timeout_handle.clear();
                                            set_toasts.update(|toasts| toasts.retain(|toast| toast.id != id));
                                        }
                                    >
                                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                                        </svg>
                                    </button>
                                })}
                            </li>
                        }
                    }
                />
            </ul>
        </wu-toast-hook>
    }
}

#[derive(Clone, Deref, DerefMut)]
struct ToastWithId {
    pub id: u64,
    #[deref]
    toast: Toast,
    timeout_handle: TimeoutHandle,
}

impl PartialEq for ToastWithId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
