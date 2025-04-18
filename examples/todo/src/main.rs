use entity::todo::Todo;
use lazybe::axum::Router;
use lazybe::axum::extract::State;
use lazybe::axum::http::StatusCode;
use lazybe::axum::routing::get;
use lazybe::db::sqlite::SqliteDbCtx;
use lazybe::oas::{CreateRouterDoc, DeleteRouterDoc, GetRouterDoc, ListRouterDoc, UpdateRouterDoc};
use lazybe::router::{CreateRouter, DeleteRouter, GetRouter, ListRouter, RouteConfig, UpdateRouter};
use sqlx::{Executor, Pool, Sqlite, SqlitePool};
use utoipa::openapi::{Info, OpenApiBuilder, Server};
use utoipa_redoc::{Redoc, Servable};

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
    tracing_subscriber::fmt::init();

    let ctx = SqliteDbCtx;
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    reset_db(&pool).await?;

    let mut app = Router::new();
    let mut openapi = OpenApiBuilder::new()
        .info(Info::new("Todo Example", "0.1.0"))
        .servers(Some([Server::new("http://localhost:8080")]))
        .build();

    let endpoint_defs = [
        (Todo::get_endpoint(), Todo::get_endpoint_doc(None)),
        (Todo::list_endpoint(), Todo::list_endpoint_doc(None)),
        (Todo::create_endpoint(), Todo::create_endpoint_doc(None)),
        (Todo::update_endpoint(), Todo::update_endpoint_doc(None)),
        (Todo::replace_endpoint(), Todo::replace_endpoint_doc(None)),
        (Todo::delete_endpoint(), Todo::delete_endpoint_doc(None)),
    ];
    for (endpoint_router, endpoint_doc) in endpoint_defs {
        app = app.merge(endpoint_router);
        openapi = openapi.merge_from(endpoint_doc);
    }

    let app_router = app
        .merge(Redoc::with_url("/", openapi))
        .route("/_system/reset", get(reset_handler))
        .with_state(AppState { ctx, pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server is listening on 0.0.0.0:8080");
    lazybe::axum::serve(listener, app_router).await?;
    Ok(())
}

async fn reset_handler(State(state): State<AppState>) -> Result<(), StatusCode> {
    reset_db(&state.pool)
        .await
        .inspect_err(|e| tracing::error!("Could not reset the system: {}", e))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn reset_db(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    pool.execute(
        r#"
DROP TABLE IF EXISTS todo;

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
