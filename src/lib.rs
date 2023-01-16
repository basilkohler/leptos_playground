use crate::db::{MockItem, DB};
use crate::pagination::{Pagination, PaginationProps, PaginationStateContext};
use leptos::*;

pub mod db;
pub mod pagination;
pub mod pagination_state;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <h1>"Paginated Items"</h1>
            // Customize rendering of Pagination component
            <Pagination link_element=Box::new(|cx, cur, page|
                if let Some(page) = page {
                    view!(cx, <span style={if cur {"color: red" } else {""}}>"<"{move || page}">"</span>).into_any()
                } else {
                    view!(cx, <span>"..."</span>).into_any()
                }
            )>
                <Items/>
            </Pagination>
        </div>
    }
}

#[component]
pub fn Items(cx: Scope) -> impl IntoView {
    let PaginationStateContext {
        pagination_state,
        set_pagination_state,
    } = use_context(cx).unwrap_throw();

    let paginated_items = create_local_resource(
        cx,
        move || pagination_state(),
        move |ps| async move {
            // Update resource on pagination state change
            let items = DB::new(42)
                .get_paginated_items(ps.calc_skip(), ps.page_size())
                .clone();
            // Update pagination by writing the total number of elements
            // This has to be done here because in effects its not allowed to write to signals (
            set_pagination_state
                .update(|pagination_state| pagination_state.set_element_count(items.total));
            items.result
        },
    );

    view! { cx,
        <div>{move || {
            paginated_items.with(|paginated_items| {
                paginated_items.iter()
                    .map(| item | view!{ cx, <MockItem item=item.clone()/> })
                .collect::<Vec<_>>()
            })
        }}</div>
    }
}

#[component]
pub fn MockItem(cx: Scope, item: MockItem) -> impl IntoView {
    view! { cx,
        <div>
            <h3>{format!("{} [{}]", item.title, item.id)}</h3>
            <p>{item.description}</p>
        </div>
    }
}
