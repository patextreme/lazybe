use entity::staff::Staff;
use entity::todo::Todo;
use lazybe::axum::Router;
use lazybe::axum::extract::State;
use lazybe::axum::http::StatusCode;
use lazybe::axum::routing::get;
use lazybe::oas::{CreateRouterDoc, DeleteRouterDoc, GetRouterDoc, ListRouterDoc, UpdateRouterDoc};
use lazybe::router::{CreateRouter, DeleteRouter, GetRouter, ListRouter, RouteConfig, UpdateRouter};
use lazybe::sqlite::SqliteDbCtx;
use sqlx::{Executor, Pool, Sqlite, SqlitePool};
use utoipa::openapi::{Info, OpenApiBuilder, Server};
use utoipa_swagger_ui::SwaggerUi;

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

    let endpoint_defs = [
        (Todo::get_endpoint(), Todo::get_endpoint_doc()),
        (Todo::list_endpoint(), Todo::list_endpoint_doc()),
        (Todo::create_endpoint(), Todo::create_endpoint_doc()),
        (Todo::update_endpoint(), Todo::update_endpoint_doc()),
        (Todo::replace_endpoint(), Todo::replace_endpoint_doc()),
        (Todo::delete_endpoint(), Todo::delete_endpoint_doc()),
        (Staff::get_endpoint(), Staff::get_endpoint_doc()),
        (Staff::list_endpoint(), Staff::list_endpoint_doc()),
        (Staff::create_endpoint(), Staff::create_endpoint_doc()),
        (Staff::update_endpoint(), Staff::update_endpoint_doc()),
        (Staff::replace_endpoint(), Staff::replace_endpoint_doc()),
        (Staff::delete_endpoint(), Staff::delete_endpoint_doc()),
    ];

    let app_state = AppState { ctx, pool };
    let mut app = Router::new();

    let mut oas = OpenApiBuilder::new()
        .info(Info::new("Todo Example", "0.1.0"))
        .servers(Some([Server::new("http://localhost:8080")]))
        .build();

    for (endpoint_router, endpoint_doc) in endpoint_defs {
        app = app.merge(endpoint_router);
        oas = oas.merge_from(endpoint_doc);
    }

    let app = app
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", oas))
        .route("/_system/reset", get(reset_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server is listening on 0.0.0.0:8080");
    lazybe::axum::serve(listener, app.with_state(app_state)).await?;
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
