use leptos::{either::*, prelude::*};

use crate::utils::*;

/// A table component displaying paginated records and providing a way to view into
/// windows of records via a footer control part.
#[component]
pub fn Table<T, E, Fut, F>(
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
	#[prop(into)]
	header: ViewFn,
	/// Table row view.
	#[prop(into)]
	row: LocatableViewFnWithArgs<T>,
	/// Fallback (loading) view.
	#[prop(optional, into)]
	fallback: ViewFn,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	class: Text,
	/// Corresponds to the 'style' attribute of elements.
	#[prop(optional, into)]
	style: Text,
) -> impl IntoView
where
	T: Clone + Send + Sync + 'static,
	E: std::fmt::Debug + Clone + Send + Sync + 'static,
	Fut: std::future::Future<Output = Result<(u64, Vec<T>), E>> + 'static,
	F: Fn(u64, u64) -> Fut + 'static,
{
	// vars
	let offset: RwSignal<u64> = RwSignal::new(0);
	let data_resource = LocalResource::new(move || data_source(offset.get(), limit));

	view! {
		<div class=move || format!("wtable {class}") style=move || style.get().into_owned()>
			<table>
				<thead>
					{move || header.run()}
				</thead>
				{move || match data_resource.get() {
					Some(res) => Either::Left(match res {
						Ok((total_count, records)) => Either::Left(view! {
							<tbody>
								{
									let row = row.clone();
									records
										.into_iter()
										.map(move |record| row.clone().run(record))
										.collect::<Vec<_>>()
								}
							</tbody>
							<tfoot>
								<td class="grow center">
									<div class="horizontal gap-4">
										// Previous
										<div class="flex center">
											{move || match offset.get() == 0 {
												true => Either::Left(view! { <div class="flex-none size-6 pointer-coarse:size-8"/> }),
												false => Either::Right(view ! {
													<button
														on:click=move |_| offset.update(move |offset| *offset = offset.saturating_sub(1))
														class="btn-icon autohighlight size-6 pointer-coarse:size-8"
													>
														<span class="icon i-o-arrow-left size-4" />
													</button>
												}),
											}}
										</div>
										// Current pages
										<div class="flex vcenter">
											<span class="text-lg">
												{move || offset.get() + 1}
												" / "
												{(limit != 0).then_some((total_count / limit) + 1).unwrap_or(1)}
											</span>
										</div>
										// Next
										<div class="flex center">
											{move || match (limit != 0).then_some((offset.get() + 1) * limit).unwrap_or(total_count) >= total_count {
												true => Either::Left(view! { <div class="flex-none size-6 pointer-coarse:size-8"/> }),
												false => Either::Right(view! {
													<button
														on:click=move |_| offset.update(move |offset| *offset = offset.saturating_add(1))
														class="btn-icon autohighlight size-6 pointer-coarse:size-8"
													>
														<span class="icon i-o-arrow-right size-4" />
													</button>
												}),
											}}
										</div>
									</div>
								</td>
							</tfoot>
						}),
						Err(err) => Either::Right(view! {
							<tbody>
								<div class="cover flex center">
									<div class="vertical gap-2">
										<span class="icon i-o-exclamation-triangle icon-error-500"/>
										<span class="font-semibold text-content-emph">
											{format!("{err:?}")}
										</span>
									</div>
								</div>
							</tbody>
						}),
					}),
					None => Either::Right(view! {
						<tbody>
							{fallback.run()}
						</tbody>
					}),
				}}
			</table>
		</div>
	}
}
