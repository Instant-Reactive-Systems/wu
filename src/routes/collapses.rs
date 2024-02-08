use crate::components::*;
use leptos::*;

#[component]
pub fn Collapses() -> impl IntoView {
    view! {
        <div class="overlay flex center">
            <Collapse>
                <CollapseSummary slot>
                    <h1 class="font-bold text-lg">"Summary"</h1>
                </CollapseSummary>
                <CollapseContent slot class="p-2">
                    <p>
                    "
                    Lorem ipsum dolor sit, amet consectetur adipisicing elit. Quo illo autem velit sint, 
                    est blanditiis deleniti facilis architecto consectetur porro libero aliquid cumque 
                    reprehenderit ullam, molestias qui iste labore asperiores expedita. Eius asperiores 
                    delectus culpa laudantium voluptates, nesciunt eos ex, maiores, officia hic nulla 
                    iure perferendis molestiae distinctio voluptatum a. Eaque, reiciendis ea dolor 
                    consequatur blanditiis ipsum quas tenetur deserunt?
                    "
                    </p>
                </CollapseContent>
            </Collapse>
        </div>
    }
}
