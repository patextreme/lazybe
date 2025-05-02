use lazybe::axum::Router;
use lazybe::axum::extract::{Path, Query};
use lazybe::axum::response::Redirect;
use lazybe::axum::routing::get;
use lazybe::macros::typed_uri;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct QueryParam {
    repetition: usize,
}

typed_uri!(Hello, "hello");
typed_uri!(Echo, "echo" / (word: String) ? QueryParam);
typed_uri!(Echo2, "echo2" / (word: String) / (repetition: usize));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route(Hello::AXUM_PATH, get(hello))
        .route(Echo::AXUM_PATH, get(echo))
        .route(Echo2::AXUM_PATH, get(echo2));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server is listening on 0.0.0.0:8080");
    lazybe::axum::serve(listener, app).await?;
    Ok(())
}

async fn hello() -> &'static str {
    "hello world"
}

async fn echo(Path(path): Path<Echo>, Query(q): Query<QueryParam>) -> String {
    std::iter::repeat_n(path.word.as_str(), q.repetition)
        .collect::<Vec<_>>()
        .join("\n")
}

async fn echo2(Path(path): Path<Echo2>) -> Redirect {
    let uri = Echo::new_uri(
        path.word,
        QueryParam {
            repetition: path.repetition,
        },
    );
    Redirect::temporary(&uri)
}
