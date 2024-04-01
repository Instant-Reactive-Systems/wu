use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "demo")] {
        cfg_if! {
            if #[cfg(feature = "ssr")] {
                use axum::{
                    body::Body,
                    extract::State,
                    response::IntoResponse,
                    http::{Request, Response, StatusCode, Uri},
                };
                use axum::response::Response as AxumResponse;
                use tower::ServiceExt;
                use tower_http::services::ServeDir;
                use leptos::*;
                use wu::App;

                pub async fn file_and_error_handler(uri: Uri, State(options): State<LeptosOptions>, req: Request<Body>) -> AxumResponse {
                    let root = options.site_root.clone();
                    let res = get_static_file(uri.clone(), &root).await.unwrap();

                    if res.status() == StatusCode::OK {
                        res.into_response()
                    } else {
                        let handler = leptos_axum::render_app_to_stream(options.to_owned(), App);
                        handler(req).await.into_response()
                    }
                }

                async fn get_static_file(
                    uri: Uri,
                    root: &str,
                ) -> Result<Response<Body>, (StatusCode, String)> {
                    let req = Request::builder()
                        .uri(uri.clone())
                        .body(Body::empty())
                        .unwrap();
                    // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
                    // This path is relative to the cargo root
                    match ServeDir::new(root).oneshot(req).await {
                        Ok(res) => Ok(res.into_response()),
                        Err(err) => Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Something went wrong: {err}"),
                        )),
                    }
                }

                #[tokio::main]
                async fn main() {
                    use axum::{routing::post, Router};
                    use leptos_axum::{generate_route_list, LeptosRoutes};
                    use wu::App;
                    use leptos::*;

                    simple_logger::init_with_level(log::Level::Info)
                        .expect("couldn't initialize logging");

                    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
                    // For deployment these variables are:
                    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
                    // Alternately a file can be specified such as Some("Cargo.toml")
                    // The file would need to be included with the executable when moved to deployment
                    let conf = get_configuration(None).await.unwrap();
                    let addr = conf.leptos_options.site_addr;
                    let leptos_options = conf.leptos_options;
                    // Generate the list of routes in your Leptos App
                    let routes = generate_route_list(App);

                    // build our application with a route
                    let app = Router::new()
                        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
                        .leptos_routes(&leptos_options, routes, App)
                        .fallback(file_and_error_handler)
                        .with_state(leptos_options);

                    // run our app with hyper
                    // `axum::Server` is a re-export of `hyper::Server`
                    tracing::info!("listening on http://{}", &addr);
                    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
                    axum::serve(listener, app.into_make_service()).await.unwrap();
                }
            } else if #[cfg(feature = "csr")] {
                use wasm_bindgen::prelude::wasm_bindgen;

                #[wasm_bindgen(start)]
                pub fn main() {
                    use app::*;
                    use leptos::*;
                    _ = console_log::init_with_level(log::Level::Debug);
                    console_error_panic_hook::set_once();

                    tracing::info!("csr mode - mounting to body");

                    mount_to_body(App);
                }
            } else {
                pub fn main() {}
            }
        }
    } else {
        pub fn main() {}
    }
}
