use std::{borrow::Cow, rc::Rc};

use leptos::{ev::*, *};
use leptos_use::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

/// Creates a focus trap signal.
pub fn create_focus_trap(selector_id: impl Into<Cow<'static, str>>) -> (SignalSetter<()>, SignalSetter<()>) {
	let selector_id: Cow<'static, str> = selector_id.into();
	let (_, set_state) = create_signal(None::<State>);
	let (unsub, set_unsub) = create_signal(None::<Rc<dyn Fn()>>);

	let add_trap = SignalSetter::map(move |_: ()| {
		// find the origin if there exists one
		// (it might not exist if the trap focus
		// was caused programmatically)
		let origin = document().active_element();

		// find the currently active focus trap container target
		let target = match document().get_element_by_id(&selector_id) {
			Some(target) => target,
			None => {
				log::warn!("no element tagged with '{selector_id}' found, will not apply trap focus");
				return;
			},
		};

		// find all focusable children inside the target
		let children = get_focusable_children(&target);

		// focus the first one in the target
		if let Some((start, _)) = &children {
			let _ = start
				.dyn_ref::<web_sys::HtmlElement>()
				.expect("node should be an html element according to the query selector")
				.focus();
		}

		// register keyboard handling for the trap focus
		set_unsub.set(Some(Rc::new(use_event_listener(document(), keydown, move |evt| focus_trap_event_handler(evt, &children)))));

		// update the state
		set_state.update(move |state| match state {
			Some(state) => {
				state.history.push(state.active.clone());
				state.active = Event { target, origin };
			},
			None => {
				*state = Some(State {
					history: Vec::default(),
					active: Event { origin, target },
				});
			},
		});
	});

	let pop_trap = SignalSetter::map(move |_: ()| {
		set_state.update(move |state_opt| {
			if let Some(state) = state_opt {
				// unsubscribe previous element
				if let Some(unsub) = unsub.get() {
					unsub();
				}

				// restore previous target and origin
				let event = match state.history.pop() {
					Some(event) => event,
					None => {
						*state_opt = None;
						return;
					},
				};

				// refetch the target's focusable children
				let children = get_focusable_children(&event.target);

				// focus the origin
				if let Some(origin) = &state.active.origin {
					let _ = origin
						.dyn_ref::<web_sys::HtmlElement>()
						.expect("node should be an html element according to the query selector")
						.focus();
				}

				// subscribe to the new target again
				set_unsub.set(Some(Rc::new(use_event_listener(event.target.clone(), keydown, move |evt| focus_trap_event_handler(evt, &children)))));

				// update the target
				state.active.target = event.target;
			}
		});
	});

	(add_trap, pop_trap)
}

fn get_focusable_children(target: &Element) -> Option<(Node, Node)> {
	const QUERY_SELECTOR: &str = r#"a[href]:not([disabled]), button:not([disabled]), textarea:not([disabled]), input[type="text"]:not([disabled]), input[type="radio"]:not([disabled]), input[type="checkbox"]:not([disabled]), select:not([disabled])"#;

	// query select all interactable (tabbable) elements
	let focusable_children = target.query_selector_all(QUERY_SELECTOR).expect("a correct selector list");

	// extract only the first and the last so we can implement cycling
	focusable_children.item(0).map(|start| {
		let end = focusable_children.item(focusable_children.length() - 1).unwrap();
		(start, end)
	})
}

fn focus_trap_event_handler(evt: KeyboardEvent, children: &Option<(Node, Node)>) {
	// focus switching is done by tab and shift-tab
	match (evt.shift_key(), &evt.code() == "Tab") {
		(false, true) => {
			if let Some((start, end)) = children {
				if end == &evt.target().expect("keyboard event should have a target").dyn_into::<Node>().unwrap() {
					let _ = start
						.dyn_ref::<web_sys::HtmlElement>()
						.expect("node should be an html element according to the query selector")
						.focus();
					evt.prevent_default();
				}
			}
		},
		(true, true) => {
			if let Some((start, end)) = children {
				if start == &evt.target().expect("keyboard event should have a target").dyn_into::<Node>().unwrap() {
					let _ = end
						.dyn_ref::<web_sys::HtmlElement>()
						.expect("node should be an html element according to the query selector")
						.focus();
					evt.prevent_default();
				}
			}
		},
		_ => {},
	}
}

/// A trap focus event.
#[derive(Clone)]
struct Event {
	/// The element that caused the trap focus.
	///
	/// Used for going back in time.
	origin: Option<Element>,

	/// The target element on which to perform the trap focus.
	target: Element,
}

/// State of the trap focus context.
#[derive(Clone)]
struct State {
	/// History of all focus trap events.
	history: Vec<Event>,

	/// Current active event.
	active: Event,
}
