use leptos::*;

crate::generate_generic_marker_signal_setter!(
	/// Pushes a new stack context.
	PushStackContext, T, T
);

crate::generate_marker_signal_setter!(
	/// Pops current stack context.
	PopStackContext, ()
);

/// Used to provide a scoped generic stack to all its children.
#[component]
pub fn StackContext<M: 'static, T: Clone + 'static>(
	#[prop(optional)] _phant: std::marker::PhantomData<(M, T)>,
	/// Children of the component.
	children: Children,
) -> impl IntoView {
	let (stack_cxs, set_stack_cxs) = create_signal::<Vec<T>>(Default::default());
	let active_cx = Signal::derive(move || stack_cxs.with(move |cxs| { !cxs.is_empty() }.then(move || cxs[cxs.len() - 1].clone())));
	provide_context(PushStackContext::<M, T>::new(move |cx| {
		set_stack_cxs.update(move |cxs| cxs.push(cx));
	}));
	provide_context(PopStackContext::<M>::new(move |_| {
		set_stack_cxs.update(move |cxs| _ = cxs.pop());
	}));
	provide_context(ActiveStackContext::<M, T> {
		cx: active_cx,
		_phant: Default::default(),
	});

	view! {
		{children()}
	}
}

/// Currently active stack context.
pub struct ActiveStackContext<M: 'static, T: 'static> {
	pub cx: Signal<Option<T>>,
	_phant: std::marker::PhantomData<M>,
}

impl<M: 'static, T: 'static> Clone for ActiveStackContext<M, T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<M: 'static, T: 'static> Copy for ActiveStackContext<M, T> {}
