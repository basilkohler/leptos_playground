use leptos::*;
use leptos_router::*;

use crate::db::{MockItem, DB};
use crate::pagination::pagination_components::{Pagination, PaginationProps};

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
                        <Route path="/" view=move |cx| view! {cx, <h1>"Home "</h1>}/>
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
    fn get_item(_cx: Scope, id: &String) -> Option<MockItem> {
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
    view! { cx,
        <div>{move || {
            items.iter()
                .map(| item | view!{ cx, <MockItem item=item.clone()/> })
            .collect::<Vec<_>>()
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
