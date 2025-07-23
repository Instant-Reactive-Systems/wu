use crate::{prelude::*, ui::docs::{self, DropdownDocs, InputCodeDocs, ModalDocs}};

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="bg-blue-600 text-white p-4">"Wu Docs"</nav>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="w-64 p-4 bg-gray-100 h-screen">
            <ul>
                <li><A href="/docs/intro">"Introduction"</A></li>
            </ul>
        </aside>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! { 
        <h1 class="text-2xl">"Welcome to Wu Docs"</h1> 
        <ul>
            <li>
            <A href="/action_button">"ActionButton"</A>
            </li>
            <li>
            <A href="/debug_console">"DebugConsole"</A>
            </li>
            <li>
            <A href="/drawer">"Drawer"</A>
            </li>
            <li>
            <A href="/dropdown">"Dropdown"</A>
            </li>
            <li>
            <A href="/input_code">"InputCode"</A>
            </li>
            <li>
            <A href="/modal">"Modal"</A>
            </li>
            <li>
            <A href="/shell">"Shell"</A>
            </li>
            <li>
            <A href="/stack_context">"StackContext"</A>
            </li>
            <li>
            <A href="/tabs">"Tabs"</A>
            </li>
            <li>
            <A href="/toasts">"Toasts"</A>
            </li>
        </ul>

    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! { <h1 class="text-red-600">"404: Page not found"</h1> }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Navbar/>
            <div class="flex">
                <Sidebar/>
                <main class="p-4 w-full">
                    <Routes fallback=NotFound>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("action_button") view=docs::ActionButtonDocs/>
                        <Route path=path!("debug_console") view=NotFound/>
                        <Route path=path!("drawer") view=docs::DrawerDocs/>
                        <Route path=path!("dropdown") view=DropdownDocs/>
                        <Route path=path!("input_code") view=InputCodeDocs/>
                        <Route path=path!("fallible_reactive_input") view=NotFound/>
                        <Route path=path!("modal") view=ModalDocs/>
                        <Route path=path!("shell") view=NotFound/>
                        <Route path=path!("stack_context") view=NotFound/>
                        <Route path=path!("tabs") view=NotFound/>
                        <Route path=path!("toasts") view=NotFound/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}