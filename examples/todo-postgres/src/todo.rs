use lazybe::macros::{Entity, Enum, Newtype};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Newtype, ToSchema)]
pub struct TodoId(i64);

#[derive(Debug, Clone, Serialize, Deserialize, Entity, ToSchema)]
#[lazybe(table = "todo", endpoint = "/todos", derive_to_schema)]
pub struct Todo {
    #[lazybe(primary_key)]
    pub id: TodoId,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
    #[lazybe(updated_at)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Enum, ToSchema)]
pub enum Status {
    Todo,
    Doing,
    Done,
}
