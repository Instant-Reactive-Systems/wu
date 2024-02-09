pub mod components;
pub mod utils;
pub use components::*;
pub use utils::*;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "demo")] {
        mod routes;
        use routes::*;
        use leptos::*;
        use leptos_meta::*;
        use leptos_router::*;

        /// Toast hook marker type.
        #[derive(Clone, Copy)]
        pub struct Main;

        #[component]
        pub fn App() -> impl IntoView {
            provide_meta_context();

            view! {
                <Stylesheet id="leptos" href="/pkg/wu.css"/>
                <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
                <Body class="min-h-lvh h-lvh max-h-lvh overlay-container bg-surface-800 text-surface-content"/>
                <Title text="wu demo"/>
                
                <Router>
                    <Routes>
                        <Route path="/" view=move || view! {
                            <ModalHook<Main> class="p-4 bg-surface-400 rounded-lg bg-gradient-to-tl from-blue-900 to-blue-600 border border-surface-800 shadow-lg">
                                <DrawerHook<Main>
                                    enable_anim=true
                                    position=Position::Left
                                    class="bg-surface-800 rounded-r-lg border-r border-r-surface-900 p-2"
                                    view=move || view! {
                                        <h1 class="font-bold text-xl text-center">"Drawer header"</h1>
                                        <p>
                                        "
                                        Lorem ipsum dolor, sit amet consectetur adipisicing elit. Saepe reiciendis quis accusantium assumenda 
                                        consequuntur quam eos velit dignissimos earum, quod asperiores rerum hic odit porro consequatur. 
                                        Voluptate nulla harum, provident nostrum facilis illum saepe adipisci eum, cum nemo nobis quibusdam ipsum perspiciatis 
                                        autem nihil voluptatum blanditiis praesentium non dignissimos. Magni sapiente qui ipsum cupiditate illo facilis 
                                        inventore dolores tenetur sint.
                                        "
                                        </p>
                                    }
                                >
                                    <ToastHook<Main>>
                                        <Shell<Main>
                                            header=ViewFn::from(move || view! { <div class="flex-none bg-surface-500 w-full h-8"/> })
                                            footer=ViewFn::from(move || view! { <div class="flex-none bg-surface-400 w-full h-8"/> })
                                        >
                                            <Outlet />
                                        </Shell<Main>>
                                    </ToastHook<Main>>
                                </DrawerHook<Main>>
                            </ModalHook<Main>>
                        }>
                            <Route path="toasts" view=move || view! {<Toasts /> } />
                            <Route path="modals" view=move || view! {<Modals /> } />
                            <Route path="collapses" view=move || view! {<Collapses /> } />
                            <Route path="drawers" view=move || view! {<Drawers /> } />
                            <Route path="shells" view=move || view! {<ShellsRoute /> } />
                            <Route path="tabs" view=move || view! {<TabsRoute /> } />
                            <Route path="accordion" view=move || view! {<AccordionRoute /> } />
                            <Route path="" view=move || view! {<Home /> }/>
                        </Route>
                    </Routes>
                </Router>
            }
        }

        cfg_if! {
            if #[cfg(feature = "hydrate")] {
                use wasm_bindgen::prelude::wasm_bindgen;

                #[wasm_bindgen]
                pub fn hydrate() {
                    // initializes logging using the `log` crate
                    _ = console_log::init_with_level(log::Level::Debug);
                    console_error_panic_hook::set_once();

                    tracing::info!("hydrate mode - hydrating");

                    leptos::mount_to_body(move || {
                        view! {<App/> }
                    });
                }
            }
        }
    }
}
