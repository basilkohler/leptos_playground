use std::rc::Rc;

use leptos::*;

use crate::pagination_state::PaginationState;

#[derive(Copy, Clone)]
pub struct PaginationStateContext {
    pub pagination_state: ReadSignal<PaginationState>,
    pub set_pagination_state: WriteSignal<PaginationState>,
}

#[component]
pub fn Pagination(
    cx: Scope,
    page: usize,
    page_size: usize,
    link_element: Box<dyn Fn(Scope, bool, Option<usize>) -> HtmlElement<AnyElement>>,
    children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
    let (state, set_state) = create_signal(cx, PaginationState::new(page, page_size));

    let link_element = Rc::new(link_element);

    let update_page_size = move |ref event: web_sys::Event| {
        let size = event_target_value(event)
            .parse()
            .unwrap_or_else(|_| 1_usize);
        set_state.update(|state| state.set_page_size(size));
    };

    provide_context(
        cx,
        PaginationStateContext {
            pagination_state: state,
            set_pagination_state: set_state,
        },
    );

    view! {cx,
        <div>
            <label>"page size: "{move || state().page_size()}</label>
            <select on:change=update_page_size
                    prop:value={move || state().page_size()}>
                <For
                    each=move || vec![1, 2, 10]
                    key=|i| i.clone()
                    view=move |i: usize| view! { cx, <option prop:value={i.clone()} selected={state().page_size() == i.clone()}>{i.clone()}</option> }
                />
            </select>
            <div>
                <button on:click=move |_| set_state.update(PaginationState::go_prev)
                        disabled=move || state().has_prev()>
                    {move || state().prev() }
                </button>

                {move ||
                    state()
                        .generate_pagination().iter()
                        .map(|maybe_page| {
                            let link_element = link_element.clone();
                            match maybe_page {
                                Some(page) => link_element(cx, state().is_cur(*page), Some(*page)),
                                None => link_element(cx, false, None),
                            }
                        })
                        .collect::<Vec<_>>()
                }

                <button on:click=move |_| set_state.update(PaginationState::go_next)
                        disabled=move || !state().has_next()>
                    {move || state().next() }
                </button>
            </div>
            {children(cx)}
        </div>
    }
}
