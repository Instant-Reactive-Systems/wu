use leptos::*;
use leptos::leptos_dom::helpers::TimeoutHandle;

use std::borrow::Cow;
use std::time::Duration;
use deref_derive::{Deref, DerefMut};

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
    #[prop(default = "".into(), into)] class: TextProp,
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
            <ul class="divide-y divide-primary-700 pointer-events-none [&>*]:pointer-events-auto">
                <For
                    each=toasts
                    key=move |toast| toast.id
                    children=move |toast| {
                        let id = toast.id;
                        let timeout_handle = toast.timeout_handle;
                        let class = {
                            let class = class.clone();
                            move || format!("flex vcenter flex-row gap-4 min-w-[400px] h-10 px-4 pr-2 py-1 bg-primary-500 last:rounded-bl-md selection:bg-surface-900/20 {}", class.get())
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
                                        class="flex center text-sm font-thin rounded-full hover:bg-surface-500/20 square-6 p-2"
                                        on:click=move |_| {
                                            timeout_handle.clear();
                                            set_toasts.update(|toasts| toasts.retain(|toast| toast.id != id));
                                        }
                                    >
                                        "✕"
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

