use log::info;

#[derive(Debug)]
pub struct DB {
    pub items: Vec<MockItem>,
}

impl DB {
    pub fn new(size: usize) -> Self {
        info!("construct db");
        let items = (0..size).map(|i| MockItem {
            id: format!("id{i}"),
            title: format!("title{i}"),
            description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Mauris a diam maecenas sed enim ut sem viverra aliquet.".to_string(),
        }).collect();
        DB { items }
    }

    pub fn get_paginated_items(&self, n_skip: usize, n_take: usize) -> PaginatedResult<MockItem> {
        info!("get paginated items");
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
    pub id: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub result: Vec<T>,
    pub total: usize,
}
