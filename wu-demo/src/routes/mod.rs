mod drawers;
mod home;
mod modals;
mod shells;
mod tabs;
mod toasts;
mod avatar;

mod routes {
	pub use super::{avatar::*, drawers::*, home::*, modals::*, shells::*, tabs::*, toasts::*};
}

use crate::prelude::*;

#[component]
fn ErrorPage(#[prop(optional)] outside_errors: Option<Errors>, #[prop(optional)] errors: Option<RwSignal<Errors>>) -> impl IntoView {
	// vars
	// - we display only the first error
	let errors = errors.map(|errors| errors()).unwrap_or(outside_errors.unwrap_or_default());
	if errors.is_empty() {
		unreachable!("should always have at least one error")
	}
	let error = errors.iter().nth(0).map(|(_, err)| err).unwrap().downcast_ref::<AppError>().cloned().unwrap();
	let code = error.code.to_string().split_ascii_whitespace().next().unwrap().to_string();

	// i18n
	let go_back = "Go back home?";

	view! {
		<div class="grow flex center">
			<main class="container space-y-2">
				<h2 class="text-xl tablet:text-3xl font-bold">{code}</h2>
				<p class="text-lg">{error.what}</p>
				<A href="/" class="link text-sm font-semibold w-fit">{go_back}</A>
			</main>
		</div>
	}
}

#[component]
pub fn AppRouter() -> impl IntoView {
	let fallback = move || {
		let mut outside_errors = Errors::default();
		outside_errors.insert_with_default_key(AppError {
			code: http::StatusCode::NOT_FOUND,
			what: "Page not found".to_string(),
		});
		view! { <ErrorPage outside_errors /> }
	};

	view! {
		<Router fallback>
			<Hooks>
				<Routes>
					<Route path="/" view=Outlet>
						<Route path="" view=routes::Home />
						<Route path="/docs" view=Outlet>
							<Route path="" view=routes::Modals />
							<Route path=":section-id" view=Outlet />
						</Route>
					</Route>
				</Routes>
			</Hooks>
		</Router>
	}
}

#[component]
fn Hooks(children: Children) -> impl IntoView {
	view! {
		<wu::ModalHook<Main>
			class="\
			border surface-1 shadow-lg rounded-lg \
			w-full m-2 p-4 \
			max-w-[400px] \
			desktop:x-[w-3/4,max-w-[600px]] \
			"
		>
			<wu::ToastHook<Main>>
				<wu::Shell<Main>
					header=AppHeader
					footer=AppFooter
				>
					<ErrorBoundary fallback=move |errors| view! { <ErrorPage errors/> }>
						{children()}
					</ErrorBoundary>
				</wu::Shell<Main>>
			</wu::ToastHook<Main>>
		</wu::ModalHook<Main>>
	}
}

#[component]
fn AppHeader() -> impl IntoView {
	// vars
	let open_drawer = expect_context::<wu::OpenDrawer<Main>>();
	let add_modal = expect_context::<wu::AddModal<Main>>();

	view! {
		<nav class="horizontal gap-4 border-b surface-2 h-12 px-4 [&>*_*]:h-full">
			// left
			<div class="grow">
				<A href="/" class="flex center">
					<Brand />
				</A>
			</div>
			// center
			// <none>
			// right
			<div class="hidden desktop:horizontal flex-1 hend gap-2">
				// links
				<ul class="horizontal gap-4">
					<li>
						<A href="/docs" active_class="link-primary" class="flex vcenter link">"Docs"</A>
					</li>
				</ul>
			</div>
		</nav>
	}
}

#[component]
fn AppFooter() -> impl IntoView {
	view! {
		<footer class="flex center border-t surface-2 h-6 tablet:h-10">
			<span class="opacity-50">"© 2024 IRS"</span>
		</footer>
	}
}
