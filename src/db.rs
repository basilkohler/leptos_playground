use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection};

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            let mut conn = SqliteConnection::connect("sqlite:Items.sqlite")
                    .await
                    .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
            Ok(conn)
        }


        pub fn register_server_functions() {
            _ = GetItems::register();
        }

    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockItem {
    pub id: i64,
    pub title: String,
    pub description: String,
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
    let total_count = sqlx::query!("SELECT COUNT(ALL) FROM items")
        .fetch_one(&mut conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok((items, total_count.0))
}

#[derive(Debug)]
pub struct DB {
    pub items: Vec<MockItem>,
}

impl DB {
    pub fn new(size: usize) -> Self {
        let items = (0..size).map(|i| MockItem {
            id: i as i64,
            title: format!("title{i}"),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Mauris a diam maecenas sed enim ut sem viverra aliquet.".to_string(),
        }).collect();
        DB { items }
    }

    pub fn get_paginated_items(&self, n_skip: usize, n_take: usize) -> (Vec<MockItem>, usize) {
        (
            self.items
                .iter()
                .cloned()
                .skip(n_skip)
                .take(n_take)
                .collect(),
            self.items.len(),
        )
    }
}
