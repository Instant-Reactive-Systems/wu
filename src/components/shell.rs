use leptos::*;
use tailwind_fuse::*;

crate::generate_marker_signal_setter!(
    /// Pushes a new shell context.
    PushShellContext, ShellContext
);

crate::generate_marker_signal_setter!(
    /// Pops current shell context.
    PopShellContext, ()
);

/// Used to specify a composable common layout.
#[component]
pub fn Shell<M: 'static>(
    #[prop(optional)] _phant: std::marker::PhantomData<M>,
    /// Header slot.
    #[prop(optional, into)]
    header: ViewFn,
    /// Left sidebar slot.
    #[prop(optional, into)]
    left_sidebar: ViewFn,
    /// Right sidebar slot.
    #[prop(optional, into)]
    right_sidebar: ViewFn,
    /// Footer slot.
    #[prop(optional, into)]
    footer: ViewFn,
    /// Corresponds to the 'class' attribute of elements.
    #[prop(default = "".into(), into)]
    class: TextProp,
    /// List of attributes to put on the top-level of the component.
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    /// Children of the component.
    children: Children,
) -> impl IntoView {
    let (shell_cxs, set_shell_cxs) = create_signal::<Vec<ShellContext>>(vec![ShellContext {
        header: Some(header.clone()),
        left_sidebar: Some(left_sidebar.clone()),
        right_sidebar: Some(right_sidebar.clone()),
        footer: Some(footer.clone()),
    }]);
    let main_cx = move || shell_cxs.with(move |cxs| {
        // SAFETY: All of the below unwraps are guaranteed to succeed, given that
        // the initial context is always populated.
        let header = cxs.iter().rev().map(|cx| cx.header.clone()).find(Option::is_some).flatten().unwrap();
        let left_sidebar = cxs.iter().rev().map(|cx| cx.left_sidebar.clone()).find(Option::is_some).flatten().unwrap();
        let right_sidebar = cxs.iter().rev().map(|cx| cx.right_sidebar.clone()).find(Option::is_some).flatten().unwrap();
        let footer = cxs.iter().rev().map(|cx| cx.footer.clone()).find(Option::is_some).flatten().unwrap();
        
        MainShellContext {
            header,
            left_sidebar,
            right_sidebar,
            footer,
        }
    });
    provide_context(PushShellContext::<M>::new(move |cx| {
        set_shell_cxs.update(move |cxs| cxs.push(cx));
    }));
    provide_context(PopShellContext::<M>::new(move |_| {
        let last_one = shell_cxs.with_untracked(move |cxs| cxs.len() == 1);
        if last_one { return } // must not pop first shell context
        set_shell_cxs.update(move |cxs| _ = cxs.pop());
    }));

    view! {
        <wu-shell {..attrs} class=move || tw_merge!("overlay flex flex-col", class.get())>
            // Header
            {move || main_cx().header.run()}
            // center area
            <div class="grow flex flex-row">
                // LeftSidebar
                {move || main_cx().left_sidebar.run()}
                // Main content area
                <div class="grow overlay-container">
                    {children()}
                </div>
                // RightSidebar
                {move || main_cx().right_sidebar.run()}
            </div>
            // Footer
            {move || main_cx().footer.run()}
        </wu-shell>
    }
}

/// Holds all slots for a context.
#[derive(Clone)]
pub struct ShellContext {
    /// Header slot.
    pub header: Option<ViewFn>,
    /// Left sidebar slot.
    pub left_sidebar: Option<ViewFn>,
    /// Right sidebar slot.
    pub right_sidebar: Option<ViewFn>,
    /// Footer slot.
    pub footer: Option<ViewFn>,
}

/// Holds the the slots of the currently displayed shell context.
#[derive(Clone)]
struct MainShellContext {
    /// Header slot.
    pub header: ViewFn,
    /// Left sidebar slot.
    pub left_sidebar: ViewFn,
    /// Right sidebar slot.
    pub right_sidebar: ViewFn,
    /// Footer slot.
    pub footer: ViewFn,
}
