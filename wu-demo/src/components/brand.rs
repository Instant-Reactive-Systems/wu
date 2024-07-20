use crate::prelude::*;

/// Webutils brand icon component.
#[component]
pub fn Brand(
    /// Corresponds to the `class` attribute of all elements.
    #[prop(default = "".into(), into)] class: TextProp,
    /// List of attributes to put on the top-level of the component.
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <div {..attrs} class=move || tw_merge!("text-xl tablet:text-3xl font-bold select-none", class.get())>
            "wu"
        </div>
    }
}
