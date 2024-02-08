use crate::components::*;
use leptos::*;

#[component]
pub fn AccordionRoute() -> impl IntoView {
    view! {
        <Accordion>
            <AccordionItem slot
                summary=move || view! { <h2>"Accordion item #1"</h2> }
                content=move || view! { <p>"Some content #1"</p> }
            />
            <AccordionItem slot
                summary=move || view! { <h2>"Accordion item #2"</h2> }
                content=move || view! { <p>"Some content #2"</p> }
            />
            <AccordionItem slot
                summary=move || view! { <h2>"Accordion item #3"</h2> }
                content=move || view! { <p>"Some content #3"</p> }
            />
        </Accordion>
    }
}
