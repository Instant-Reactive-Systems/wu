use leptos::*;

/// A wrapper around a `<select>` and `<option>` that automatically
/// interactivity automatically.
///
/// # Example
/// ```rust,ignore
/// let a = create_rw_signal(42u32);
/// let b = create_rw_signal(72u32);
/// let fetch_epic_data = create_action(move |(a, b)| {...});
/// <ActionButton
///     action=fetch_epic_data
///     input=move || (a.get(), b.get())
///     idle_view=move || view! {...}
///     pending_view=move || view! {...}
///     finished_view=move || view! {...}
///     class="btn btn-primary"
///  />
/// ```
#[component]
pub fn ActionButton<I: 'static, O: 'static>(
	/// The type of the button.
	#[prop(default = "button".into(), into)]
	r#type: std::borrow::Cow<'static, str>,
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
	finished_view: ViewFn,
	/// Logic to run after the finished state.
	#[prop(default = (|_| ()).into(), into)]
	on_finish: Callback<(), ()>,
	/// How long the finished state will last for.
	#[prop(into)]
	finished_lasts_for: f64,
	/// Specifies the default 'class' attribute for all modals.
	#[prop(default = "".into(), into)]
	class: TextProp,
	/// List of attributes to put on the top-level of the component.
	#[prop(attrs)]
	attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
	// types
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	enum State {
		Idle,
		Pending,
		Finished,
	}

	// vars
	let state = create_rw_signal(State::Idle);
	let leptos_use::UseTimeoutFnReturn { start, stop, is_pending, .. } = leptos_use::use_timeout_fn(
		move |_| {
			on_finish.call(());
			state.update(move |state| *state = State::Idle);
		},
		finished_lasts_for,
	);

	// logic
	_ = watch(
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
			{..attrs}
			type=r#type
			on:click=move |_| action.dispatch(input.call(()))
			disabled=move || state.get() != State::Idle
			class=class
		>
			{move || match state.get() {
				State::Idle => idle_view.run(),
				State::Pending => pending_view.run(),
				State::Finished => finished_view.run(),
			}}
		</button>
	}
}
