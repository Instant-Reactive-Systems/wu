use leptos::{prelude::*, either::Either};

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
pub fn ShowOption<T, VF, IV, TF>(
	/// The data to process.
	data: TF,
	/// A closure that returns what gets rendered if the data is `None`. By default this is the empty view.
	#[prop(optional, into)]
	fallback: ViewFn,
	/// The children will be shown whenever the data is `Some`.
	children: VF,
) -> impl IntoView
where
	VF: Send + Sync + Fn(T) -> IV + 'static,
	T: Send + Sync + Clone + 'static,
	IV: IntoView + 'static,
	TF: Send + Sync + Fn() -> Option<T> + 'static,
{
	let data = Signal::derive(data);
	move || match data.get() {
		Some(data) => Either::Left(children(data)),
		None => Either::Right(fallback.clone().run()),
	}
}
