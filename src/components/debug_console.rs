use std::collections::VecDeque;

use leptos::*;
use tailwind_fuse::*;

use crate::components::Modal;

/// A trait that debugging commands should implement in order to execute their commands.
pub trait DebugCommand {
	type State: Clone + Copy;

	/// Parses the command.
	fn parse(s: &str) -> Result<Self, ViewFn>
	where
		Self: Sized;

	/// Executes the command.
	fn execute(self, state: &mut Self::State) -> ViewFn;
}

/// A context type used to set commands on the debug console from the outside.
pub struct DebugConsoleExternalCommand<M: 'static, T: 'static> {
	inner: WriteSignal<T>,
	phantom: std::marker::PhantomData<M>,
}

impl<M: 'static, T: 'static> DebugConsoleExternalCommand<M, T> {
	pub fn new(cmd: WriteSignal<T>) -> Self {
		Self { inner: cmd, phantom: Default::default() }
	}
}

impl<M: 'static, T: 'static> Clone for DebugConsoleExternalCommand<M, T> {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			phantom: Default::default(),
		}
	}
}

impl<M: 'static, T: 'static> Copy for DebugConsoleExternalCommand<M, T> {}

impl<M: 'static, T: 'static> std::ops::Deref for DebugConsoleExternalCommand<M, T> {
	type Target = WriteSignal<T>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<M: 'static, T: 'static> std::ops::DerefMut for DebugConsoleExternalCommand<M, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

/// Used to provide dynamic modification of arbitrary states.
#[component]
pub fn DebugConsole<M: 'static, T: DebugCommand + Clone + Default + 'static>(
	#[prop(optional)] _phant: std::marker::PhantomData<(M, T)>,
	/// The key to register for opening the console.
	#[prop(into)]
	key: std::borrow::Cow<'static, str>,
	/// The state to provide to [`DebugCommand`]s.
	mut state: <T as DebugCommand>::State,
	/// Additional overlay view to display.
	#[prop(optional, into)]
	overlay: ViewFn,
	/// Specifies the default 'class' attribute for the default debug overlay.
	#[prop(default = "".into(), into)]
	dbg_overlay_class: TextProp,
	/// Children of the component.
	children: Children,
) -> impl IntoView {
	#[derive(Clone)]
	struct LogItem {
		id: uuid::Uuid,
		command: String,
		output: ViewFn,
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
		let open_debug_console = create_rw_signal(());
		let (cmd_text, set_cmd_text) = create_signal(String::default());
		let cmd_history: RwSignal<VecDeque<LogItem>> = create_rw_signal(VecDeque::default());
		let external_cmd: RwSignal<T> = create_rw_signal(T::default());

		provide_context(DebugConsoleExternalCommand::<M, T>::new(external_cmd.write_only()));

		// logic
		let mut submit_cmd = move || {
			let cmd_text = cmd_text.get();
			let cmd: T = match T::parse(&cmd_text) {
				Ok(cmd) => cmd,
				Err(err) => {
					cmd_history.update(move |history| {
						history.push_front(LogItem {
							id: uuid::Uuid::new_v4(),
							command: cmd_text,
							output: err,
						})
					});
					set_cmd_text.update(String::clear);
					return;
				},
			};

			let output = cmd.execute(&mut state);
			cmd_history.update(move |history| {
				history.push_front(LogItem {
					id: uuid::Uuid::new_v4(),
					command: cmd_text,
					output,
				})
			});
			set_cmd_text.update(String::clear);
		};

		let external_cmd_watcher_stop = watch(
			move || external_cmd.get(),
			move |cmd, _, _| {
				cmd.clone().execute(&mut state.clone());
			},
			false,
		);

		// - register a global keydown event listener for opening and closing the console
		let cloned_key = key.clone();
		create_effect(move |_| {
			let key = cloned_key.clone();
			let unsub = leptos_use::use_event_listener(document(), ev::keydown, move |evt| {
				if evt.key() == key {
					open_debug_console.set(());
				}
			});

			on_cleanup(move || {
				unsub();
			});
		});

		on_cleanup(move || {
			external_cmd_watcher_stop();
		});

		view! {
			{children()}
			// Watermark overlay for debug mode
			<wu-debug-console-watermark class="overlay-viewport-container p-8 z-[99]">
				<div class="overlay">
					{move || overlay.run()}
				</div>
				<div class=move || tw_merge!("overlay flex items-end justify-end opacity-75", dbg_overlay_class.get())>
					// if mobile
					<button on:click=move |_| open_debug_console.set(()) class="inline-flex desktop:hidden gap-2 vcenter p-2 rounded-lg border surface-2">
						<span class="text-xl font-bold text-red-600">"⬤"</span>
						<span class="text-xl font-bold">"In debug mode"</span>
					</button>
					// if >mobile
					<div class="hidden desktop:horizontal vcenter gap-2 py-2 px-4 rounded-lg border surface-2">
						<span class="text-xl font-bold text-red-600">"⬤"</span>
						<span class="text-xl font-bold">"In debug mode"</span>
						<span clasS="hidden desktop:block text-xl font-bold">"-"</span>
						<span class="hidden desktop:block kbd">{key.clone()}</span>
						<span class="hidden desktop:block text-xl font-bold">"for console"</span>
					</div>
				</div>
			</wu-debug-console-watermark>
			<wu-debug-console class="contents">
				<Modal class="vertical gap-2 surface-1 border shadow-lg rounded-md" signal_to_open=open_debug_console>
					<h1 class="text-2xl font-bold text-center">
						"Debugger"
					</h1>
					<div class="hdivider">
						<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="stroked size-8 opacity-75">
							<path stroke-linecap="round" stroke-linejoin="round" d="M12 12.75c1.148 0 2.278.08 3.383.237 1.037.146 1.866.966 1.866 2.013 0 3.728-2.35 6.75-5.25 6.75S6.75 18.728 6.75 15c0-1.046.83-1.867 1.866-2.013A24.204 24.204 0 0 1 12 12.75Zm0 0c2.883 0 5.647.508 8.207 1.44a23.91 23.91 0 0 1-1.152 6.06M12 12.75c-2.883 0-5.647.508-8.208 1.44.125 2.104.52 4.136 1.153 6.06M12 12.75a2.25 2.25 0 0 0 2.248-2.354M12 12.75a2.25 2.25 0 0 1-2.248-2.354M12 8.25c.995 0 1.971-.08 2.922-.236.403-.066.74-.358.795-.762a3.778 3.778 0 0 0-.399-2.25M12 8.25c-.995 0-1.97-.08-2.922-.236-.402-.066-.74-.358-.795-.762a3.734 3.734 0 0 1 .4-2.253M12 8.25a2.25 2.25 0 0 0-2.248 2.146M12 8.25a2.25 2.25 0 0 1 2.248 2.146M8.683 5a6.032 6.032 0 0 1-1.155-1.002c.07-.63.27-1.222.574-1.747m.581 2.749A3.75 3.75 0 0 1 15.318 5m0 0c.427-.283.815-.62 1.155-.999a4.471 4.471 0 0 0-.575-1.752M4.921 6a24.048 24.048 0 0 0-.392 3.314c1.668.546 3.416.914 5.223 1.082M19.08 6c.205 1.08.337 2.187.392 3.314a23.882 23.882 0 0 1-5.223 1.082" />
						</svg>
					</div>
					<div class="vertical w-full desktop:w-[600px] gap-2">
						// Text buffer
						<div class="grow surface-bg-2 rounded-md">
							<ul class="flex flex-col-reverse w-full h-full min-h-[250px] max-h-[250px] desktop:min-h-[400px] desktop:max-h-[400px] overflow-y-auto">
								<For
									each=move || cmd_history.get()
									key=move |cmd| cmd.id
									let:cmd
								>
									<li class="w-full flex flex-col font-mono border-t surface-border-3 px-2 py-2">
										<span class="font-semibold"> "> " {cmd.command} </span>
										<span> {move || cmd.output.run()} </span>
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
				</Modal>
			</wu-debug-console>
		}
	}
}
