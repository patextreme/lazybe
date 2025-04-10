use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInput {
    /// 0-index page number (0 is the first page)
    pub page: u32,
    pub limit: u32,
}

impl Default for PaginationInput {
    fn default() -> Self {
        Self { page: 0, limit: 100 }
    }
}

impl PaginationInput {
    pub fn offset(&self) -> u32 {
        self.page * self.limit
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page<T> {
    pub page: u32,
    pub page_size: u64,
    pub total_records: u64,
    pub data: Vec<T>,
}
