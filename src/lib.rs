use leptos::*;
use leptos_router::*;

use crate::db::{MockItem, DB};
use crate::pagination::{Pagination, PaginationProps, PaginationStateContext};
use crate::pagination_state::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};

pub mod db;
pub mod pagination;
pub mod pagination_state;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <Router>
                <nav>
                    <A exact=true href="/">"Home"</A>
                    <A href="items">"Items"</A>
                </nav>
                <main>
                    <Routes>
                        <Route path="/" view=move |cx| view! {cx, <h1>"Home"</h1>}/>
                        <Route path="items" view=move |cx| view! {cx, <div><h1>"Items"</h1><ItemsView/></div>}/>
                        <Route path="items/:id" view=move |cx| view! {cx, <h1>"Item"</h1><ItemView/>}/>
                    </Routes>
                </main>
            </Router>
        </div>
    }
}

#[component]
pub fn ItemsView(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <h1>"Paginated Items"</h1>
            // Customize rendering of Pagination component
            <Pagination
                fetch_items=Box::new(|page, page_size| DB::new(42).get_paginated_items(page, page_size))
                pagination_link=Box::new(|page, page_size| format!("/items?page={}&page_size={}", page, page_size))
                page_query_param="page".to_string()
                page_size_query_param="page_size".to_string()
                items_view=Box::new(|cx, items| view!(cx, <Items items=items/>)) >
            </Pagination>
            <Outlet/>
        </div>
    }
}

#[component]
pub fn ItemView(cx: Scope) -> impl IntoView {
    fn get_item(cx: Scope, id: &String) -> Option<MockItem> {
        let id: usize = id.parse().ok()?;
        let db = DB::new(42);
        db.items.get(id).cloned()
    }
    let params = use_params_map(cx);
    let id = &params()
        .get("id")
        .map(|s| s.to_string())
        .unwrap_or("".to_string());

    view! {cx,
        <A href="/items">"Back to Items"</A>
        <div>{
            if let Some(item) = get_item(cx, id) {
                (view! {cx, <MockItem item=item.clone()/> }).into_view(cx)
            } else {
                (view! {cx, <div>{format!("Item '{}' not found", id)}</div>}).into_view(cx)
            }
        }</div>
    }
}

#[component]
pub fn Items(cx: Scope, items: Vec<MockItem>) -> impl IntoView {
    // let PaginationStateContext {
    //     pagination_state,
    //     set_pagination_state,
    // } = use_context(cx).unwrap_throw();
    //
    // let paginated_items = create_local_resource(
    //     cx,
    //     move || pagination_state(),
    //     move |ps| async move {
    //         // Update resource on pagination state change
    //         let items = DB::new(42)
    //             .get_paginated_items(ps.calc_skip(), ps.page_size())
    //             .clone();
    //         // Update pagination by writing the total number of elements
    //         // This has to be done here because in effects its not allowed to write to signals (
    //         set_pagination_state
    //             .update(|pagination_state| pagination_state.set_element_count(items.total));
    //         items.result
    //     },
    // );

    view! { cx,
        <div>{move || {
            // paginated_items.with(|paginated_items| {
                items.iter()
                    .map(| item | view!{ cx, <MockItem item=item.clone()/> })
                .collect::<Vec<_>>()
            // })
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
