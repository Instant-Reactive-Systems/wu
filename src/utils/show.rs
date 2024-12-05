use leptos::*;

/// A component that will show its children when the `when` condition is `true`,
/// and show the fallback when it is `false`, without rerendering every time
/// the condition changes.
///
/// The fallback prop is optional and defaults to rendering nothing.
///
/// ```rust
/// # use leptos_reactive::*;
/// # use leptos_macro::*;
/// # use leptos_dom::*; use leptos::*;
/// # let runtime = create_runtime();
/// let (data, set_data) = create_signal(0);
///
/// view! {
///   <ShowWithData data
/// 	when=move |data| value.get() < 5
/// 	fallback=|| view! { "Big number!" }
/// 	let:data
///   >
/// 	"Small number: " {data} "!"
///   </ShowWithData>
/// }
/// # ;
/// # runtime.dispose();
/// ```
#[component]
pub fn ShowWithData<VF, T, IV, W>(
	/// The data to process.
	#[prop(into)]
	data: Signal<T>,
	/// A closure that returns a bool that determines whether this thing runs
	when: W,
	/// A closure that returns what gets rendered if the when statement is false. By default this is the empty view.
	#[prop(optional, into)]
	fallback: ViewFn,
	/// The children will be shown whenever the condition in the `when` closure returns `true`.
	children: VF,
) -> impl IntoView
where
	VF: Fn(T) -> IV + 'static,
	T: Clone + 'static,
	IV: IntoView + 'static,
	W: Fn(&T) -> bool + 'static,
{
	view! {
		<Show when=move || with!(|data| when(&data)) fallback>
			{children(data.get())}
		</Show>
	}
}

/// A component that will show its children when the `when` condition is `true`,
/// and show the fallback when it is `false`, without rerendering every time
/// the condition changes.
///
/// The fallback prop is optional and defaults to rendering nothing.
///
/// ```rust
/// # use leptos_reactive::*;
/// # use leptos_macro::*;
/// # use leptos_dom::*; use leptos::*;
/// # let runtime = create_runtime();
/// let (data, set_data) = create_signal(None);
///
/// view! {
///   <ShowOption data
/// 	fallback=|| view! { "No data" }
/// 	let:data
///   >
/// 	"Data: " {data} "!"
///   </ShowOption>
/// }
/// # ;
/// # runtime.dispose();
/// ```
#[component]
pub fn ShowOption<VF, T, IV>(
	/// The data to process.
	#[prop(into)]
	data: Signal<Option<T>>,
	/// A closure that returns what gets rendered if the when statement is false. By default this is the empty view.
	#[prop(optional, into)]
	fallback: ViewFn,
	/// The children will be shown whenever the condition in the `when` closure returns `true`.
	children: VF,
) -> impl IntoView
where
	VF: Fn(T) -> IV + 'static,
	T: Clone + 'static,
	IV: IntoView + 'static,
{
	view! {
		<Show when=move || with!(|data| data.is_some()) fallback>
			{children(data.get().unwrap())}
		</Show>
	}
}
