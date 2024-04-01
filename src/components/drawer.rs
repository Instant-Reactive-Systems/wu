use crate::utils::Position;
use deref_derive::{Deref, DerefMut};
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
    /// Enables an animation during opening and closing the drawer.
    #[prop(default = true, into)]
    enable_anim: bool,
    /// What side to put the drawer on.
    #[prop(optional, into)]
    position: MaybeSignal<Position>,
    /// Drawer class.
    #[prop(default = "".into(), into)]
    class: TextProp,
    /// Children of the component.
    children: Children,
) -> impl IntoView {
    let (opened, set_opened) = create_signal(false);
    provide_context(OpenDrawer::<M>::new(move |_| set_opened(true)));
    provide_context(CloseDrawer::<M>::new(move |_| set_opened(false)));

    // setup 'close-on-outside-click' functionality
    let target = create_node_ref::<html::Custom>();
    create_effect(move |_| {
        target.on_load(move |_| {
            let _ = on_click_outside(target, move |_| set_opened(false));
        });
    });

    // dynamic drawer classes
    let drawer_classes = {
        move || {
            let position_related_classes = match position() {
                Position::Left => {
                    tw_merge!(
                        "h-full w-[300px] desktop:w-[400px] justify-self-start -translate-x-[300px] desktop:-translate-x-[400px]",
                        if opened() { "!translate-x-0" } else { "" }
                    )
                }
                Position::Right => {
                    tw_merge!(
                        "h-full w-[300px] desktop:w-[400px] justify-self-end translate-x-[300px] desktop:translate-x-[400px]",
                        if opened() { "!translate-x-0" } else { "" }
                    )
                }
                Position::Top => {
                    tw_merge!(
                        "w-full h-[200px] desktop:h-[300px] self-start -translate-y-[200px] desktop:-translate-y-[300px]",
                        if opened() { "!translate-y-0" } else { "" }
                    )
                }
                Position::Bottom => {
                    tw_merge!(
                        "w-full h-[200px] desktop:h-[300px] self-end translate-y-[200px] desktop:translate-y-[300px]",
                        if opened() { "!translate-y-0" } else { "" }
                    )
                }
            };

            tw_merge!(
                "overlay motion-safe:transition-none",
                if enable_anim {
                    "transition-transform"
                } else {
                    ""
                },
                position_related_classes,
                class.get()
            )
        }
    };

    view! {
        {children()}
        <wu-drawer-hook node_ref=target class=drawer_classes>
            {move || view.run()}
        </wu-drawer-hook>
    }
}
