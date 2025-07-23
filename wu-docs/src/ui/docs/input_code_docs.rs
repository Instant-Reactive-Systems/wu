use crate::prelude::*;
use wu::InputCode;

#[component]
pub fn InputCodeDocs() -> impl IntoView {
    let input_value = RwSignal::new(String::new());
    let req = RwSignal::new(false);

    view! {
        <InputCode 
            value=input_value
            code_length=3
            field_size=50
            field_thickness=2
            placeholder="Input text"
            required=req
        />

        <p>{move || input_value.get()}</p>
    }
}