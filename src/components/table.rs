use leptos::{either::*, prelude::*};

use crate::utils::{Text, ViewFnWithArgs};

/// A table component displaying paginated records and providing a way to view into
/// windows of records via a footer control part.
#[component]
pub fn Table<F, T>(
	/// Source for getting the data.
	///
	/// # Note
	/// The parameters of the function returning the future are: (u32, u32)
	/// 0: Offset
	/// 1: Limit
	data_source: F,
	/// The limit on the number of records to fetch.
	limit: u64,
	/// Table header view.
	header: ViewFn,
	/// Table row view.
	row: ViewFnWithArgs<T>,
	/// Fallback (loading) view.
	fallback: ViewFn,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	class: Text,
	/// Corresponds to the 'style' attribute of elements.
	#[prop(optional, into)]
	style: Text,
) -> impl IntoView
where
	F: Fn(u32, u32) -> std::future::Future<Output = (u64, Vec<T>)>,
	T: Send + Sync + 'static,
{
	// vars
	let offset: RwSignal<u64> = RwSignal::new(0);
	let data_resource = LocalResource::new(move || data_source(offset.get(), limit));

	view! {
		<div class=move || format!("wtable {class}") style=style>
			<table>
				<thead>
					{move || header.run()}
				</thead>
				{move || match data_resource.get() {
					Some((total_count, records)) => Either::Left(view! {
						<tbody>
							{
								records
									.into_iter()
									.map(move |record| row.run(record))
									.collect::<Vec<_>>()
							}
						</tbody>
						<tfoot>
							<td class="grow center">
								<div class="horizontal gap-2">
									// Previous
									{move || match offset.get() == 0 {
										true => Either::Left(view! { <div class="flex-none size-10"/> }),
										false => Either::Right(view ! {
											<button
												on:click=move |_| offset.write().saturating_sub(1)
												class="btn-icon btn-primary size-10"
											>
												<span class="icon i-o-arrow-left" />
											</button>
										}),
									}}
									// Current pages
									<div class="h-10 flex vcenter">
										<span class="text-lg">
											{move || offset.get()}
											" / "
											{total_count}
										</span>
									</div>
									// Next
									{move || match offset.get() + limit >= total_count {
										true => Either::Left(view! { <div class="flex-none size-10"/> }),
										false => Either::Right(view ! {
											<button
												on:click=move |_| offset.write().saturating_add(limit)
												class="btn-icon btn-primary size-10"
											>
												<span class="icon i-o-arrow-right" />
											</button>
										}),
									}}
								</div>
							</td>
						</tfoot>
					}),,
					None => Either::Right(fallback.run()),
				}}
			</table>
		</div>
	}
}
