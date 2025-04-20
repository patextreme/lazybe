use chrono::NaiveDate;
use lazybe::axum::Router;
use lazybe::db::sqlite::SqliteDbCtx;
use lazybe::macros::Entity;
use lazybe::router::{CreateRouter, DeleteRouter, ErrorResponse, GetRouter, ListRouter, RouteConfig, ValidationHook};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Sqlite, SqlitePool};

#[derive(Clone)]
struct AppState {
    ctx: SqliteDbCtx,
    pool: SqlitePool,
}

impl RouteConfig for AppState {
    type Ctx = SqliteDbCtx;
    type Db = Sqlite;

    fn db_ctx(&self) -> (Self::Ctx, SqlitePool) {
        (self.ctx.clone(), self.pool.clone())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let ctx = SqliteDbCtx;
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    migrate(&pool).await?;

    let app = Router::new()
        .merge(Book::get_endpoint())
        .merge(Book::list_endpoint())
        .merge(Book::create_endpoint())
        .merge(Book::delete_endpoint())
        .with_state(AppState { ctx, pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server is listening on 0.0.0.0:8080");
    lazybe::axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[lazybe(table = "book", endpoint = "/books", validation = "manual")]
pub struct Book {
    #[lazybe(primary_key)]
    pub id: u32,
    pub title: String,
    pub author: String,
    pub publication_date: NaiveDate,
}

impl ValidationHook for Book {
    fn before_create(input: &Self::Create) -> Result<(), ErrorResponse> {
        if input.title.len() > 100 {
            Err(ErrorResponse {
                title: "Invalid book create payload".to_string(),
                detail: Some("Book title cannot be longer than 100 characters".to_string()),
                instance: None,
            })?
        }
        Ok(())
    }
}

async fn migrate(pool: &SqlitePool) -> anyhow::Result<()> {
    pool.execute(
        r#"
CREATE TABLE IF NOT EXISTS book (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    publication_date DATE NOT NULL
);
        "#,
    )
    .await?;
    Ok(())
}
