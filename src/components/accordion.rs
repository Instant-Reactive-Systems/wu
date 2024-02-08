use leptos::*;

/// A collapsible section.
#[slot]
pub struct AccordionItem {
    /// Corresponds to the 'class' attribute of elements.
    #[prop(default = "".into(), into)]
    pub class: TextProp,
    /// Summary class.
    #[prop(default = "".into(), into)]
    pub summary_class: TextProp,
    /// Summary slot.
    #[prop(into)] pub summary: ViewFn,
    /// Content class.
    #[prop(default = "".into(), into)]
    pub content_class: TextProp,
    /// Content slot.
    #[prop(into)] pub content: ViewFn,
}

/// Divides content into collapsible sections.
#[component]
pub fn Accordion(
    /// Corresponds to the 'class' attribute of elements.
    #[prop(default = "".into(), into)] class: TextProp,
    /// List of attributes to put on the top-level of the slot.
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    /// Accordion item slots.
    accordion_item: Vec<AccordionItem>,
) -> impl IntoView {
    view! {
        <div {..attrs} class=move || format!("flex flex-col {}", class.get())>
            {accordion_item.into_iter().map(move |item| {
                let (collapsed, set_collapsed) = create_signal(true);

                view! {
                    <div class=move || format!("flex flex-col {}", item.class.get())>
                        // Summary
                        <button
                            on:click=move |_| set_collapsed.update(move |x| *x = !*x)
                            class=move || format!("w-full {}", item.summary_class.get())
                        >
                            {move || item.summary.run()}
                        </button>
                        // Content
                        <div
                            class=move || format!("w-full {}", item.content_class.get())
                            class=("hidden", collapsed)
                        >
                            {move || item.content.run()}
                        </div>
                    </div>
                }.into_view()
            }).collect::<Vec<_>>()}
        </div>
    }
}
