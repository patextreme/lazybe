use lazybe::axum::GetRoutable;
use lazybe::axum::sqlite::ToSqliteAxumState;
use lazybe::{DbCtx, SqliteDbCtx};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Executor, Pool, Sqlite, SqlitePool};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::DalNewtype)]
pub struct StaffId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::DalEntity)]
#[lazybe(table = "staff")]
pub struct Staff {
    #[lazybe(primary_key)]
    pub id: StaffId,
    pub name: String,
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
    #[lazybe(updated_at)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::DalNewtype)]
pub struct TodoId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::DalEntity)]
#[lazybe(table = "todo")]
pub struct Todo {
    #[lazybe(primary_key)]
    pub id: TodoId,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub assignee: Option<StaffId>,
    #[lazybe(created_at)]
    pub created_at: DateTime<Utc>,
    #[lazybe(updated_at)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, lazybe::DalEnum)]
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

impl ToSqliteAxumState for AppState {
    fn to_sqlite_state(&self) -> (lazybe::SqliteDbCtx, sqlx::Pool<sqlx::Sqlite>) {
        (self.ctx.clone(), self.pool.clone())
    }
}

impl GetRoutable for Todo {
    fn get_route() -> &'static str {
        "/todos/{id}"
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx: SqliteDbCtx = DbCtx::sqlite();
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    run_migration(&pool).await?;

    let state = AppState { ctx, pool };

    let app = axum::Router::new().with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

async fn run_migration(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    pool.execute(
        r#"
CREATE TABLE IF NOT EXISTS staff (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    assignee INTEGER REFERENCES staff(id) ON DELETE RESTRICT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);
        "#,
    )
    .await?;
    Ok(())
}
