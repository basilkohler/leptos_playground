use leptos::*;
use leptos_router::*;

use crate::db::{MockItem, DB};
use crate::pagination::pagination_components::{
    Pagination, PaginationProps, PaginationStateContext,
};

// #[server(GetItems, "/api")]
// pub async fn get_todos(cx: Scope) -> Result<Vec<MockItem()>, ServerFnError> {
//
// }

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
                <Transition fallback = move || view! {cx, <div>"Loading..."</div>} >
                    <Pagination
                        pagination_link=Box::new(|page, page_size| format!("/items?page={}&page_size={}", page, page_size))
                        page_query_param="page".to_string()
                        page_size_query_param="page_size".to_string()>
                        <Items/>
                    </Pagination>
                </Transition>
            <Outlet/>
        </div>
    }
}

#[component]
pub fn Items(cx: Scope) -> impl IntoView {
    let PaginationStateContext {
        pagination_state,
        set_pagination_state,
    } = use_context(cx).unwrap();

    log::info!("init Items");

    let paginated_items = create_resource(
        cx,
        move || pagination_state(),
        move |ps| async move {
            let db = DB::new(42);
            let (items, total_count) = db.get_paginated_items(ps.page(), ps.page_size());
            set_pagination_state.update(|ps| ps.set_element_count(total_count));
            return items;
        },
    );
    view! { cx, <div>
        <Transition fallback = move || view! {cx, <div>"Loading..."</div>} >
        {move || {
            paginated_items.with(|items| {
                let items = items.clone();
                view! { cx, <div>
                {move ||
                items.iter()
                    .map(| item | view!{ cx, <MockItem item=item.clone()/> })
                .collect::<Vec<_>>()}
            </div>}})
        }}
                </Transition>
    </div>}
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
pub fn MockItem(cx: Scope, item: MockItem) -> impl IntoView {
    view! { cx,
        <div>
            <h3>{format!("{} [{}]", item.title, item.id)}</h3>
            <p>{item.description}</p>
        </div>
    }
}
