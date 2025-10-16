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
	/// Fallback view if the dataset is empty.
	#[prop(optional, into)]
	on_empty: ViewFn,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	class: Text,
	/// Corresponds to the 'class' attribute of elements.
	#[prop(optional, into)]
	tbody_class: Text,
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

	move || match data_resource.get() {
		None => Either::Left(fallback.run()),
		Some(res) => Either::Right(view! {
			<div class=move || format!("wtable {class}") style=move || style.get().into_owned()>
				<table>
					<thead>
						{let header = header.clone(); move || header.run()}
					</thead>
					{match res {
						Err(err) => Either::Left(view! {
							<tbody class=move || tbody_class.get()>
								<div class="cover flex hvcenter">
									<div class="vertical gap-2">
										<div class="flex hcenter">
											<span class="icon i-o-exclamation-triangle icon-error-500 size-12"/>
										</div>
										<span class="font-semibold text-content-emph">
											{format!("{err:?}")}
										</span>
									</div>
								</div>
							</tbody>
						}),
						Ok((total_count, records)) => Either::Right(match total_count == 0 {
							true => Either::Left(view! {
								<tbody class=move || tbody_class.get()>
									{let on_empty = on_empty.clone(); move || on_empty.run()}
								</tbody>
							}),
							false => Either::Right(view! {
								<tbody class=move || tbody_class.get()>
									{
										let row = row.clone();
										records
											.into_iter()
											.map(move |record| row.clone().run(record))
											.collect::<Vec<_>>()
									}
								</tbody>
								<tfoot>
									<td class="grow hvcenter">
										<div class="horizontal gap-4">
											// Previous
											<div class="flex hvcenter">
												{move || match offset.get() != 0 {
													false => Either::Left(view! { <div class="flex-none size-8"/> }),
													true => Either::Right(view ! {
														<button
															on:click=move |_| offset.update(move |offset| *offset = offset.saturating_sub(1))
															class="btn-icon autohighlight size-8"
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
													{
														if limit != 0 {
															(total_count + limit - 1) / limit
														} else {
															1
														}

													}
												</span>
											</div>
											// Next
											<div class="flex hvcenter">
												{move || match limit != 0 && offset.get() < (total_count + limit - 1) / limit - 1 {
													false => Either::Left(view! { <div class="flex-none size-8"/> }),
													true => Either::Right(view! {
														<button
															on:click=move |_| offset.update(move |offset| *offset = offset.saturating_add(1))
															class="btn-icon autohighlight size-8"
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
						}),
					}}
				</table>
			</div>
		}),
	}
}
