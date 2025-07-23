use crate::prelude::*;
use wu::Drawer;

#[component]
pub fn DrawerDocs() -> impl IntoView {
    let (show_drawer, set_show_drawer) = signal(false);
    let (drawer_position, set_drawer_position) = signal(wu::DrawerPosition::Right);
    let get_drawer_position = move || {
        drawer_position.get()
    };

    view! {
        <button on:click=move |_| { 
            set_show_drawer.update(|val| *val = !*val);
        }>"Show Drawer"</button>

        <button on:click=move |_| {
            set_drawer_position.set(wu::DrawerPosition::Top);
        }>"Move drawer to Top"</button>
        
        <button on:click=move |_| {
            set_drawer_position.set(wu::DrawerPosition::Bottom);
        }>"Move drawer to Bottom"</button>

        <button on:click=move |_| {
            set_drawer_position.set(wu::DrawerPosition::Left);
        }>"Move drawer to Left"</button>

        <button on:click=move |_| {
            set_drawer_position.set(wu::DrawerPosition::Right);
        }>"Move drawer to Right"</button>

        <Drawer position=wu::DrawerPosition::Top size=800 toggle=show_drawer>
            <p>"This is a Drawer"</p>
        </Drawer>
    }
}