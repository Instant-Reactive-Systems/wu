use leptos::*;

/// Summary section of the collapse component.
#[slot]
pub struct CollapseSummary {
    /// Corresponds to the 'class' attribute of elements.
    #[prop(default = "".into(), into)]
    pub class: TextProp,
    /// Children of the slot.
    pub children: ChildrenFn,
}

/// Content section of the collapse component.
#[slot]
pub struct CollapseContent {
    /// Corresponds to the 'class' attribute of elements.
    #[prop(default = "".into(), into)]
    pub class: TextProp,
    /// Children of the slot.
    pub children: ChildrenFn,
}

/// Used for showing and hiding content.
#[component]
pub fn Collapse(
    /// Corresponds to the 'class' attribute of elements.
    #[prop(default = "".into(), into)] class: TextProp,
    /// List of attributes to put on the top-level of the slot.
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    /// Whether to create the component with an initially opened state.
    #[prop(default = false)] opened: bool,
    /// Summary slot.
    collapse_summary: CollapseSummary,
    /// Content slot.
    collapse_content: CollapseContent,
) -> impl IntoView {
    let (collapsed, set_collapsed) = create_signal(!opened);

    view! {
        <div {..attrs} class=move || format!("flex flex-col {}", class.get())>
            // <Summary>
            <button
                on:click=move |_| set_collapsed.update(move |x| *x = !*x)
                class=move || format!("w-full {}", collapse_summary.class.get())
            >
                {(collapse_summary.children)().into_view()}
            </button>
            // <Content>
            <div
                class=move || format!("w-full {}", collapse_content.class.get())
                class=("hidden", collapsed)
            >
                {(collapse_content.children)().into_view()}
            </div>
        </div>
    }
}
