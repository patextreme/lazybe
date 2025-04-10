use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use entity::staff::Staff;
use entity::todo::Todo;
use lazybe::axum::{CreateRouter, DeleteRouter, GetRouter, ListRouter, RouteConfig, UpdateRouter};
use lazybe::sqlite::SqliteDbCtx;
use sqlx::{Executor, Pool, Sqlite, SqlitePool};

mod entity;

#[derive(Clone)]
struct AppState {
    ctx: SqliteDbCtx,
    pool: SqlitePool,
}

impl RouteConfig for AppState {
    type Ctx = SqliteDbCtx;
    type Db = Sqlite;

    fn db_ctx(&self) -> (Self::Ctx, Pool<Self::Db>) {
        (self.ctx.clone(), self.pool.clone())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = SqliteDbCtx;
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    reset_db(&pool).await?;

    let state = AppState { ctx, pool };
    let app = axum::Router::new()
        .route("/_system/reset", get(reset_handler))
        .merge(Todo::get_endpoint())
        .merge(Todo::list_endpoint())
        .merge(Todo::create_endpoint())
        .merge(Todo::replace_endpoint())
        .merge(Todo::update_endpoint())
        .merge(Todo::delete_endpoint())
        .merge(Staff::get_endpoint())
        .merge(Staff::list_endpoint())
        .merge(Staff::create_endpoint())
        .merge(Staff::replace_endpoint())
        .merge(Staff::update_endpoint())
        .merge(Staff::delete_endpoint())
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}

async fn reset_handler(State(state): State<AppState>) -> Result<(), StatusCode> {
    reset_db(&state.pool)
        .await
        .inspect_err(|e| println!("Could not reset the system: {}", e))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn reset_db(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    pool.execute(
        r#"
DROP TABLE IF EXISTS todo;
DROP TABLE IF EXISTS staff;

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
    assignee INTEGER NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (assignee) REFERENCES staff(id) ON DELETE RESTRICT
);
        "#,
    )
    .await?;
    Ok(())
}
