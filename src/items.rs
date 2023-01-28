use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::pagination::pagination_components::{
    Pagination, PaginationProps, PaginationStateContext,
};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection};

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            let conn = SqliteConnection::connect("sqlite:Items.sqlite")
                    .await
                    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
            Ok(conn)
        }

        pub fn register_server_functions() {
            _ = GetItems::register();
            _ = GetItem::register();
            _ = AddItem::register();
            _ = RemoveItem::register();
        }
    }
}

#[server(GetItems, "/api")]
pub async fn get_items(
    cx: Scope,
    page: u32,
    page_size: u32,
) -> Result<(Vec<MockItem>, u32), ServerFnError> {
    let req_parts = use_context::<leptos_axum::RequestParts>(cx);

    if let Some(req_parts) = req_parts {
        log::info!("Uri = {:?}", req_parts.uri);
    }

    let mut conn = db().await?;

    let offset = page.saturating_sub(1) * page_size.clone();
    let limit = page_size;
    let items: Vec<MockItem> = sqlx::query_as!(
        MockItem,
        "SELECT * FROM items LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(&mut conn)
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    let mut conn = db().await?;
    let total_count = sqlx::query!("SELECT COUNT(*) as count FROM items")
        .fetch_one(&mut conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok((items, total_count.count as u32))
}

#[server(GetItem, "/api")]
pub async fn get_item(id: i32) -> Result<MockItem, ServerFnError> {
    let mut conn = db().await?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    let item = sqlx::query_as!(MockItem, "SELECT * FROM items WHERE id = $1", id)
        .fetch_one(&mut conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(item)
}

#[server(AddItem, "/api")]
pub async fn add_item(title: String, description: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    std::thread::sleep(std::time::Duration::from_secs(1));

    sqlx::query!(
        "INSERT INTO items (title, description) VALUES ($1, $2)",
        title,
        description
    )
    .execute(&mut conn)
    .await
    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(())
}

#[server(RemoveItem, "/api")]
pub async fn remove_item(id: i32) -> Result<(), ServerFnError> {
    let mut conn = db().await?;
    std::thread::sleep(std::time::Duration::from_secs(1));

    sqlx::query!("DELETE FROM items WHERE id = $1", id)
        .execute(&mut conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(())
}

#[component]
pub fn ItemsView(cx: Scope) -> impl IntoView {
    view! {cx,
        <div>
            <h1>"Paginated Items"</h1>
                <Pagination
                    pagination_link=Box::new(|page, page_size| format!("/items?page={}&page_size={}", page, page_size))
                    page_query_param="page".to_string()
                    page_size_query_param="page_size".to_string()>
                    <Items/>
                </Pagination>
            <Outlet/>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MockItem {
    pub id: i64,
    pub title: String,
    pub description: String,
}

#[derive(Clone)]
struct ItemsActions {
    add_item: MultiAction<AddItem, Result<(), ServerFnError>>,
    remove_item: Action<RemoveItem, Result<(), ServerFnError>>,
}

impl ItemsActions {
    fn new(cx: Scope) -> Self {
        ItemsActions {
            add_item: create_server_multi_action::<AddItem>(cx),
            remove_item: create_server_action::<RemoveItem>(cx),
        }
    }
}

#[component]
pub fn Items(cx: Scope) -> impl IntoView {
    let PaginationStateContext {
        pagination_state,
        set_pagination_state,
    } = use_context(cx).unwrap();

    let add_item = create_server_multi_action::<AddItem>(cx);
    let remove_item = create_server_action::<RemoveItem>(cx);

    log::info!("init Items");

    let paginated_items = create_resource(
        cx,
        move || {
            (
                pagination_state(),
                add_item.version().get(),
                remove_item.version().get(),
            )
        },
        move |(ps, _, _)| async move {
            let res = get_items(cx, ps.page() as u32, ps.page_size() as u32).await;
            match res {
                Ok((items, total_count)) => {
                    set_pagination_state.update(|ps| ps.set_element_count(total_count as usize));
                    items
                }
                Err(msg) => {
                    log::error!("Error reading items: {msg}");
                    vec![]
                }
            }
        },
    );
    view! { cx, <div>
        <MultiActionForm action=add_item>
            <h3>"Add Item"</h3>
            <label>"Title" <input type="text" name="title"/></label>
            <label>"Description" <input type="text" name="description"/></label>
            <input type="submit" value="Add"/>
        </MultiActionForm>
        <Transition fallback=move || view! {cx, <div>"Loading..."</div>}>
            {move || match paginated_items.read() {
                None => None,
                Some(items) => {
                    let items = items.clone();
                    Some(
                        view! { cx, <div>
                            <For
                                each=move || items.clone()
                                key=|item| item.id.clone()
                                view=move |item| {
                                    view!{ cx, <MockItem item=item remove_item=remove_item/> }
                                }/>
                        </div>}.into_any()
                    )
                }
            }}
        </Transition>
    </div>}
}

#[component]
pub fn ItemView(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let id_string = move || params().get("id").cloned();
    let id = move || id_string().and_then(|s| s.parse().ok());

    let remove_item = create_server_action::<RemoveItem>(cx);

    let item_res: Resource<i32, Result<MockItem, ServerFnError>> = create_resource(
        cx,
        move || id().unwrap_or_default(),
        move |id| async move { get_item(id).await },
    );

    view! {cx,
        <A href="/items">"Back to Items"</A>
        <Transition fallback=move || view! {cx, <p>"Loading..."</p> }>
            {move ||
                item_res.read().map(|item| match item {
                    Ok(item) => { (view! {cx, <MockItem item=item.clone() remove_item=remove_item/> }).into_view(cx) },
                    Err(e) => view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_view(cx) ,
                })
            }
        </Transition>
    }
}

#[component]
pub fn MockItem(
    cx: Scope,
    item: MockItem,
    remove_item: Action<RemoveItem, Result<(), ServerFnError>>,
) -> impl IntoView {
    view! { cx,
        <div>
            <h3>{format!("{} [{}]", item.title, item.id)}</h3>
            <p>{item.description}</p>
            <A href={format!("/items/{}", item.id)}>"details"</A>
            <ActionForm action=remove_item>
                <input type="hidden" name="id" value={item.id}/>
                <input type="submit" value="[x]"/>
            </ActionForm>
        </div>
    }
}
