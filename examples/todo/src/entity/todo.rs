use lazybe::db::{DbCtx, DbOps};
use lazybe::entity::ops::CreateEntity;
use lazybe::macros::{Entity, EntityEndpoint, Enum, Newtype};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Sqlite, Transaction};
use utoipa::ToSchema;

use super::staff::StaffId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Newtype, ToSchema)]
pub struct TodoId(u64);

#[derive(Debug, Clone, Serialize, Deserialize, Entity, ToSchema)]
#[lazybe(table = "todo", endpoint = "/todos", derive_to_schema)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Enum, ToSchema)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, EntityEndpoint, ToSchema)]
#[lazybe(endpoint = "/bulk/todos", create_ty = "Vec<CreateTodo>")]
pub struct TodoBulkCreate {
    todos: Vec<Todo>,
}

impl CreateEntity<Sqlite> for TodoBulkCreate {
    async fn create<Ctx>(ctx: &Ctx, tx: &mut Transaction<'_, Sqlite>, input: Self::Create) -> Result<Self, sqlx::Error>
    where
        Ctx: DbCtx<Sqlite> + Sync,
    {
        let mut todos = Vec::with_capacity(input.len());
        for create_todo in input {
            let todo = ctx.create::<Todo>(tx, create_todo).await?;
            todos.push(todo);
        }
        Ok(TodoBulkCreate { todos })
    }
}
