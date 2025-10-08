use leptos::prelude::*;
use web_sys::wasm_bindgen::UnwrapThrowExt;

const THEME_NAME: &'static str = "wu-theme";
const THEME_EVENT_CHANGE_NAME: &'static str = "wu-theme-change-notification";

/// All possible theme options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Theme {
	/// Use `prefers-color-scheme` to dictate the theme.
	Auto,
	/// Force a light theme.
	Light,
	/// Force a dark theme.
	Dark,
}

/// Type alias for a reactive theme.
pub type ReactiveTheme = ReadSignal<Theme>;

/// Changes the theme given browser preference or explicit selection.
pub fn change_theme(theme: Theme) {
	let theme_str = serde_json::to_string(&theme).expect("should always be valid");
	let custom_event_init = web_sys::CustomEventInit::new();
	custom_event_init.set_detail(&theme_str.into());
	let custom_event = leptos::ev::CustomEvent::new_with_event_init_dict(THEME_EVENT_CHANGE_NAME, &custom_event_init).expect("should pass always");
	match window().dispatch_event(&custom_event) {
		Ok(success) => {
			if success {
				log::trace!("sent `{THEME_EVENT_CHANGE_NAME}` event");
			} else {
				log::error!("failed to send `{THEME_EVENT_CHANGE_NAME}` event");
			}
		},
		Err(err) => {
			log::error!("failed to dispatch `{THEME_EVENT_CHANGE_NAME}` event: {err:?}");
		},
	}
}

/// Provides the active theme as a signal and sets up reactive updates from [`change_theme`].
pub fn provide_theme_context() {
	let initial_theme = get_theme_from_local_storage();
	set_theme_to_local_storage(initial_theme);
	let theme = RwSignal::new(initial_theme);

	// handle programmatic change of theme
	let custom_event = leptos::ev::Custom::<leptos::ev::CustomEvent>::new(THEME_EVENT_CHANGE_NAME);
	_ = leptos_use::use_event_listener(leptos_use::use_window(), custom_event, move |data| {
		let new_theme: Theme = match serde_json::from_str(&data.detail().as_string().unwrap()) {
			Ok(theme) => theme,
			Err(err) => {
				log::error!("could not parse `{THEME_NAME}`: {err}");
				return;
			},
		};

		set_theme_to_local_storage(new_theme);
		theme.set(new_theme);
	});
	// handle external modification of localStorage
	leptos_use::use_interval_fn(
		move || {
			let storage_theme = get_theme_from_local_storage();
			let prev_theme = theme.get_untracked();
			if storage_theme != prev_theme {
				theme.set(storage_theme);
			}
		},
		1000,
	);

	// apply the class document-wide
	Effect::new(move |_| {
		let theme = theme.get();
		_ = document()
			.document_element()
			.expect("should be present always here")
			.class_list()
			.toggle_with_force("dark", {
				match theme {
					Theme::Auto => window()
						.match_media("(prefers-color-scheme: dark)")
						.expect_throw("should never fail")
						.expect_throw("should be Some")
						.matches(),
					Theme::Light => false,
					Theme::Dark => true,
				}
			})
			.expect_throw("should never fail");

		log::trace!("changed theme to `{theme:?}`");
	});

	provide_context(theme.read_only());
}

fn get_theme_from_local_storage() -> Theme {
	match window().local_storage() {
		Ok(storage) => match storage {
			Some(storage) => match storage.get_item(THEME_NAME) {
				Ok(item) => match item {
					Some(item) => match serde_json::from_str::<'_, Theme>(&item) {
						Ok(theme) => return theme,
						Err(err) => log::error!("could not parse `{THEME_NAME}`: {err}"),
					},
					None => log::error!("could not get `{THEME_NAME}` from local storage"),
				},
				Err(err) => log::error!("error while getting `{THEME_NAME}` from local storage: {err:?}"),
			},
			None => log::error!("could not get local storage"),
		},
		Err(err) => log::error!("error while getting local storage: {err:?}"),
	};

	Theme::Auto
}

fn set_theme_to_local_storage(new_theme: Theme) {
	match window().local_storage() {
		Ok(storage) => match storage {
			Some(storage) => match storage.set_item(THEME_NAME, &serde_json::to_string(&new_theme).expect("should always be valid")) {
				Ok(..) => log::trace!("successfully set `{THEME_NAME}` to local storage"),
				Err(err) => log::error!("error while setting `{THEME_NAME}` to local storage: {err:?}"),
			},
			None => log::error!("could not get local storage"),
		},
		Err(err) => log::error!("error while getting local storage: {err:?}"),
	};
}
