use leptos::*;
use std::rc::Rc;
use deref_derive::{Deref, DerefMut};

crate::generate_marker_signal_setter!(
    /// Pushes a new shell context.
    PushShellContext, ShellContext
);

crate::generate_marker_signal_setter!(
    /// Pops current shell context.
    PopShellContext, ()
);

crate::generate_marker_bootleg_read_signal!(
    /// Gets the current active shell context.
    ActiveShellContext, ShellContext
);

/// Used to specify a composable common layout.
#[component]
pub fn ShellHook<M: 'static>(
    #[prop(optional)] _phant: std::marker::PhantomData<M>,
    /// Corresponds to the 'class' attribute of elements.
	#[prop(default = "".into(), into)] class: TextProp,
    /// Header slot.
	#[prop(optional, into)] header: ViewFn,
    /// Left sidebar slot.
	#[prop(optional, into)] left_sidebar: ViewFn,
    /// Right sidebar slot.
	#[prop(optional, into)] right_sidebar: ViewFn,
    /// Footer slot.
	#[prop(optional, into)] footer: ViewFn,
    /// Children of the component.
	children: Children,
) -> impl IntoView {
    let (shell_cxs, set_shell_cxs) = create_signal::<Vec<ShellContext>>(vec![ShellContext {
        header: header.clone(),
        left_sidebar: left_sidebar.clone(),
        right_sidebar: right_sidebar.clone(),
        footer: footer.clone(),
    }]);
    let active_cx = Rc::new(move || shell_cxs.with(move |cxs| {
        let id = cxs.len().saturating_sub(1);
        cxs[id].clone()
    }));
    provide_context(PushShellContext::<M>::new(move |cx| {
        set_shell_cxs.update(move |cxs| cxs.push(cx));
        tracing::info!("pushing shell cx");
    }));
    provide_context(PopShellContext::<M>::new(move |_| {
        tracing::info!("popping shell cx");
        if shell_cxs.with(move |cxs| cxs.len() > 1) {
            set_shell_cxs.update(move |cxs| _ = cxs.pop());
        }
    }));
    provide_context(ActiveShellContext::<M>::new(active_cx.clone()));

	view! {
        <div class="overlay overlay-container">
            <div class=move || format!("flex flex-col {}", class.get())>
                // Header
                {let cx = active_cx.clone(); move || cx().header.run()}
                // center area
                <div class="grow flex flex-row">
                    // LeftSidebar
                    {let cx = active_cx.clone(); move || cx().left_sidebar.run()}
                    // Main content area
                    <div class="grow overlay-container">
                        {children()}
                    </div>
                    // RightSidebar
                    {let cx = active_cx.clone(); move || cx().right_sidebar.run()}
                </div>
                // Footer
                {let cx = active_cx.clone(); move || cx().footer.run()}
            </div>
        </div>
	}
}

/// Holds all slots for a context.
#[derive(Clone)]
pub struct ShellContext {
    /// Header slot.
    pub header: ViewFn,
    /// Left sidebar slot.
    pub left_sidebar: ViewFn,
    /// Right sidebar slot.
    pub right_sidebar: ViewFn,
    /// Footer slot.
    pub footer: ViewFn,
}

