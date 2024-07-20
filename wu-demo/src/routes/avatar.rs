use crate::components::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn AvatarRoute() -> impl IntoView {

    view! {
        <div class="flex center">
            <div class="avatar avatar-size-32 bg-[url('https://avatars.githubusercontent.com/u/69715407?s=96&v=4')]">
                <span class="avatar-slot-1 badge rounded-full bg-primary">"epic"</span>
                <span class="avatar-slot-2 badge rounded-full bg-primary">"epic"</span>
                <span class="avatar-slot-3 badge rounded-full bg-primary">"epic"</span>
                <span class="avatar-slot-4 badge rounded-full bg-primary">"epic"</span>
                <span class="avatar-slot-5 badge rounded-full bg-primary">"epic"</span>
                <span class="avatar-slot-6 badge rounded-full bg-primary">"epic"</span>
                <span class="avatar-slot-7 badge rounded-full bg-primary">"epic"</span>
                <span class="avatar-slot-8 badge rounded-full bg-primary">"epic"</span>
            </div>
        </div>
    }
}
