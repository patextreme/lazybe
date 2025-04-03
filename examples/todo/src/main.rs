use lazybe::axum::{CreateRouter, GetRouter, ToDbState};
use lazybe::sqlite::SqliteDbCtx;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Executor, Pool, Sqlite, SqlitePool};

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
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
    #[lazybe(updated_at)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::Enum)]
pub enum Status {
    Backlog,
    Todo,
    Doing,
    Done,
}

#[derive(Clone)]
struct AppState {
    ctx: SqliteDbCtx,
    pool: SqlitePool,
}

impl ToDbState for AppState {
    type Ctx = SqliteDbCtx;
    type Db = Sqlite;

    fn to_db_state(&self) -> (Self::Ctx, Pool<Self::Db>) {
        (self.ctx.clone(), self.pool.clone())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = SqliteDbCtx;
    // let pool = SqlitePool::connect("sqlite::memory:").await?;
    let pool = SqlitePool::connect("sqlite://test.db").await?;
    run_migration(&pool).await?;

    let state = AppState { ctx, pool };
    let app = axum::Router::new()
        .merge(Todo::get_router())
        .merge(Todo::create_router())
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_migration(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    pool.execute(
        r#"
CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);
        "#,
    )
    .await?;
    Ok(())
}
