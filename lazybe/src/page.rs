#[derive(Debug, Clone)]
pub struct Pagination {
    /// 0-index page number (0 is the first page)
    pub page: u64,
    pub limit: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 0, limit: 100 }
    }
}

impl Pagination {
    pub fn offset(&self) -> u64 {
        self.page * self.limit
    }
}

#[derive(Debug)]
pub struct Page<T> {
    pub total: u64,
    pub data: Vec<T>,
}
