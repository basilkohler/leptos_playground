use crate::pagination::{Pagination, PaginationProps, PaginationStateContext};
use leptos::*;

pub mod pagination;
pub mod pagination_state;

#[derive(Debug)]
pub struct DB {
    pub items: Vec<MockItem>,
}

impl DB {
    pub fn new(size: usize) -> Self {
        let items = (0..size).map(|i| MockItem {
            id: format!("id{i}"),
            title: format!("title{i}"),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Mauris a diam maecenas sed enim ut sem viverra aliquet.".to_string(),
        }).collect();
        DB { items }
    }

    pub fn get_paginated_items(&self, n_skip: usize, n_take: usize) -> PaginatedResult<MockItem> {
        PaginatedResult {
            result: self
                .items
                .iter()
                .cloned()
                .skip(n_skip)
                .take(n_take)
                .collect(),
            total: self.items.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MockItem {
    id: String,
    title: String,
    description: String,
}

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    result: Vec<T>,
    total: usize,
}

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
