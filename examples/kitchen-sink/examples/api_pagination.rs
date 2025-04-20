use chrono::NaiveDate;
use lazybe::axum::Router;
use lazybe::db::sqlite::SqliteDbCtx;
use lazybe::filter::Filter;
use lazybe::macros::Entity;
use lazybe::oas::{CreateRouterDoc, DeleteRouterDoc, GetRouterDoc, ListRouterDoc};
use lazybe::page::{Page, PaginationInput};
use lazybe::router::{CreateRouter, DeleteRouter, EntityCollectionApi, GetRouter, ListRouter, RouteConfig};
use lazybe::sort::Sort;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Sqlite, SqlitePool};
use utoipa::ToSchema;
use utoipa::openapi::{Info, OpenApiBuilder, Server};
use utoipa_redoc::{Redoc, Servable};

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

    let openapi = OpenApiBuilder::new()
        .info(Info::new("Todo Example", "0.1.0"))
        .servers(Some([Server::new("http://localhost:8080")]))
        .build()
        .merge_from(Book::get_endpoint_doc(None))
        .merge_from(Book::list_endpoint_doc(None))
        .merge_from(Book::create_endpoint_doc(None))
        .merge_from(Book::delete_endpoint_doc(None));

    let app = Router::new()
        .merge(Redoc::with_url("/", openapi))
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

#[derive(Debug, Clone, Serialize, Deserialize, Entity, ToSchema)]
#[lazybe(table = "book", endpoint = "/books", collection_api = "manual", derive_to_schema)]
pub struct Book {
    #[lazybe(primary_key)]
    pub id: u32,
    pub title: String,
    pub author: String,
    pub publication_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedBook {
    pub page: u32,
    pub page_size: u64,
    pub count: u64,
    pub data: Vec<Book>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BookQuery {
    page: Option<u32>,
    title: Option<String>,
    author: Option<String>,
}

impl EntityCollectionApi for Book {
    type Resp = PaginatedBook;
    type Query = BookQuery;

    fn page_response(page: Page<Self>) -> Self::Resp {
        Self::Resp {
            page: page.page,
            page_size: page.page_size,
            count: page.total_records,
            data: page.data,
        }
    }

    fn page_input(input: &Self::Query) -> Option<PaginationInput> {
        Some(PaginationInput {
            page: input.page.unwrap_or(0),
            limit: 100,
        })
    }

    fn filter_input(input: &Self::Query) -> Filter<Self> {
        let mut conds = Vec::new();
        if let Some(title) = input.title.as_ref() {
            conds.push(BookFilter::title().like(format!("%{}%", title)));
        }
        if let Some(author) = input.author.as_ref() {
            conds.push(BookFilter::author().like(format!("%{}%", author)))
        }
        Filter::all(conds)
    }

    fn sort_input(_input: &Self::Query) -> Sort<Self> {
        Sort::new([BookSort::publication_date().asc()])
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
