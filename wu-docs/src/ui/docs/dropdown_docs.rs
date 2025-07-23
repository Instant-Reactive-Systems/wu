use crate::prelude::*;
use wu::Dropdown;

#[component]
pub fn DropdownDocs() -> impl IntoView {
    let chosen_value = RwSignal::new(String::from("Default"));
    

    view! {
        <p>"Dropdown is a wrapper component of <select> bound to a signal"</p>
        <div style="font-family: sans-serif; font-size: 14px; line-height: 1.5;">
            <h3 style="margin-bottom: 0.25em;">Required Props</h3>
            <ul style="list-style: none; padding-left: 0;">
                <li>
                <strong>value</strong>: <code>impl Into&lt;RwSignal&lt;T&gt;&gt;</code><br />
                <div style="margin-left: 2em; color: gray;">Signal used for getting/setting the value.</div>
                </li>
                <li>
                <strong>items</strong>: <code>impl Into&lt;Vec&lt;T&gt;&gt;</code><br />
                <div style="margin-left: 2em; color: gray;">Possible items to choose from.</div>
                </li>
            </ul>
        </div>


        <p>"Example used below:"</p>
        <div>
            <p><code>"<Dropdown"</code></p>
            <p><code style="margin-left: 2em;">"value=chosen_value "</code></p>
            <p><code style="margin-left: 2em;">"items={vec!["</code></p>
            <p><code style="margin-left: 2em;">r##""Default".into(),"##</code></p>
            <p><code style="margin-left: 2em;">r##""Option 1".into(),"##</code></p>
            <p><code style="margin-left: 2em;">r##""Option 2".into(),"##</code></p>
            <p><code style="margin-left: 2em;">r##""Option 3".into()"##</code></p>
            <p><code style="margin-left: 2em;">"]"</code></p>
            <p><code style="margin-left: 2em;">"}"</code></p>
            <p><code></code>"/>"</p>
        </div>


        <h1>{move || chosen_value.get()}</h1>
        <Dropdown 
            value=chosen_value 
            items={vec![
                "Default".into(),
                "Option 1".into(),
                "Option 2".into(),
                "Option 3".into()
                ]
            }
        />
    }
}