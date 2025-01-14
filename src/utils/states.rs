use std::sync::Arc;

use leptos::prelude::*;
use reactive_stores::*;

crate::generate_marker_type!(
	#[doc(hidden)]
	AddGlobalStateMarker
);
crate::generate_marker_type!(
	#[doc(hidden)]
	SetActiveGlobalStateMarker
);
crate::generate_marker_type!(
	#[doc(hidden)]
	RemoveGlobalStateMarker
);

/// Adds a new global state to the context.
pub type AddGlobalState = crate::utils::Marked<AddGlobalStateMarker, Callback<()>>;
/// Sets a new active global state in the context.
pub type SetActiveGlobalState = crate::utils::Marked<SetActiveGlobalStateMarker, Callback<StateId>>;
/// Removes a global state from the context.
pub type RemoveGlobalState = crate::utils::Marked<RemoveGlobalStateMarker, Callback<StateId>>;

/// Type alias for a state ID.
pub type StateId = uuid::Uuid;

/// A hook that manages global arbitrary states.
pub struct ArcGlobalStates<T>
where
	T: Send + Sync + 'static,
{
	default_id: StateId,
	selected_id: ArcRwSignal<StateId>,
	selected_state: ArcRwSignal<ArcStore<T>>,
	states: ArcRwSignal<Vec<(StateId, (ArcStore<T>, Vec<Arc<RenderEffect<()>>>))>>,
	add_state: AddGlobalState,
	set_active_state: SetActiveGlobalState,
	remove_state: RemoveGlobalState,
}

impl<T> ArcGlobalStates<T>
where
	T: Send + Sync + 'static,
{
	/// Gets all states and their IDs reactively.
	#[track_caller]
	pub fn states(&self) -> Vec<(StateId, ArcStore<T>)> {
		self.states.get().iter().map(|(id, (state, _))| (id.clone(), state.clone())).collect()
	}

	/// Gets all states and their IDs non-reactively.
	#[track_caller]
	pub fn states_untracked(&self) -> Vec<(StateId, ArcStore<T>)> {
		self.states.get_untracked().iter().map(|(id, (state, _))| (id.clone(), state.clone())).collect()
	}

	/// Gets all IDs reactively.
	#[track_caller]
	pub fn ids(&self) -> Vec<StateId> {
		self.states.get().iter().map(|(id, _)| id).cloned().collect()
	}

	/// Gets all IDs non-reactively.
	#[track_caller]
	pub fn ids_untracked(&self) -> Vec<StateId> {
		self.states.get_untracked().iter().map(|(id, _)| id).cloned().collect()
	}

	/// Adds a new global state to the context.
	#[track_caller]
	pub fn add_state(&self) {
		self.add_state.run(())
	}

	/// Sets a new active global state in the context.
	#[track_caller]
	pub fn set_active_state(&self, id: StateId) {
		self.set_active_state.run(id)
	}

	/// Removes a global state from the context.
	#[track_caller]
	pub fn remove_state(&self, id: StateId) {
		self.remove_state.run(id)
	}

	/// Gets the default ID.
	#[track_caller]
	pub fn default_id(&self) -> StateId {
		self.default_id.clone()
	}

	/// Gets the selected ID arc signal.
	#[track_caller]
	pub fn selected_id(&self) -> ArcRwSignal<StateId> {
		self.selected_id.clone()
	}

	/// Gets the selected state arc signal.
	#[track_caller]
	pub fn selected_state(&self) -> ArcRwSignal<ArcStore<T>> {
		self.selected_state.clone()
	}
}

impl<T> Clone for ArcGlobalStates<T>
where
	T: Send + Sync + 'static,
{
	fn clone(&self) -> Self {
		Self {
			default_id: self.default_id.clone(),
			selected_id: self.selected_id.clone(),
			selected_state: self.selected_state.clone(),
			states: self.states.clone(),
			add_state: self.add_state.clone(),
			set_active_state: self.set_active_state.clone(),
			remove_state: self.remove_state.clone(),
		}
	}
}

/// A builder for [`GlobalStates`].
pub struct GlobalStateBuilder<T> {
	states: Vec<(StateId, ArcStore<T>)>,
	effects: Vec<Arc<dyn Fn(StateId, Store<T>, bool) + Send + Sync>>,
	tasks: Vec<Arc<dyn Fn(StateId, ArcStore<T>) + Send + Sync>>,
}

impl<T> GlobalStateBuilder<T>
where
	T: Default + 'static,
{
	/// Creates a new [`GlobalStateBuilder`] with a single default state.
	pub fn new() -> Self {
		Self {
			states: vec![(StateId::new_v4(), ArcStore::default())],
			effects: Default::default(),
			tasks: Default::default(),
		}
	}

	/// Creates a new [`GlobalStateBuilder`] with a predefined set of states.
	pub fn from_states(states: impl IntoIterator<Item = T>) -> Self {
		Self {
			states: {
				let states = states.into_iter().map(|state| (StateId::new_v4(), ArcStore::new(state))).collect::<Vec<_>>();
				if states.is_empty() {
					vec![(StateId::new_v4(), ArcStore::default())]
				} else {
					states
				}
			},
			effects: Default::default(),
			tasks: Default::default(),
		}
	}

	/// Creates a new [`GlobalStateBuilder`] with a predefined set of states with IDs.
	pub fn from_states_with_ids(states: impl IntoIterator<Item = (StateId, T)>) -> Self {
		Self {
			states: {
				let states = states.into_iter().map(|(id, state)| (id, ArcStore::new(state))).collect::<Vec<_>>();
				if states.is_empty() {
					vec![(StateId::new_v4(), ArcStore::default())]
				} else {
					states
				}
			},
			effects: Default::default(),
			tasks: Default::default(),
		}
	}
}

impl<T> GlobalStateBuilder<T>
where
	T: Default + Clone + Send + Sync + 'static,
{
	/// Inserts a special task to be run in an effect on the reactive store.
	pub fn with_effect<F>(mut self, f: F) -> Self
	where
		F: Fn(StateId, Store<T>, bool) + Send + Sync + 'static,
	{
		self.effects.push(Arc::new(f));
		self
	}

	/// Inserts a special task to be run when a state is constructed.
	pub fn with_task<F>(mut self, f: F) -> Self
	where
		F: Fn(StateId, ArcStore<T>) + Send + Sync + 'static,
	{
		self.tasks.push(Arc::new(f));
		self
	}

	/// Finalizes the [`GlobalStates`] object and spawns all effects required for it to function.
	pub fn finish(self) -> ArcGlobalStates<T> {
		let GlobalStateBuilder { states, effects, tasks } = self;

		// spawn effects and populate states
		let states = states
			.into_iter()
			.map(|(id, state)| {
				tasks.iter().cloned().for_each(|task| task(id.clone(), state.clone()));
				let effects = effects
					.iter()
					.cloned()
					.map(|effect| {
						let state = state.clone();
						Arc::new(RenderEffect::new(move |prev| {
							let state: Store<T> = state.clone().into();
							let has_run = prev.is_some();
							(effect.clone())(id.clone(), state, has_run)
						}))
					})
					.collect::<Vec<_>>();

				(id, (state.clone(), effects))
			})
			.collect::<Vec<_>>();
		let default_id = states.get(0).map(|(id, _)| id).cloned().expect("that there should always be at least ONE state");
		// SAFETY: The default state should ALWAYS exist.
		let default_state = states
			.get(0)
			.map(|(_, (state, _))| state)
			.cloned()
			.expect("that there should always be at least ONE state");
		let selected_id = ArcRwSignal::new(default_id.clone());
		let selected_state = ArcRwSignal::new(default_state.clone());
		let states = ArcRwSignal::new(states);

		// api
		let add_state = AddGlobalState::new(Callback::new({
			let states = states.clone();
			let effects = effects.clone();
			let tasks = tasks.clone();
			move |_| {
				let id = StateId::new_v4();
				let state = ArcStore::new(T::default());
				tasks.iter().cloned().for_each(|task| task(id.clone(), state.clone()));
				let effects = effects
					.iter()
					.cloned()
					.map(|effect| {
						let state = state.clone();
						Arc::new(RenderEffect::new(move |prev| {
							let state: Store<T> = state.clone().into();
							let has_run = prev.is_some();
							(effect.clone())(id.clone(), state, has_run)
						}))
					})
					.collect::<Vec<_>>();
				states.clone().write().push((id, (state, effects)));
			}
		}));
		let set_active_state = SetActiveGlobalState::new(Callback::new({
			let selected_id = selected_id.clone();
			let selected_state = selected_state.clone();
			let states = states.clone();
			let default_id = default_id.clone();
			let default_state = default_state.clone();
			move |id| {
				// check if the state exists before assigning it
				match states
					.clone()
					.get()
					.iter()
					.find(|(state_id, _)| id == *state_id)
					.map(|(_, (state, _))| state)
					.cloned()
				{
					Some(state) => {
						selected_id.clone().set(id);
						selected_state.clone().set(state);
					},
					None => {
						selected_id.clone().set(default_id.clone());
						selected_state.clone().set(default_state.clone());
					},
				};
			}
		}));
		let remove_state = RemoveGlobalState::new(Callback::new({
			let default_id = default_id.clone();
			let selected_id = selected_id.clone();
			let states = states.clone();
			move |id| {
				if default_id == id {
					return;
				}
				if selected_id.clone().get() == id {
					selected_id.clone().set(id);
				}
				states.clone().write().retain(|(other_id, _)| *other_id != id);
			}
		}));

		ArcGlobalStates {
			default_id,
			selected_id,
			selected_state,
			states,
			add_state,
			set_active_state,
			remove_state,
		}
	}
}

impl<T> Clone for GlobalStateBuilder<T>
where
	T: Send + Sync + 'static,
{
	fn clone(&self) -> Self {
		Self {
			states: self.states.clone(),
			effects: self.effects.clone(),
			tasks: self.tasks.clone(),
		}
	}
}

/// A `Copy`-able version of [`ArcGlobalStates`].
pub struct GlobalStates<T>(ArenaItem<ArcGlobalStates<T>>)
where
	T: Send + Sync + 'static;

impl<T> std::convert::From<ArcGlobalStates<T>> for GlobalStates<T>
where
	T: Send + Sync + 'static,
{
	fn from(value: ArcGlobalStates<T>) -> Self {
		Self(ArenaItem::new_with_storage(value))
	}
}

impl<T> Clone for GlobalStates<T>
where
	T: Send + Sync + 'static,
{
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T> Copy for GlobalStates<T> where T: Send + Sync + 'static {}

impl<T> GlobalStates<T>
where
	T: Send + Sync + 'static,
{
	/// Returns the inner stored arc.
	#[track_caller]
	pub fn into_arc(&self) -> ArcGlobalStates<T> {
		self.0.try_get_value().clone().expect("should not be disposed")
	}

	/// Gets all states and their IDs reactively.
	#[track_caller]
	pub fn states(&self) -> Vec<(StateId, ArcStore<T>)> {
		self.0.try_get_value().map(|states| states.states()).expect("should not be disposed")
	}

	/// Gets all states and their IDs non-reactively.
	#[track_caller]
	pub fn states_untracked(&self) -> Vec<(StateId, ArcStore<T>)> {
		self.0.try_get_value().map(|states| states.states_untracked()).expect("should not be disposed")
	}

	/// Gets all IDs reactively.
	#[track_caller]
	pub fn ids(&self) -> Vec<StateId> {
		self.0.try_get_value().map(|states| states.ids()).expect("should not be disposed")
	}

	/// Gets all IDs non-reactively.
	#[track_caller]
	pub fn ids_untracked(&self) -> Vec<StateId> {
		self.0.try_get_value().map(|states| states.ids_untracked()).expect("should not be disposed")
	}

	/// Adds a new global state to the context.
	#[track_caller]
	pub fn add_state(&self) {
		self.0.try_get_value().map(|states| states.add_state()).expect("should not be disposed")
	}

	/// Sets a new active global state in the context.
	#[track_caller]
	pub fn set_active_state(&self, id: StateId) {
		self.0.try_get_value().map(|states| states.set_active_state(id)).expect("should not be disposed")
	}

	/// Removes a global state from the context.
	#[track_caller]
	pub fn remove_state(&self, id: StateId) {
		self.0.try_get_value().map(|states| states.remove_state(id)).expect("should not be disposed")
	}

	/// Gets the default ID.
	#[track_caller]
	pub fn default_id(&self) -> StateId {
		self.0.try_get_value().map(|states| states.default_id()).expect("should not be disposed")
	}

	/// Gets the selected ID arc signal.
	#[track_caller]
	pub fn selected_id(&self) -> ArcRwSignal<StateId> {
		self.0.try_get_value().map(|states| states.selected_id()).expect("should not be disposed")
	}

	/// Gets the selected state arc signal.
	#[track_caller]
	pub fn selected_state(&self) -> ArcRwSignal<ArcStore<T>> {
		self.0.try_get_value().map(|states| states.selected_state()).expect("should not be disposed")
	}
}
