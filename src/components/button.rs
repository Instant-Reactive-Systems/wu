use std::time::Duration;

use leptos::prelude::*;

/// A wrapper around a `<button>` that automatically
/// handles actions.
///
/// # Example
/// ```rust,ignore
/// let a = RwSignal::new(42u32);
/// let b = RwSignal::new(72u32);
/// let fetch_epic_data = Action::new(move |(a, b)| {...});
/// <ActionButton
///     action=fetch_epic_data
///     input=move || (a.get(), b.get())
///     idle_view=move || view! {...}
///     pending_view=move || view! {...}
///     finished_view=move || view! {...}
///     attr:class="btn btn-primary"
///  />
/// ```
#[component]
pub fn ActionButton<I, O>(
	/// Action to dispatch and to await.
	action: Action<I, O>,
	/// Input to the action.
	#[prop(into)]
	input: Callback<(), I>,
	/// View to display during idle state.
	#[prop(optional, into)]
	idle_view: ViewFn,
	/// View to display during pending state.
	#[prop(optional, into)]
	pending_view: ViewFn,
	/// View to display during finished state.
	#[prop(optional, into)]
	finished_view: crate::utils::ViewFnWithArgs<O>,
	/// Logic to run after the finished state.
	#[prop(default=(|_: O| ()).into(), into)]
	on_finish: Callback<(O,), ()>,
	/// How long the finished state will last for.
	#[prop(into)]
	finished_lasts_for: Duration,
) -> impl IntoView
where
	I: Send + Sync + 'static,
	O: Clone + Send + Sync + 'static,
{
	// types
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	enum State {
		Idle,
		Pending,
		Finished,
	}

	// vars
	let state = RwSignal::new(State::Idle);
	let leptos_use::UseTimeoutFnReturn { start, stop, is_pending, .. } = leptos_use::use_timeout_fn(
		move |_| {
			on_finish.run((action.value().get_untracked().as_ref().cloned().expect("should be Some"),));
			state.update(move |state| *state = State::Idle);
		},
		finished_lasts_for.as_millis() as f64,
	);

	// logic
	Effect::watch(
		move || action.pending().get(),
		move |curr, past, _| {
			if is_pending.get_untracked() {
				stop();
			}

			if let Some(past) = past {
				match (curr, past) {
					(true, false) => state.update(move |state| *state = State::Pending),
					(false, true) => {
						state.update(move |state| *state = State::Finished);
						start(());
					},
					_ => unreachable!("this type of state should be impossible to occur"),
				}
			} else {
				state.update(move |state| *state = State::Pending);
			}
		},
		false,
	);

	view! {
		<button
			on:click=move |_| _ = action.dispatch(input.run(()))
			disabled=move || state.get() != State::Idle
		>
			{move || match state.get() {
				State::Idle => idle_view.run(),
				State::Pending => pending_view.run(),
				State::Finished => finished_view.run(action.value().get_untracked().as_ref().cloned().expect("should be Some")),
			}}
		</button>
	}
}
