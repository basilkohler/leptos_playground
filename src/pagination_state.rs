#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PaginationState {
    page: usize,
    page_size: usize,
    element_count: usize,
    n_left_right: usize,
}

impl PaginationState {
    pub fn calc_skip(&self) -> usize {
        self.page.saturating_sub(1) * self.page_size
    }
    pub fn page_size(&self) -> usize {
        self.page_size
    }
    pub fn has_first(&self) -> bool {
        self.page > self.n_left_right + 1
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
        self.page <= self.pages().saturating_sub(self.n_left_right + 1)
    }
    pub fn last(&self) -> usize {
        self.pages()
    }
    pub fn is_cur(&self, page: usize) -> bool {
        self.page == page
    }
    pub fn has_prev(&self) -> bool {
        self.page <= 1
    }
    pub fn go_prev(&mut self) {
        self.page -= 1;
    }
    pub fn next(&self) -> usize {
        self.page + 1
    }
    pub fn has_next(&self) -> bool {
        self.page < self.pages()
    }
    pub fn go_next(&mut self) {
        self.page += 1;
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
    pub fn set_page_size(&mut self, page_size: usize) {
        self.page_size = page_size;
        self.page = 1;
    }
    pub fn set_element_count(&mut self, element_count: usize) {
        self.element_count = element_count;
    }
    pub fn element_count(&self) -> usize {
        self.element_count
    }
    pub fn generate_pagination(&self) -> Vec<Option<usize>> {
        let mut pagination = Vec::new();
        if self.has_first() {
            pagination.push(Some(self.first()));
        }
        if self.has_dots_left() {
            pagination.push(None);
        }
        for p in self.from_to() {
            pagination.push(Some(p));
        }
        if self.has_dots_right() {
            pagination.push(None);
        }
        if self.has_last() {
            pagination.push(Some(self.last()));
        }
        pagination
    }
}

impl Default for PaginationState {
    fn default() -> Self {
        PaginationState {
            page: 1,
            page_size: 1,
            element_count: 0,
            n_left_right: 1,
        }
    }
}
