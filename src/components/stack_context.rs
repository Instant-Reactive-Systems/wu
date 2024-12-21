use leptos::prelude::*;

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StackMarker<M>(std::marker::PhantomData<M>);

crate::generate_marker_type!(
	#[doc(hidden)]
	PushStackCtxMarker
);

/// Pushes a new stack context.
pub type PushStackCtx<M, T> = crate::utils::Marked<StackMarker<(M, PushStackCtxMarker)>, Callback<T>>;
/// Pops current stack context.
pub type PopStackCtx<M> = crate::utils::Marked<StackMarker<(M, PushStackCtxMarker)>, Callback<()>>;

/// Used to provide a scoped generic stack to all its children.
#[component]
pub fn StackContext<M, T>(
	#[prop(optional)] _phant: std::marker::PhantomData<(M, T)>,
	/// Children of the component.
	children: Children,
) -> impl IntoView
where
	M: Send + Sync + 'static,
	T: Send + Sync + Clone + 'static,
{
	let stack_cxs = RwSignal::<Vec<T>>::new(Default::default());
	let active_cx = Signal::derive(move || stack_cxs.with(move |cxs| { !cxs.is_empty() }.then(move || cxs[cxs.len() - 1].clone())));
	provide_context(PushStackCtx::<M, T>::new(Callback::new(move |cx| {
		stack_cxs.update(move |cxs| cxs.push(cx));
	})));
	provide_context(PopStackCtx::<M>::new(Callback::new(move |_| {
		stack_cxs.update(move |cxs| _ = cxs.pop());
	})));
	provide_context(ActiveStackCtx::<M, T> {
		cx: active_cx,
		_phant: Default::default(),
	});

	view! {
		{children()}
	}
}

/// Currently active stack context.
pub struct ActiveStackCtx<M: Send + Sync + 'static, T: Send + Sync + Clone + 'static> {
	pub cx: Signal<Option<T>>,
	_phant: std::marker::PhantomData<M>,
}

impl<M: Send + Sync + 'static, T: Send + Sync + Clone + 'static> Clone for ActiveStackCtx<M, T> {
	fn clone(&self) -> Self {
		Self {
			cx: self.cx.clone(),
			_phant: Default::default(),
		}
	}
}

impl<M: Send + Sync + 'static, T: Send + Sync + Clone + 'static> Copy for ActiveStackCtx<M, T> {}
