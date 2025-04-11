use lazybe::Page;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod staff;
pub mod todo;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Paginated<T> {
    pub page: u32,
    pub page_size: u64,
    pub total_records: u64,
    pub total_pages: u64,
    pub data: Vec<T>,
}

impl<T> From<Page<T>> for Paginated<T> {
    fn from(value: Page<T>) -> Self {
        Self {
            page: value.page + 1,
            page_size: value.page_size,
            total_records: value.total_records,
            total_pages: value.total_records.div_ceil(value.page_size),
            data: value.data,
        }
    }
}
