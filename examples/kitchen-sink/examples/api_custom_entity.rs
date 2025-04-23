use chrono::NaiveDate;
use lazybe::axum::Router;
use lazybe::db::sqlite::SqliteDbCtx;
use lazybe::db::{DbCtx, DbOps};
use lazybe::entity::ops::CreateEntity;
use lazybe::macros::{Entity, EntityEndpoint};
use lazybe::openapi::{CreateRouterDoc, DeleteRouterDoc, GetRouterDoc, ListRouterDoc};
use lazybe::router::{CreateRouter, DeleteRouter, GetRouter, ListRouter, RouteConfig};
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
        .merge_from(BookBatch::create_endpoint_doc(None))
        .merge_from(Book::get_endpoint_doc(None))
        .merge_from(Book::list_endpoint_doc(None))
        .merge_from(Book::create_endpoint_doc(None))
        .merge_from(Book::delete_endpoint_doc(None));

    let app = Router::new()
        .merge(Redoc::with_url("/", openapi))
        .merge(BookBatch::create_endpoint())
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
#[lazybe(table = "book", endpoint = "/books", derive_to_schema)]
pub struct Book {
    #[lazybe(primary_key)]
    pub id: u32,
    pub title: String,
    pub author: String,
    pub publication_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, EntityEndpoint, ToSchema)]
#[lazybe(endpoint = "/book-batches", create_ty = "Vec<CreateBook>")]
pub struct BookBatch {
    books: Vec<Book>,
}

impl CreateEntity<Sqlite> for BookBatch {
    async fn create<Ctx>(
        ctx: &Ctx,
        tx: &mut sqlx::Transaction<'_, Sqlite>,
        input: Self::Create,
    ) -> Result<Self, sqlx::Error>
    where
        Ctx: DbCtx<Sqlite> + Sync,
    {
        let mut books = Vec::with_capacity(input.len());
        for create_book in input {
            let book = ctx.create::<Book>(tx, create_book).await?;
            books.push(book);
        }
        Ok(BookBatch { books })
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
