use crate::utils::Position;
use leptos::*;
use leptos_use::*;
use tailwind_fuse::*;

crate::generate_marker_signal_setter!(
    /// Opens the drawer.
    OpenDrawer, ()
);

crate::generate_marker_signal_setter!(
    /// Closes the drawer.
    CloseDrawer, ()
);

/// Displays an overlay panel that attaches to any side of the screen.
#[component]
pub fn DrawerHook<M: 'static>(
    #[prop(optional)] _phant: std::marker::PhantomData<M>,
    /// The contents of the drawer.
    #[prop(into)]
    view: ViewFn,
    /// What side to put the drawer on.
    #[prop(optional, into)]
    position: MaybeSignal<Position>,
    /// Drawer class.
    #[prop(default = "".into(), into)]
    class: TextProp,
    /// List of attributes to put on the top-level of the component.
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    /// Children of the component.
    children: Children,
) -> impl IntoView {
    let (opened, set_opened) = create_signal(false);
    provide_context(OpenDrawer::<M>::new(move |_| set_opened.set(true)));
    provide_context(CloseDrawer::<M>::new(move |_| set_opened.set(false)));

    // setup 'close-on-outside-click' functionality
    let target = create_node_ref::<html::Custom>();
    create_effect(move |_| {
        target.on_load(move |_| {
            let _ = on_click_outside(target, move |_| set_opened.set(false));
        });
    });

    // dynamic drawer classes
    let drawer_classes = move || {
        let position_related_classes = match position.get() {
            Position::Left => {
                tw_merge!(
                    "h-full max-h-svh w-[300px] desktop:w-[400px] justify-self-start -translate-x-[300px] desktop:-translate-x-[400px]",
                    if opened.get() { "!translate-x-0" } else { "!hidden" }
                )
            }
            Position::Right => {
                tw_merge!(
                    "h-full max-h-svh w-[300px] desktop:w-[400px] justify-self-end translate-x-[300px] desktop:translate-x-[400px]",
                    if opened.get() { "!translate-x-0" } else { "!hidden" }
                )
            }
            Position::Top => {
                tw_merge!(
                    "w-full max-w-svw h-[200px] desktop:h-[300px] self-start -translate-y-[200px] desktop:-translate-y-[300px]",
                    if opened.get() { "!translate-y-0" } else { "!hidden" }
                )
            }
            Position::Bottom => {
                tw_merge!(
                    "w-full max-w-svw h-[200px] desktop:h-[300px] self-end translate-y-[200px] desktop:translate-y-[300px]",
                    if opened.get() { "!translate-y-0" } else { "!hidden" }
                )
            }
        };

        tw_merge!(
            "overlay transition-transform motion-safe:transition-none",
            position_related_classes,
            class.get()
        )
    };

    view! {
        {children()}
        <wu-drawer-hook class="overlay-viewport-container">
            <wu-drawer {..attrs} node_ref=target class=drawer_classes>
                {move || view.run()}
            </wu-drawer>
        </wu-drawer-hook>
    }
}
