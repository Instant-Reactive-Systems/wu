use std::collections::VecDeque;

use leptos::{ev, prelude::*, text_prop::TextProp};
use tailwind_fuse::*;

use crate::components::Modal;

/// A trait that debugging commands should implement in order to execute their commands.
pub trait DebugCommand {
	type State: Clone;

	/// Parses the command.
	fn parse(s: &str) -> Result<Self, ViewFn>
	where
		Self: Sized;

	/// Executes the command.
	fn execute(self, state: Self::State) -> ViewFn;
}

/// A context type used to set commands on the debug console from the outside.
pub struct DebugConsoleExternalCommand<M: Send + Sync + 'static, T: Send + Sync + 'static> {
	inner: WriteSignal<T>,
	phantom: std::marker::PhantomData<M>,
}

impl<M: Send + Sync + 'static, T: Send + Sync + 'static> DebugConsoleExternalCommand<M, T> {
	pub fn new(cmd: WriteSignal<T>) -> Self {
		Self { inner: cmd, phantom: Default::default() }
	}
}

impl<M: Send + Sync + 'static, T: Send + Sync + 'static> Clone for DebugConsoleExternalCommand<M, T> {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone(),
			phantom: Default::default(),
		}
	}
}

impl<M: Send + Sync + 'static, T: Send + Sync + 'static> Copy for DebugConsoleExternalCommand<M, T> {}

impl<M: Send + Sync + 'static, T: Send + Sync + 'static> std::ops::Deref for DebugConsoleExternalCommand<M, T> {
	type Target = WriteSignal<T>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<M: Send + Sync + 'static, T: Send + Sync + 'static> std::ops::DerefMut for DebugConsoleExternalCommand<M, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

/// Used to provide dynamic modification of arbitrary states.
#[component]
pub fn DebugConsole<M, T>(
	#[prop(optional)] _phant: std::marker::PhantomData<(M, T)>,
	/// The key to register for opening the console.
	#[prop(into)]
	key: std::borrow::Cow<'static, str>,
	/// The state to provide to [`DebugCommand`]s.
	state: <T as DebugCommand>::State,
	/// Additional overlay view to display.
	#[prop(optional, into)]
	overlay: ViewFn,
	/// Specifies the default 'class' attribute for the default debug overlay.
	#[prop(default = "".into(), into)]
	dbg_overlay_class: TextProp,
	/// Children of the component.
	children: Children,
) -> impl IntoView
where
	M: Send + Sync + 'static,
	T: Send + Sync + DebugCommand + Clone + Default + 'static,
	<T as DebugCommand>::State: Send + Sync + 'static,
{
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
		let toggle_debug_console = RwSignal::new(false);
		let cmd_text = RwSignal::new(String::default());
		let cmd_history: RwSignal<VecDeque<LogItem>> = RwSignal::new(VecDeque::default());
		let external_cmd: RwSignal<T> = RwSignal::new(T::default());

		provide_context(DebugConsoleExternalCommand::<M, T>::new(external_cmd.write_only()));

		// logic
		let mut submit_cmd = Callback::new({
			let state = state.clone();
			move |_| {
				let cmd = cmd_text.get();
				let exe: T = match T::parse(&cmd) {
					Ok(cmd) => cmd,
					Err(err) => {
						cmd_history.update(move |history| {
							history.push_front(LogItem {
								id: uuid::Uuid::new_v4(),
								command: cmd,
								output: err,
							})
						});
						cmd_text.update(String::clear);
						return;
					},
				};

				let output = exe.execute(state.clone());
				cmd_history.update(move |history| {
					history.push_front(LogItem {
						id: uuid::Uuid::new_v4(),
						command: cmd,
						output,
					})
				});
				cmd_text.update(String::clear);
			}
		});

		Effect::watch(
			move || external_cmd.get(),
			{
				let state = state.clone();
				move |cmd: &T, _, _| {
					cmd.clone().execute(state.clone());
				}
			},
			false,
		);

		// - register a global keydown event listener for opening and closing the console
		let cloned_key = key.clone();
		Effect::new(move |_| {
			let key = cloned_key.clone();
			let unsub = leptos_use::use_event_listener(document(), ev::keydown, move |evt| {
				if evt.key() == key {
					toggle_debug_console.set(true);
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
				<div class="overlay">
					{move || overlay.run()}
				</div>
				<div class=move || tw_merge!("overlay flex items-end justify-end opacity-75", dbg_overlay_class.get())>
					// if mobile
					<button on:click=move |_| toggle_debug_console.set(true) class="inline-flex desktop:hidden gap-2 vcenter p-2 rounded-lg border surface-2">
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
				<Modal class="vertical gap-2 surface-1 border shadow-lg rounded-md" toggle=toggle_debug_console>
					<h1 class="text-2xl font-bold text-center">
						"Debugger"
					</h1>
					<div class="hdivider">
						<span class="icon i-o-console"/>
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
								on:input=move |ev| cmd_text.set(event_target_value(&ev))
								on:keyup=move |ev| {
									if ev.key() == "Enter" {
										submit_cmd.run(());
									}
								}
								prop:value=cmd_text
							/>
							<button class="size-12 grid center btn-square btn-primary rounded-r-md" on:click=move |_| submit_cmd.run(())>
								<span class="icon i-o-cog icon-primary-content"/>
							</button>
						</div>
					</div>
				</Modal>
			</wu-debug-console>
		}
	}
}
