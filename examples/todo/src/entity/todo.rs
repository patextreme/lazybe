use lazybe::PaginationInput;
use lazybe::axum::EntityCollectionApi;
use lazybe::filter::Filter;
use lazybe::sort::Sort;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};

use super::staff::StaffId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::Newtype)]
pub struct TodoId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::Entity)]
#[lazybe(table = "todo", endpoint = "/todos")]
pub struct Todo {
    #[lazybe(primary_key)]
    pub id: TodoId,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub assignee: StaffId,
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
    #[lazybe(updated_at)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::Enum)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoCollectionQuery {
    pub status: Option<Status>,
    pub assignee: Option<StaffId>,
    pub page: Option<u32>,
    pub size: Option<u32>,
}

impl EntityCollectionApi for Todo {
    type Resp = lazybe::Page<Todo>;
    type Query = TodoCollectionQuery;

    fn page_response(page: lazybe::Page<Self>) -> Self::Resp {
        page
    }

    fn page_input(input: &Self::Query) -> Option<PaginationInput> {
        Some(PaginationInput {
            page: input.page.unwrap_or(0),
            limit: input.size.unwrap_or(10).min(100),
        })
    }

    fn filter_input(input: &Self::Query) -> Filter<Self> {
        let mut conditions = Vec::new();
        if let Some(status) = input.status.as_ref().cloned() {
            conditions.push(TodoFilter::status().eq(status));
        }
        if let Some(assignee) = input.assignee.as_ref().cloned() {
            conditions.push(TodoFilter::assignee().eq(assignee));
        }
        Filter::all(conditions)
    }

    fn sort_input(_input: &Self::Query) -> Sort<Self> {
        Sort::new([TodoSort::id().asc()])
    }
}
