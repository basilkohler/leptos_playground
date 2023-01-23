use crate::pagination::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PaginationState {
    pub page: usize,
    pub page_size: usize,
    element_count: usize,
    n_left_right: usize,
}

impl PaginationState {
    pub fn new(page: usize, page_size: usize) -> Self {
        PaginationState {
            page,
            page_size,
            ..PaginationState::default()
        }
    }
    pub fn new_with_count(page: usize, page_size: usize, element_count: usize) -> Self {
        PaginationState {
            page,
            page_size,
            element_count,
            ..PaginationState::default()
        }
    }
    pub fn calc_skip(&self) -> usize {
        self.page.saturating_sub(1) * self.page_size
    }
    pub fn page_size(&self) -> usize {
        self.page_size
    }
    pub fn has_first(&self) -> bool {
        self.page > 1
    }
    pub fn first(&self) -> usize {
        1
    }
    pub fn has_dots_left(&self) -> bool {
        self.from() > 2
    }
    pub fn has_dots_right(&self) -> bool {
        self.to() < self.pages().saturating_sub(1)
    }
    pub fn prev(&self) -> usize {
        self.page.saturating_sub(1)
    }
    pub fn page(&self) -> usize {
        self.page
    }
    pub fn from(&self) -> usize {
        self.page.saturating_sub(self.n_left_right).max(1)
    }
    pub fn to(&self) -> usize {
        (self.page + self.n_left_right).min(self.pages())
    }
    pub fn from_to(&self) -> Vec<usize> {
        (self.from()..=self.to()).collect()
    }
    pub fn has_last(&self) -> bool {
        self.page < self.pages()
    }
    pub fn last(&self) -> usize {
        self.pages()
    }
    pub fn is_cur(&self, page: usize) -> bool {
        self.page == page
    }
    pub fn has_prev(&self) -> bool {
        self.page > 1
    }
    pub fn next(&self) -> usize {
        (self.page + 1).min(self.element_count)
    }
    pub fn has_next(&self) -> bool {
        self.page < self.pages()
    }
    pub fn pages(&self) -> usize {
        self.element_count
            .checked_div(self.page_size)
            .map(|p| {
                p + if self.element_count % self.page_size == 0 {
                    0
                } else {
                    1
                }
            })
            .unwrap_or_default()
    }
    pub fn set_page(&mut self, page: usize) {
        self.page = page;
    }
    pub fn update(&mut self, page: usize, page_size: usize, n: usize) {
        self.page = page;
        self.page_size = page_size;
        self.element_count = n;
    }
    pub fn set_page_size(&mut self, page_size: usize) {
        self.page_size = page_size;
        self.page = 1;
    }
    pub fn set_page_and_size(&mut self, page: usize, page_size: usize) {
        self.page = page;
        self.page_size = page_size;
    }
    pub fn set_element_count(&mut self, element_count: usize) {
        self.element_count = element_count;
    }
    pub fn element_count(&self) -> usize {
        self.element_count
    }
    pub fn generate_pagination(&self) -> Vec<PaginationItem> {
        use PaginationItem::*;
        log::info!("gen pag: {:?}", self);

        fn value_bool_to_option<T>(v: T, b: bool) -> Option<T> {
            if b {
                Some(v)
            } else {
                None
            }
        }

        let mut pagination = Vec::new();
        pagination.push(First(value_bool_to_option(self.first(), self.has_first())));
        pagination.push(Prev(value_bool_to_option(self.prev(), self.has_prev())));
        if self.has_dots_left() {
            pagination.push(DotsLeft);
        }
        let pages = self
            .from_to()
            .into_iter()
            .map(|p| (p, self.is_cur(p)))
            .collect();
        pagination.push(Pages(pages));
        if self.has_dots_right() {
            pagination.push(DotsRight);
        }
        pagination.push(Next(value_bool_to_option(self.next(), self.has_next())));
        pagination.push(Last(value_bool_to_option(self.last(), self.has_last())));
        pagination
    }
}

impl Default for PaginationState {
    fn default() -> Self {
        PaginationState {
            page: DEFAULT_PAGE,
            page_size: DEFAULT_PAGE_SIZE,
            element_count: 0,
            n_left_right: 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaginationItem {
    First(Option<usize>),
    Prev(Option<usize>),
    DotsLeft,
    Pages(Vec<(usize, bool)>),
    DotsRight,
    Next(Option<usize>),
    Last(Option<usize>),
}
