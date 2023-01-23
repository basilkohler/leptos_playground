use std::rc::Rc;

use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::pagination::pagination_state::{PaginationItem::*, PaginationState};
use crate::pagination::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};

#[derive(Copy, Clone)]
pub struct PaginationStateContext {
    pub pagination_state: ReadSignal<PaginationState>,
    pub set_pagination_state: WriteSignal<PaginationState>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(bound(deserialize = "'de: 'a"))]
// #[derive(Debug, Clone)]
// pub struct PaginatedResult<Tstatic>> {
//     pub result: Vec<T>,
//     pub total: usize,
// }

#[component]
pub fn Pagination(
    cx: Scope,
    pagination_link: Box<dyn Fn(usize, usize) -> String>,
    page_query_param: String,
    page_size_query_param: String,
    #[prop(optional)] page_sizes: Option<Vec<usize>>,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    let (state, set_state) = create_signal(cx, PaginationState::default());
    let page_sizes = page_sizes.unwrap_or(vec![1, 2, 10]);

    let query = use_query_map(cx);
    let navigate = use_navigate(cx);

    let query_state = move || {
        query.with(|query_map| {
            let page = query_map
                .get(&page_query_param)
                .cloned()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_PAGE);
            let page_size = query_map
                .get(&page_size_query_param)
                .cloned()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_PAGE_SIZE);
            (page, page_size)
        })
    };

    create_effect(cx, move |_| {
        let (page, page_size) = query_state();
        set_state.update(|ps| ps.set_page_and_size(page, page_size));
        state()
    });

    let pagination_link = Rc::new(pagination_link);
    provide_context(
        cx,
        PaginationStateContext {
            pagination_state: state,
            set_pagination_state: set_state,
        },
    );

    let pagination_link_update = pagination_link.clone();
    let update_page_size = move |ref event: web_sys::Event, page: usize| {
        let page_size = event_target_value(event)
            .parse()
            .unwrap_or_else(|_| 1_usize);
        let path = &pagination_link_update(page, page_size);
        navigate(path, NavigateOptions::default()).unwrap();
    };

    view! {cx, <div>
                <label>"page size: "{move || state().page_size()}</label>
                <select on:change=move |e| update_page_size(e, state().page())
                        prop:value={move || state().page_size()}>
                    <For
                        each=move || page_sizes.clone()
                        key=|i| i.clone()
                        view=move |i: usize| view! { cx, <option prop:value={i.clone()} selected={state().page_size() == i.clone()}>{i.clone()}</option> }
                    />
                </select>
                <div>
                    {move || {
                        let pl = pagination_link.clone();
                        state()
                            .generate_pagination().into_iter()
                            .map(|pagination_item| {
                                let v: Vec<View> = match pagination_item {
                                        First(Some(page)) => vec![view!(cx, <A href={pl(page, state().page_size())}>"<<"</A>).into_view(cx)],
                                        First(None) => vec![view!(cx, <span>"<<"</span>).into_view(cx)],
                                        Prev(Some(page)) => vec![view!(cx, <A href={pl(page, state().page_size())}>"<"</A>).into_view(cx)],
                                        Prev(None) => vec![view!(cx, <span>"<"</span>).into_view(cx)],
                                        DotsLeft | DotsRight => vec![view!(cx, <span>"..."</span>).into_view(cx)],
                                        Pages(pages) => pages.into_iter()
                                                                .map(|(page, is_cur)|
                                                                if is_cur {
                                                                     view!(cx, <span>{page}</span>).into_view(cx)
                                                                } else {
                                                                     view!(cx, <A href={pl(page, state().page_size())}>{page}</A>).into_view(cx)
                                                                }).collect::<Vec<_>>(),
                                        Next(Some(page)) => vec![view!(cx, <A href={pl(page, state().page_size())}>">"</A>).into_view(cx)],
                                        Next(None) => vec![view!(cx, <span>">"</span>).into_view(cx)],
                                        Last(Some(page)) => vec![view!(cx, <A href={pl(page, state().page_size())}>">>"</A>).into_view(cx)],
                                        Last(None) => vec![view!(cx, <span>">>"</span>).into_view(cx)],
                                };
                             view! {cx, <span>{
                                v.into_iter().map(|v| view!(cx, <span>{v}" | "</span>)).collect::<Vec<_>>()
                             }</span>}
                            }).collect::<Vec<_>>()}}
                </div>

                {children(cx)}

        </div>
    }
}
