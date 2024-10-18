use leptos::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct DebugCommandParseError {
	pub what: String,
	pub span: (usize, usize),
}

impl std::fmt::Display for DebugCommandParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		<Self as std::fmt::Debug>::fmt(self, f)
	}
}

impl DebugCommandParseError {
	/// Creates a new [`DebugCommandParseError`].
	pub fn new(what: impl AsRef<str>, span: (usize, usize)) -> Self {
		Self { what: String::from(what.as_ref()), span }
	}
}

/// A trait that debugging commands should implement in order to execute their commands.
pub trait DebugCommand {
	type State: Clone + Copy;

	/// Parses the command.
	fn parse(s: &str) -> Result<Self, DebugCommandParseError>
	where
		Self: Sized;

	/// Executes the command.
	fn execute(self, state: &mut Self::State) -> String;
}

/// Used to provide dynamic modification of arbitrary states.
#[component]
pub fn DebugConsole<M: 'static, T: DebugCommand + 'static>(
	#[prop(optional)] _phant: std::marker::PhantomData<(M, T)>,
	/// The key to register for opening the console.
	#[prop(into)]
	key: std::borrow::Cow<'static, str>,
	/// The state to provide to [`DebugCommand`]s.
	mut state: <T as DebugCommand>::State,
	/// Children of the component.
	children: Children,
) -> impl IntoView {
	#[derive(Debug, Clone)]
	struct LogItem {
		id: uuid::Uuid,
		command: String,
		text: String,
	}

	#[cfg(not(debug_assertions))]
	{
		view! {
			{children()}
		}
	}

	#[cfg(debug_assertions)]
	{
		// vars
		let dialog_ref = create_node_ref::<html::Dialog>();
		let (cmd_text, set_cmd_text) = create_signal(String::default());
		let cmd_history: RwSignal<VecDeque<LogItem>> = create_rw_signal(VecDeque::default());

		// logic
		let mut submit_cmd = move || {
			let cmd_text = cmd_text.get();
			let cmd: T = match T::parse(&cmd_text) {
				Ok(cmd) => cmd,
				Err(err) => {
					log::error!("failed to parse the command: {}", err);
					cmd_history.update(move |history| {
						history.push_front(LogItem {
							id: uuid::Uuid::new_v4(),
							command: cmd_text,
							text: err.to_string(),
						})
					});
					set_cmd_text.update(String::clear);
					return;
				},
			};

			let output = cmd.execute(&mut state);
			log::info!("executed command '{}' with output: {}", cmd_text, output);
			cmd_history.update(move |history| {
				history.push_front(LogItem {
					id: uuid::Uuid::new_v4(),
					command: cmd_text,
					text: output,
				})
			});
			set_cmd_text.update(String::clear);
		};

		// - register a global keydown event listener for opening and closing the console
		let cloned_key = key.clone();
		dialog_ref.on_load(move |dialog| {
			let unsub = leptos_use::use_event_listener(document(), ev::keydown, move |evt| {
				if evt.key() == cloned_key {
					if dialog.open() {
						dialog.close();
					} else {
						dialog.show_modal();
					}
				}
			});

			on_cleanup(move || {
				unsub();
			});
		});

		view! {
			{children()}
			// Watermark overlay for debug mode
			<wu-debug-console-watermark class="overlay-viewport-container p-8 z-[99]">
				<div class="overlay flex items-end justify-end opacity-75">
					<div class="horizontal vcenter gap-2 py-2 px-4 rounded-lg border border-2 surface-2">
						<span class="text-xl font-bold text-red-600">"⬤"</span>
						<span class="text-xl font-bold">"In debug mode -"</span>
						<span class="kbd">{key.clone()}</span>
						<span class="text-xl font-bold">"for console"</span>
					</div>
				</div>
			</wu-debug-console-watermark>
			<wu-debug-console class="contents">
				<dialog _ref=dialog_ref class="w-lvw h-lvh">
					<div class="overlay-viewport-container p-8">
						// Watermark overlay when in modal mode
						<wu-debug-console-watermark class="overlay flex items-end justify-end opacity-75">
							<div class="horizontal vcenter gap-2 py-2 px-4 rounded-lg border border-2 surface-2">
								<span class="text-xl font-bold text-red-600">"⬤"</span>
								<span class="text-xl font-bold">"In debug mode -"</span>
								<span class="kbd">{key.clone()}</span>
								<span class="text-xl font-bold">"for console"</span>
							</div>
						</wu-debug-console-watermark>
						<div class="overlay flex center">
							<div class="overlay-container">
								// Content
								<div class="overlay vertical gap-1 p-4 surface-1 rounded-md">
									<h1 class="text-xl font-bold text-center">
										"Debugger"
									</h1>
									<div class="hdivider divider-light-3 dark:divider-dark-3">
										<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="stroked size-8 stroke-light-3 dark:stroke-dark-3">
											<path stroke-linecap="round" stroke-linejoin="round" d="M12 12.75c1.148 0 2.278.08 3.383.237 1.037.146 1.866.966 1.866 2.013 0 3.728-2.35 6.75-5.25 6.75S6.75 18.728 6.75 15c0-1.046.83-1.867 1.866-2.013A24.204 24.204 0 0 1 12 12.75Zm0 0c2.883 0 5.647.508 8.207 1.44a23.91 23.91 0 0 1-1.152 6.06M12 12.75c-2.883 0-5.647.508-8.208 1.44.125 2.104.52 4.136 1.153 6.06M12 12.75a2.25 2.25 0 0 0 2.248-2.354M12 12.75a2.25 2.25 0 0 1-2.248-2.354M12 8.25c.995 0 1.971-.08 2.922-.236.403-.066.74-.358.795-.762a3.778 3.778 0 0 0-.399-2.25M12 8.25c-.995 0-1.97-.08-2.922-.236-.402-.066-.74-.358-.795-.762a3.734 3.734 0 0 1 .4-2.253M12 8.25a2.25 2.25 0 0 0-2.248 2.146M12 8.25a2.25 2.25 0 0 1 2.248 2.146M8.683 5a6.032 6.032 0 0 1-1.155-1.002c.07-.63.27-1.222.574-1.747m.581 2.749A3.75 3.75 0 0 1 15.318 5m0 0c.427-.283.815-.62 1.155-.999a4.471 4.471 0 0 0-.575-1.752M4.921 6a24.048 24.048 0 0 0-.392 3.314c1.668.546 3.416.914 5.223 1.082M19.08 6c.205 1.08.337 2.187.392 3.314a23.882 23.882 0 0 1-5.223 1.082" />
										</svg>
									</div>
									<div class="vertical w-full desktop:w-[600px] gap-2 overflow-y-auto">
										// Text buffer
										<div class="grow surface-bg-2 rounded-md">
											<ul class="flex flex-col-reverse w-full h-full min-h-[400px] max-h-[400px]">
												<For
													each=move || cmd_history.get()
													key=move |cmd| cmd.id
													let:cmd
												>
													<li class="w-full flex flex-col font-mono border-t surface-border-3 px-2 py-2">
														<span class="font-semibold"> "> " {cmd.command} </span>
														<span> {cmd.text} </span>
													</li>
												</For>
											</ul>
										</div>
										// Input
										<div class="shrink flex flex-row vcenter">
											<input
												class="grow min-h-12 surface-bg-2 surface-2 border rounded-l-md"
												type="text"
												placeholder="type a command"
												on:input=move |ev| set_cmd_text.set(event_target_value(&ev))
												on:keyup=move |ev| {
													if ev.key() == "Enter" {
														submit_cmd();
													}
												}
												prop:value=cmd_text
											/>
											<button class="size-12 grid center btn-square btn-primary rounded-r-md" on:click=move |_| submit_cmd()>
												<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="stroked size-10">
													<path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12a7.5 7.5 0 0 0 15 0m-15 0a7.5 7.5 0 1 1 15 0m-15 0H3m16.5 0H21m-1.5 0H12m-8.457 3.077 1.41-.513m14.095-5.13 1.41-.513M5.106 17.785l1.15-.964m11.49-9.642 1.149-.964M7.501 19.795l.75-1.3m7.5-12.99.75-1.3m-6.063 16.658.26-1.477m2.605-14.772.26-1.477m0 17.726-.26-1.477M10.698 4.614l-.26-1.477M16.5 19.794l-.75-1.299M7.5 4.205 12 12m6.894 5.785-1.149-.964M6.256 7.178l-1.15-.964m15.352 8.864-1.41-.513M4.954 9.435l-1.41-.514M12.002 12l-3.75 6.495" />
												</svg>
											</button>
										</div>
									</div>
								</div>
								// Close button
								<div class="overlay p-4 flex justify-end">
									<div class="horizontal w-fit h-fit vcenter gap-2 opacity-50">
										<span class="kbd surface-2">"ESC"</span>
										<span class="text-xs">"or"</span>
										<button
											class="flex center btn-circle p-2 focus-within:bg-light-3/20 dark:focus-within:bg-dark-3/20 hover:bg-light-3/20 dark:hover:bg-dark-3/20"
											on:click=move |_| dialog_ref.get().unwrap().close()
										>
											<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
												<path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
											</svg>
										</button>
									</div>
								</div>
							</div>
						</div>
					</div>
				</dialog>
			</wu-debug-console>
		}
	}
}
